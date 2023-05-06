use std::{cmp::{max, min}, collections::{BinaryHeap, HashSet}, iter, error::Error, fs, hash::Hash};

use aoc_lib::parsing::optional_newline;
use nom:: {
    sequence::delimited,
    Parser, error::VerboseError,
    bytes::complete::tag,
    character::complete
};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Effect {
    Shielded,
    Poisoned,
    Recharging
}

#[derive(Clone, Eq)]
struct StatusEffect {
    effect: Effect,
    duration: u32
}

impl Hash for StatusEffect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.effect.hash(state);
    }
}

impl PartialEq for StatusEffect {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect
    }
}

impl StatusEffect {
    fn apply(mut self, battle: &mut Battle) -> Option<StatusEffect> {
        self.duration -= 1;
        
        match self {
            StatusEffect { effect: Effect::Poisoned, duration: _ } => {
                battle.enemy.hurt(3);
            },
            StatusEffect { effect: Effect::Recharging, duration: _ } => {
                battle.player.mana += 101;
            }
            StatusEffect { effect: Effect::Shielded, duration: 0 } => {
                battle.player.armor -= 7;
            }
            _ => {}
        }

        (!self.ended()).then_some(self)
    }
    
    fn ended(&self) -> bool { self.duration == 0 }
}

#[derive(Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u32 {
        match self {
            &Self::MagicMissile => 53,
            &Self::Drain        => 73,
            &Self::Shield       => 113,
            &Self::Poison       => 173,
            &Self::Recharge     => 229
        }
    }

    fn effect(&self) -> Option<StatusEffect> {
        match self {
            &Self::Poison   => Some(StatusEffect { effect: Effect::Poisoned,   duration: 6 }),
            &Self::Shield   => Some(StatusEffect { effect: Effect::Shielded,   duration: 6 }),
            &Self::Recharge => Some(StatusEffect { effect: Effect::Recharging, duration: 5 }),
            _ => None
        }
    }
}

trait Alive {
    fn health(&self) -> u32;
    fn health_mut(&mut self) -> &mut u32;
    fn armor(&self) -> u32;

    fn hurt(&mut self, damage: u32) {
        let true_damage = max(1, damage as i32 - self.armor() as i32) as u32;

        let health = self.health_mut();
        *health -= min(*health, true_damage);
    }

    fn heal(&mut self, health: u32) {
        *self.health_mut() += health;
    }

    fn alive(&self) -> bool { self.health() > 0 }
}

impl Spell {
    fn castable(&self, battle: &Battle) -> bool {
        if battle.player.mana < self.cost() { return false; }
        match self.effect() {
            Some(effect) => !battle.effects.contains(&effect),
            None => true
        }
    }

    fn cast(&self, battle: &mut Battle) {
        battle.player.mana -= self.cost();
        battle.mana_expended += self.cost();

        match self {
            &Self::MagicMissile => {
                battle.enemy.hurt(4);
            },
            &Self::Drain => {
                battle.enemy.hurt(2);
                battle.player.heal(2);
            },
            &Self::Shield => {
                battle.player.armor += 7;
            },
            _ => { }
        }

        if let Some(effect) = self.effect() {
            battle.effects.insert(effect);
        }
    }
}

#[derive(Debug)]
enum BattleResult {
    Victory,
    Defeat
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Player {
    health: u32,
    mana: u32,
    armor: u32
}

impl Alive for Player {
    fn health(&self) -> u32 { self.health }
    fn health_mut(&mut self) -> &mut u32 { &mut self.health }
    fn armor(&self) -> u32 { self.armor }
}

#[derive(Clone, PartialEq, Eq)]
struct Enemy {
    health: u32,
    damage: u32,
}

impl Enemy {
    fn parse(input: &str) -> Result<Enemy, String> {
        let kv = |key| delimited(tag(key).and(tag(": ")), complete::u32, optional_newline);

        let mut entity = kv("Hit Points").and(kv("Damage"))
            .map(|(health, damage)| Enemy { health, damage });

        Ok(entity.parse(input).map_err(|err: nom::Err<VerboseError<&str>>| err.to_string())?.1)
    }
}

impl Alive for Enemy {
    fn health(&self) -> u32 { self.health }
    fn health_mut(&mut self) -> &mut u32 { &mut self.health }
    fn armor(&self) -> u32 { 0 }
}

#[derive(Clone, PartialEq, Eq)]
struct Battle {
    player: Player,
    enemy: Enemy,
    effects: HashSet<StatusEffect>,
    mana_expended: u32,
    difficulty: Difficulty
}

impl Ord for Battle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-(self.mana_expended as i32), self.enemy.health)
            .cmp(&(-(other.mana_expended as i32), other.enemy.health))
    }
}

impl PartialOrd for Battle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Battle {
    fn new(player: Player, enemy: Enemy, difficulty: Difficulty) -> Battle {
        Battle {
            player, enemy, difficulty,
            mana_expended: 0,
            effects: HashSet::new()
        }
    }

    fn is_finished(&self) -> Option<BattleResult> {
        match (&self.player, &self.enemy) {
            (_, enemy) if !enemy.alive() => Some(BattleResult::Victory),
            (player, _) if !player.alive() || player.mana == 0 => Some(BattleResult::Defeat),
            _ => None
        }
    }

    fn apply_effects(&mut self) {
        let effects = self.effects.clone()
            .into_iter()
            .filter_map(|effect| effect.apply(self))
            .collect();

        self.effects = effects;
    }

    fn with_spell(mut self, spell: &Spell) -> Battle {
        match self.difficulty {
            Difficulty::Normal => { },
            Difficulty::Hard => {
                self.player.hurt(1);
                if !self.player.alive() { return self }
            }
        }

        spell.cast(&mut self);
        if !self.enemy.alive() { return self }

        self.apply_effects();
        self.player.hurt(self.enemy.damage);
        self.apply_effects();
        
        self
    }

    fn possible_moves(&self) -> Vec<Battle> {
        [
            Spell::MagicMissile,
            Spell::Poison,
            Spell::Drain,
            Spell::Recharge,
            Spell::Shield
        ].iter().filter(|spell| spell.castable(&self))
            .map(|spell| self.clone().with_spell(spell))
            .collect()
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Difficulty {
    Normal,
    Hard
}

fn least_amount_of_mana_for_victory(battle: Battle) -> Option<u32> {
    let mut queue: BinaryHeap<Battle> = BinaryHeap::from_iter(iter::once(battle));
    let mut best_mana: Option<u32> = None;

    while let Some(battle) = queue.pop() {
        if best_mana.map_or(false, |best| battle.mana_expended >= best) { continue; }

        match battle.is_finished() {
            Some(BattleResult::Victory) => {
                best_mana = Some(min(best_mana.unwrap_or(u32::MAX), battle.mana_expended));
            },
            None => {
                queue.extend(battle.possible_moves().into_iter());
            },
            Some(BattleResult::Defeat) => { }
        }
    }

    best_mana
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_22.txt")?;    
    let enemy = Enemy::parse(&content)?;

    let player = Player {
        health: 50,
        mana: 500,
        armor: 0
    };

    let battle = Battle::new(player.clone(), enemy.clone(), Difficulty::Normal);
    let mana = least_amount_of_mana_for_victory(battle.clone())
        .expect("Player can't win");

    println!("The least amount of mana to expend in order to win is {}", mana);

    let battle = Battle::new(player, enemy, Difficulty::Hard);
    let mana = least_amount_of_mana_for_victory(battle)
        .expect("Player can't win");

    println!("The least amount of mana to expend in order to win on hard difficulty is {}", mana);

    Ok(())
}