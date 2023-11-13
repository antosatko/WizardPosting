use raylib::prelude::*;

use crate::player::{PlayerId, Sides};

use super::effects::Effect;

pub struct Attack {
    pub duration: f32,
    pub max_duration: f32,
    pub source: AttackSources,
}

pub enum AttackSources {
    /// ID of the player
    Player(PlayerId),
    /// ID of the familiars master
    Familiar(PlayerId),
    Enemy,
}

pub struct AttackTarget {
    pub side: Sides,
    pub target: AttackTargets
}

pub enum AttackTargets {
    Players,
    Familiars,
    All,
}


#[derive(Debug, Clone)]
pub enum Attacks {
    Magicball {
        position: Vector2,
        speed: Vector2,
        unit_hit: Option<Box<AttackNodes>>,
        environment_hit: Option<Box<AttackNodes>>,
    },
    Slash {

    },
    Arrow {

    },
}


#[derive(Debug, Clone)]
pub enum AttackNodes {
    Attack(Attacks),
    Effect(Effect),
}