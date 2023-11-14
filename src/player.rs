use raylib::prelude::*;

use crate::{
    assets,
    systems::{
        effects::{Effect, Effects},
        inventory::Inventory,
        units::{EffectStats, Stats, Unit},
        up::Up,
    },
};

const GRAVITY: f32 = 0.5;
const AIR_RESISTANCE: f32 = 0.002;
const GROUND_FRICTION: f32 = 0.1;
pub struct Player {
    pub position: Vector2,
    pub speed: Vector2,
    pub up: Up,
    /// The player's stats
    pub hard_stats: Stats,
    /// The player's stats, modified by effects
    pub stats: Stats,
    pub effects: Vec<Effect>,
    pub effect_stats: EffectStats,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(
        assets: &mut assets::AssetStorage,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Self {
        Player {
            position: Vector2::new(0.0, 0.0),
            speed: Vector2::new(0.0, 0.0),
            up: Up::new(),
            hard_stats: Stats::new(),
            stats: Stats::new(),
            effects: vec![],
            effect_stats: EffectStats::new(),
            inventory: Inventory::new(assets, rl, thread),
        }
    }

    pub fn draw(&self, assets: &assets::AssetStorage, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_texture(
            &assets.player,
            self.position.x as i32,
            self.position.y as i32,
            Color::WHITE,
        );
    }

    pub fn update(&mut self, rl: &RaylibHandle, thread: &RaylibThread) {
        self.stats = self.hard_stats.clone();
        self.effect_stats = EffectStats::new();

        self.inventory.scroll(rl.get_mouse_wheel_move() as i32);

        let mut i = 0;
        while i < self.effects.len() {
            self.effects[i] = self.effects[i].update(self);
            if self.effects[i].disposable() {
                self.effects.remove(i);
            } else {
                i += 1;
            }
        }

        if self.effect_stats.has_gravity {
            self.speed.y += GRAVITY;
        }

        if self.effect_stats.can_move {
            if rl.is_key_down(KeyboardKey::KEY_D) {
                self.speed.x = (self.speed.x + self.stats.acceleration).min(self.stats.max_speed);
            } else if rl.is_key_down(KeyboardKey::KEY_A) {
                self.speed.x = (self.speed.x - self.stats.acceleration).max(-self.stats.max_speed);
            }
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.up = self.up.up(self, rl, thread);
        } else if rl.is_key_released(KeyboardKey::KEY_W) {
            self.up = self.up.release(self, rl, thread);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) && self.effect_stats.can_croutch {
            self.up = self.up.down(self, rl, thread);
        }

        self.speed -= self.speed * AIR_RESISTANCE;
        self.position += self.speed;

        if self.on_ground() {
            self.up = self.up.land();
            self.position.y = 0.;
            self.speed.y = 0.;
            self.speed.x -= self.speed.x * GROUND_FRICTION;
        }
    }

    fn on_ground(&self) -> bool {
        self.position.y >= 0.
    }

    pub fn cleanse(&mut self) {
        let mut i = 0;
        while i < self.effects.len() {
            if self.effects[i].is_buff {
                self.effects.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl Unit for Player {
    fn update(&mut self, rl: &RaylibHandle, thread: &RaylibThread) {
        self.update(rl, thread);
    }

    fn draw(&self, assets: &assets::AssetStorage, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.draw(assets, d);
    }

    fn get_speed_mut(&mut self) -> &mut Vector2 {
        &mut self.speed
    }

    fn get_position_mut(&mut self) -> &mut Vector2 {
        &mut self.position
    }

    fn set_speed(&mut self, speed: Vector2) {
        self.speed = speed;
    }

    fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
    }

    fn get_effect_stats_mut(&mut self) -> &mut EffectStats {
        &mut self.effect_stats
    }

    fn get_stats_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }

    fn get_effects_mut(&mut self) -> &mut Vec<Effect> {
        &mut self.effects
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerId {
    pub id: usize,
    pub side: Sides,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sides {
    Blue,
    Red,
    All,
}

impl PlayerId {
    pub fn new(id: usize, side: Sides) -> PlayerId {
        if side == Sides::All {
            panic!("PlayerId cannot be created with side All");
        }
        PlayerId { id, side }
    }
}
