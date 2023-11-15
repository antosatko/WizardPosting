//! Describes how the game treats input based on selected item
//! 
//! For example:
//!  - player has a weapon selected
//!  -- the main and secondary options will trigger an attack
//! 
//! - player has selected empty slot
//! -- primary and secondary is locked
//! 
//! etc..

use super::attacks::AttackTarget;


pub struct Pointer {
    pub kind: Pointers,
}

pub enum Pointers {
    /// Points at a specific block
    Point {
        range: f32,
    },
    /// Calculates angle for projectile
    Projectile,
    /// Helps aim at entities
    Entity {
        range: f32,
        allowed: AttackTarget
    },
    /// Same as projectile, but always stretches the vector for the same distance
    FixedDistance {
        distance: f32,
    },
    /// Targets self
    This,
    /// Selects all that fit the target
    Select {
        target: AttackTarget
    }
}