use raylib::prelude::*;

use crate::assets;

pub trait Unit {
    fn update(&mut self, rl: &RaylibHandle, thread: &RaylibThread);

    fn draw(&self, assets: &assets::AssetStorage, d: &mut RaylibMode2D<RaylibDrawHandle>);

    fn get_speed(&self) -> Vector2;

    fn get_position(&self) -> Vector2;

    fn set_speed(&mut self, speed: Vector2);

    fn set_position(&mut self, pos: Vector2);
}
