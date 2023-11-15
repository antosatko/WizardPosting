use std::ops::{Mul, Div};

use raylib::prelude::*;

use super::procgen::{Tile, Grid};


/// checks collisions for tile grid
pub fn grid(grid: &Grid, hitbox: Vector4, speed: Vector2) {
    let mut x = hitbox.x;
    let mut y = hitbox.y;
    let speed = todo!();
}