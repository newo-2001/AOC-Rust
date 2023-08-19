use std::{fs, error::Error, cmp::{max, min}, collections::HashMap, iter};

use aoc_lib::parsing::optional_newline;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    sequence::{preceded, tuple, terminated},
    Parser,
    error::VerboseError
};

#[derive(Clone)]
struct Entity {
    health: u32,
    damage: u32,
    armor: u32
}

impl Entity {
    fn alive(&self) -> bool { self.health > 0 }
    
    fn attack(&self, target: &mut Self) {
        let damage = max(1, self.damage as i32 - target.armor as i32) as u32;
        target.health -= min(damage, target.health);
    }

    fn parse(input: &str) -> Result<Entity, String> {
        let kv = |key| terminated(preceded(tag(key).and(tag(": ")), complete::u32), optional_newline);
        let mut entity = tuple((kv("Hit Points"), kv("Damage"), kv("Armor")))
            .map(|(health, damage, armor)| Entity { health, damage, armor });

        Ok(entity.parse(input).map_err(|err: nom::Err<VerboseError<&str>>| err.to_string())?.1)
    }

    fn with_gear(&self, gear: &Vec<&Item>) -> Entity {
        let gear_armor: u32 = gear.iter().map(|item| item.armor).sum();
        let gear_damage: u32 = gear.iter().map(|item| item.damage).sum();

        Entity {
            health: self.health,
            armor: self.armor + gear_armor,
            damage: self.damage + gear_damage
        }
    }

    fn fight(mut self: Entity, mut other: Entity) -> BattleResult {
        loop {
            self.attack(&mut other);
            other.attack(&mut self);

            match (self.alive(), other.alive()) {
                (_, false) => return BattleResult::Victory,
                (false, true) => return BattleResult::Defeat,
                (true, true) => { }
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum ItemSlot {
    Weapon, Armor, Ring
}

struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

type Shop = HashMap<ItemSlot, Vec<Item>>;
type Gear<'a> = Vec<&'a Item>;

fn make_shop() -> Shop {
    fn make_weapon(cost: u32, damage: u32) -> Item {
        Item { cost, damage, armor: 0 }
    }

    fn make_armor(cost: u32, armor: u32) -> Item {
        Item { cost, armor, damage: 0 }
    }

    fn make_ring(cost: u32, damage: u32, armor: u32) -> Item {
        Item { cost, damage, armor }
    }

    let weapons: Vec<Item> = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)]
        .into_iter()
        .map(|(cost, damage)| make_weapon(cost, damage))
        .collect();

    let armor: Vec<Item> = [(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)]
        .into_iter()
        .map(|(cost, armor)| make_armor(cost, armor))
        .collect();

    let rings: Vec<Item> = [
        (0, 0, 0), (25, 1, 0), (50, 2, 0), (100, 3, 0),
        (20, 0, 1), (40, 0, 2), (80, 0, 3)
    ].into_iter()
        .map(|(cost, damage, armor)| make_ring(cost, damage, armor))
        .collect();

    HashMap::from_iter([
        (ItemSlot::Weapon, weapons),
        (ItemSlot::Armor, armor),
        (ItemSlot::Ring, rings)
    ])
}

#[derive(Eq, PartialEq)]
enum BattleResult {
    Victory,
    Defeat
}

impl BattleResult {
    fn won(&self) -> bool { self == &BattleResult::Victory }
}

fn gear_cost(gear: &Gear) -> u32 {
    gear.iter().map(|item| item.cost).sum()
}

fn all_loadouts(shop: &Shop) -> Vec<Gear> {
    let no_ring = shop[&ItemSlot::Ring].iter()
        .find(|ring| ring.cost == 0).unwrap();

    let rings = shop[&ItemSlot::Ring].iter()
        .combinations(2).chain(iter::once(vec![no_ring, no_ring]))
        .collect_vec();

    let armor_weapons = shop[&ItemSlot::Armor].iter()
        .flat_map(|armor| shop[&ItemSlot::Weapon].iter()
            .map(move |weapon| vec![armor, weapon]))
            .collect_vec();

    rings.iter().flat_map(|rings| armor_weapons.iter().map(|gear| {
        rings.clone().into_iter()
            .chain(gear.clone().into_iter()).collect()
    })).collect()
}

const PLAYER: Entity = Entity {
    health: 100,
    armor: 0,
    damage: 0
};

fn cheapest_victory(enemy: &Entity, loadouts: &Vec<Gear>) -> u32 {
    loadouts.iter()
        .map(|gear| (PLAYER.with_gear(gear), gear_cost(gear)))
        .filter_map(|(player, cost)| player.fight(enemy.clone()).won().then_some(cost))
        .min().expect("Player can't defeat the enemy")
}

fn most_expensive_loss(enemy: &Entity, loadouts: &Vec<Gear>) -> u32 {
    loadouts.iter()
        .map(|gear| (PLAYER.with_gear(gear), gear_cost(gear)))
        .filter_map(|(player, cost)| (!player.fight(enemy.clone()).won()).then_some(cost))
        .max().expect("Player can't lose to the enemy")
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_21.txt")?;

    let shop = make_shop();
    let loadouts = all_loadouts(&shop);

    let boss = Entity::parse(&content)?;

    let cost = cheapest_victory(&boss, &loadouts);
    println!("The least amount of gold to expend on equipment to win is {}", cost);

    let cost = most_expensive_loss(&boss, &loadouts);
    println!("The most amount of gold to expend on equipment and still lose is {}", cost);

    Ok(())
}