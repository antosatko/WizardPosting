use raylib::prelude::*;

use super::attacks::Attacks;

#[derive(Debug, Clone)]
pub struct Weapon {
    pub kind: Weapons,
}

#[derive(Debug, Clone)]
pub enum Weapons {
    Staff {
        primary: Option<Attacks>,
        secondary: Option<Attacks>,
    },
    Sword {},
}
