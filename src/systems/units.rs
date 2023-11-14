use raylib::prelude::*;

use crate::assets;

use super::{attacks::AttackNodes, effects::Effect};

pub trait Unit {
    fn update(&mut self, rl: &RaylibHandle, thread: &RaylibThread);

    fn draw(&self, assets: &assets::AssetStorage, d: &mut RaylibMode2D<RaylibDrawHandle>);

    fn get_speed_mut(&mut self) -> &mut Vector2;

    fn get_position_mut(&mut self) -> &mut Vector2;

    fn set_speed(&mut self, speed: Vector2);

    fn set_position(&mut self, pos: Vector2);

    fn get_effect_stats_mut(&mut self) -> &mut EffectStats;

    fn get_stats_mut(&mut self) -> &mut Stats;

    fn get_effects_mut(&mut self) -> &mut Vec<Effect>;
}

#[derive(Debug, Clone, Copy)]
pub struct EffectStats {
    pub can_jump: bool,
    pub can_move: bool,
    pub can_teleport: bool,
    pub can_fly: bool,
    pub has_gravity: bool,
    pub can_croutch: bool,
}

impl EffectStats {
    pub fn new() -> EffectStats {
        EffectStats {
            can_jump: true,
            can_move: true,
            can_teleport: true,
            can_fly: true,
            has_gravity: true,
            can_croutch: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub acceleration: f32,
    pub max_speed: f32,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            health: 100,
            max_health: 100,
            mana: 100,
            max_mana: 100,
            stamina: 100,
            max_stamina: 100,
            acceleration: 3.0,
            max_speed: 7.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Summons {
    Bat {
        stats: Stats,
        effects: Vec<Effect>,
        on_attack: Option<Box<AttackNodes>>,
        on_death: Option<Box<AttackNodes>>,
    },
}
