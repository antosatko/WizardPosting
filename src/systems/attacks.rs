use raylib::prelude::*;

use crate::player::{PlayerId, Sides};

use super::{
    effects::Effect,
    units::{Summons, Unit}, mouse_mode::{Pointer, Pointers},
};

pub struct Attack {
    pub duration: f32,
    pub max_duration: f32,
    pub source: AttackSources,
    pub kind: AttackNodes,
    pub pointer: Pointer,
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
    pub target: AttackTargets,
}

pub enum AttackTargets {
    Players,
    Familiars,
    Neutral,
    Enemy,
    All,
}

#[derive(Debug, Clone)]
pub enum Attacks {
    Magicball {
        unit_hit: Option<Box<AttackNodes>>,
        environment_hit: Option<Box<AttackNodes>>,
    },
    Slash {
        distance: f32,
    },
    Arrow {},
}

impl Attack {
    pub fn new(kind: AttackNodes, cooldown: f32, source: AttackSources) -> Self {
        let pointer = match &kind {
            AttackNodes::Attack(attack) => match &attack {
                Attacks::Magicball { unit_hit: _, environment_hit: _ } => Pointers::Projectile,
                Attacks::Slash { distance } => Pointers::FixedDistance { distance: *distance },
                Attacks::Arrow {  } => Pointers::Projectile,
            },
            AttackNodes::Effect(_) => Pointers::This,
            AttackNodes::Summon(_) => Pointers::This,
        };
        let pointer = Pointer {
            kind: pointer,
        };
        Self {
            duration: 0.0,
            max_duration: cooldown,
            source,
            kind,
            pointer,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttackNodes {
    Attack(Attacks),
    Effect(Effect),
    Summon(Summons),
}
