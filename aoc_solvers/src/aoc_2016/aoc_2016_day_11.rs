use std::{rc::Rc, collections::{BTreeSet, VecDeque}, hash::Hash, iter::once};
use aoc_lib::{parsing::{ParseError, parse_lines, Runnable, skip_over}, iteration::queue::{Dedupable, FindState}, NoSolutionError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{bytes::complete::{tag, take_till}, sequence::{terminated, preceded}, Parser, multi::separated_list0, combinator::{all_consuming, opt}, character::complete};

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
struct Material<'a>(Rc<&'a str>);

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
enum Item<'a> {
    Chip(Material<'a>),
    Generator(Material<'a>)
}

impl<'a> Item<'a> {
    fn is_safe_with(&'a self, other: &'a Item) -> bool {
        match (self, other) {
            | (Self::Chip(chip), Self::Generator(generator))
            | (Self::Generator(generator), Self::Chip(chip)) => {
                chip == generator
            },
            _ => true
        }
    }

    fn material(&self) -> &Material {
        match self {
            Self::Chip(material) => material,
            Self::Generator(material) => material
        }
    }

    fn is_generator(&self) -> bool {
        match self {
            Self::Chip(..) => false,
            Self::Generator(..) => true
        }
    }
}

#[derive(Clone)]
enum Inventory<'a, 'b> {
    Single(&'b Item<'a>),
    Double((&'b Item<'a>, &'b Item<'a>))
}

enum ElevatorDirection {
    Up,
    Down
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Floor<'a>(BTreeSet<Item<'a>>);

impl<'a> Floor<'a> {
    fn iter_items<'b>(&'b self) -> std::collections::btree_set::Iter<'b, Item<'a>> { self.0.iter() }
    fn is_empty<'b>(&'b self) -> bool { self.0.is_empty() }
    
    fn counts<'b>(&'b self) -> (usize, usize) {
        let counts = self.iter_items()
            .counts_by(|item| Item::is_generator(item));
        (*counts.get(&false).unwrap_or(&0), *counts.get(&true).unwrap_or(&0))
    }
    
    fn take_item<'b>(&'b mut self, item: &'b Item<'a>) -> Option<Item<'a>> {
        self.0.take(item)
    }

    fn place_item<'b>(&'b mut self, item: Item<'a>) {
        if !self.0.insert(item) {
            panic!("Item was duplicated")
        }
    }

    fn is_safe_with<'b>(&'b self, item: &'a Item) -> bool {
        match item {
            Item::Generator(_) => {
                self.iter_items().all(|other| {
                    other.is_safe_with(item) ||
                    self.0.contains(&Item::Generator(other.material().clone()))
                })
            },
            Item::Chip(material) => {
                self.0.contains(&Item::Generator(material.clone())) ||
                self.iter_items().all(|other| item.is_safe_with(other))
            }
        }
    }

    fn is_safe_without<'b>(&'b self, item: &'a Item) -> bool {
        match item {
            Item::Chip(_) => true,
            Item::Generator(material) => {
                let chip = Item::Chip(material.clone());
                !self.0.contains(&chip) ||
                self.iter_items().all(|other| chip.is_safe_with(other))
            }
        }
    }
}

fn parse_floor(input: &str) -> Result<Floor, ParseError> {
    if input.contains("nothing relevant") { return Ok(Floor(BTreeSet::new()))}

    let material = || take_till(|c| " -".contains(c))
        .map(|name: &str| Material(Rc::new(name)));

    let generator = terminated(material(), tag(" generator")).map(Item::Generator);
    let microchip = terminated(material(), tag("-compatible microchip")).map(Item::Chip);
    let item = preceded(tag("a "), generator.or(microchip));
    
    let sep = preceded(opt(complete::char(',')), tag(" and ")).or(tag(", "));

    let floor = all_consuming(terminated(separated_list0(sep, item), complete::char('.')))
        .map(|items| Floor(BTreeSet::from_iter(items.into_iter())));

    preceded(skip_over("contains "), floor).run(input)
}

#[derive(Debug, Clone)]
struct Configuration<'a> {
    floors: Vec<Floor<'a>>,
    current_floor: usize,
    depth: usize
}

impl Hash for Configuration<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current_floor.hash(state);
        self.state().hash(state)
    }
}

impl Eq for Configuration<'_> {}
impl PartialEq for Configuration<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.current_floor == other.current_floor &&
        self.state() == other.state()
    }
}

impl Configuration<'_> {
    fn is_complete(&self) -> bool {
        self.current_floor == 3 &&
        self.floors[0..3].iter()
            .all(|floor| floor.is_empty())
    }

    fn state(&self) -> Vec<(usize, usize)> {
        self.floors.iter()
            .map(Floor::counts)
            .collect_vec()
    }
}

impl<'a> Configuration<'a> {
    fn with<'b>(&'b self, inventory: Inventory<'a, 'b>, direction: ElevatorDirection) -> Option<Configuration<'a>> {
        let target_floor_number = match (direction, self.current_floor) {
            (ElevatorDirection::Down, floor) if floor > 0 => floor - 1,
            (ElevatorDirection::Up, floor) if floor < 3 => floor + 1,
            _ => return None
        };

        let mut floors = self.floors.clone();
        let [floor, target_floor] = floors.get_many_mut([self.current_floor, target_floor_number]).ok()?;
        
        let mut move_item = |item| Some(target_floor.place_item(floor.take_item(item)?));
        match inventory {
            Inventory::Single(item) => move_item(item),
            Inventory::Double((first, second)) => {
                move_item(first)?;
                move_item(second)
            }
        }?;

        let is_safe = |item| floor.is_safe_without(item) && target_floor.is_safe_with(item);
        match inventory {
            Inventory::Single(item) => is_safe(item),
            Inventory::Double((first, second)) => is_safe(first) && is_safe(second)
        }.then_some(())?;

        Some(Configuration {
            depth: self.depth + 1,
            current_floor: target_floor_number,
            floors
        })
    }
}

fn initial_configuration(input: &str) -> Result<Configuration, ParseError> {
    Ok(Configuration {
        floors: parse_lines(parse_floor, input)?,
        current_floor: 0,
        depth: 0
    })
}

fn min_moves_to_top(initial: Configuration) -> Result<usize, NoSolutionError> {
    VecDeque::from_iter(once(initial))
        .filter_duplicates()
        .recursive_find(|state| {
            if state.is_complete() { return FindState::Result(state.depth) }

            let current_floor = &state.floors[state.current_floor];
            let inventories = current_floor.iter_items()
                .tuple_combinations()
                .map(Inventory::Double)
                .chain(current_floor.iter_items().map(Inventory::Single));

            let branches = inventories.flat_map(|inventory| [
                state.with(inventory.clone(), ElevatorDirection::Down),
                state.with(inventory, ElevatorDirection::Up)
            ]).flatten().collect_vec();

            FindState::Branch(branches)
        }).ok_or(NoSolutionError)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let initial_configuration = initial_configuration(input)?;
    let min_moves = min_moves_to_top(initial_configuration)?;
    Ok(Box::new(min_moves))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut initial_configuration = initial_configuration(input)?;
    let ground_floor = &mut initial_configuration.floors[0];

    let elerium = Material(Rc::new("elerium"));
    let dilithium = Material(Rc::new("dilithium"));
    
    for item in [
        Item::Generator(elerium.clone()),
        Item::Chip(elerium),
        Item::Generator(dilithium.clone()),
        Item::Chip(dilithium)
    ] { ground_floor.place_item(item); }

    let min_moves = min_moves_to_top(initial_configuration)?;
    Ok(Box::new(min_moves))
}