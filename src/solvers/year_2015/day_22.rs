use std::{cmp::{min, max}, collections::VecDeque, iter, hash::{Hash, Hasher}};
use anyhow::Result;
use ahash::{HashSet, HashSetExt};
use aoc_lib::parsing::{TextParser, Parsable, TextParserResult};
use yuki::errors::NoSolution;
use crate::SolverResult;
use nom:: {
    bytes::complete::tag, character::complete::{line_ending, u32}, combinator::map, sequence::{preceded, separated_pair}, Parser
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.effect.hash(state);
    }
}

impl PartialEq for StatusEffect {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect
    }
}

impl StatusEffect {
    fn apply(mut self, battle: &mut Battle) -> Option<Self> {
        self.duration -= 1;
        
        match self {
            Self { effect: Effect::Poisoned, duration: _ } => {
                battle.enemy.hurt(3);
            },
            Self { effect: Effect::Recharging, duration: _ } => {
                battle.player.mana += 101;
            }
            Self { effect: Effect::Shielded, duration: 0 } => {
                battle.player.armor -= 7;
            }
            _ => {}
        }

        (!self.ended()).then_some(self)
    }
    
    const fn ended(&self) -> bool { self.duration == 0 }
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
    const fn cost(&self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain        => 73,
            Self::Shield       => 113,
            Self::Poison       => 173,
            Self::Recharge     => 229
        }
    }

    const fn effect(&self) -> Option<StatusEffect> {
        match self {
            Self::Poison   => Some(StatusEffect { effect: Effect::Poisoned,   duration: 6 }),
            Self::Shield   => Some(StatusEffect { effect: Effect::Shielded,   duration: 6 }),
            Self::Recharge => Some(StatusEffect { effect: Effect::Recharging, duration: 5 }),
            _ => None
        }
    }
}

trait Alive {
    fn health(&self) -> u32;
    fn health_mut(&mut self) -> &mut u32;
    fn armor(&self) -> u32;

    fn hurt(&mut self, damage: u32) {
        let true_damage = max(1, damage.saturating_sub(self.armor()));

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

        self
            .effect()
            .is_none_or(|effect| !battle.effects.contains(&effect))
    }

    fn cast(&self, battle: &mut Battle) {
        battle.player.mana -= self.cost();
        battle.mana_expended += self.cost();

        match self {
            Self::MagicMissile => {
                battle.enemy.hurt(4);
            },
            Self::Drain => {
                battle.enemy.hurt(2);
                battle.player.heal(2);
            },
            Self::Shield => {
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

impl Parsable<'_> for Enemy {
    fn parse(input: &str) -> TextParserResult<Self> {
        map(
            separated_pair(
                preceded(tag("Hit Points: "), u32),
                line_ending,
                preceded(tag("Damage: "), u32)
            ),
            |(health, damage)| Self { health, damage }
        )
        .parse(input)
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
        self.mana_expended.cmp(&other.mana_expended).reverse()
    }
}

impl PartialOrd for Battle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Battle {
    fn new(player: Player, enemy: Enemy, difficulty: Difficulty) -> Self {
        Self {
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
        let effects = self.effects
            .clone()
            .into_iter()
            .filter_map(|effect| effect.apply(self))
            .collect();

        self.effects = effects;
    }

    fn with_spell(mut self, spell: &Spell) -> Self {
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

    fn possible_moves(&self) -> Vec<Self> {
        [
            Spell::MagicMissile,
            Spell::Poison,
            Spell::Drain,
            Spell::Recharge,
            Spell::Shield
        ].iter().filter(|spell| spell.castable(self))
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
    let mut queue: VecDeque<Battle> = iter::once(battle).collect();
    let mut best_mana: Option<u32> = None;

    while let Some(battle) = queue.pop_front() {
        if best_mana.is_some_and(|best| battle.mana_expended >= best) { continue; }

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

fn least_mana_for_input(input: &str, difficulty: Difficulty) -> Result<u32> {
    let enemy = Enemy::parse.run(input)?;

    let player = Player {
        health: 50,
        mana: 500,
        armor: 0
    };

    let battle = Battle::new(player, enemy, difficulty);

    let mana = least_amount_of_mana_for_victory(battle)
        .ok_or(NoSolution)?;

    Ok(mana)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mana = least_mana_for_input(input, Difficulty::Normal)?;
    Ok(Box::from(mana))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mana = least_mana_for_input(input, Difficulty::Hard)?;
    Ok(Box::from(mana))
}