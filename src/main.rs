use std::ops::{Add, Mul};

use raylib::prelude::*;

mod player;
use player::*;

mod assets;
use assets::*;

mod systems;

fn main() {
    let (w, h) = (1600, 900);
    let (mut rl, thread) = raylib::init().title("Noha").size(w, h).build();

    rl.set_target_fps(60);
    rl.set_exit_key(None);

    let mut screen = ScreenData::new(w, h);

    let assets = AssetStorage::new(&mut rl, &thread);

    let mut player = Player::new();

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            let (w, h) = (rl.get_screen_width(), rl.get_screen_height());
            screen.resize(w, h);
        }

        player.update(&rl, &thread);

        screen.move_to(player.position);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d2 = d.begin_mode2D(screen.camera);

            player.draw(&assets, &mut d2);

            d2.draw_rectangle(-300, assets.player.height, 1000, 1000, Color::RED);
        }

        d.draw_fps(10, 10);
    }
}
