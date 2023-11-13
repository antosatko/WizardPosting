use std::ops::{Add, Mul};

use raylib::prelude::*;

pub struct AssetStorage {
    pub player: raylib::texture::Texture2D,
    pub magicball: raylib::texture::Texture2D,
}

impl AssetStorage {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> AssetStorage {
        let player = Self::load_texture(rl, thread, "assets/player.png");
        let magicball = Self::load_texture(rl, thread, "assets/magicball.png");
        AssetStorage { player, magicball }
    }

    pub fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Texture2D {
        let img = Image::load_image(path).unwrap();
        rl.load_texture_from_image(thread, &img).unwrap()
    }
}

pub struct ScreenData {
    pub dimensions: Vector2,
    pub camera: Camera2D,
}

impl ScreenData {
    pub fn new(w: i32, h: i32) -> ScreenData {
        ScreenData {
            dimensions: Vector2::new(w as f32, h as f32),
            camera: Camera2D {
                target: Vector2::new(0.0, 0.0),
                offset: Vector2::new(w as f32 / 2.0, h as f32 / 2.0),
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.dimensions = Vector2::new(w as f32, h as f32);
        self.camera.offset = Vector2::new(w as f32 / 2.0, h as f32 / 2.0);
    }

    pub fn move_to(&mut self, pos: Vector2) {
        self.camera.target = self.camera.target.lerp(pos, 0.2);
    }
}
