use std::ops::{Add, Mul};

use raylib::prelude::*;

mod player;
use player::*;

mod assets;
use assets::*;
use systems::inventory;

mod systems;

fn main() {
    let (w, h) = (1600, 900);
    let (mut rl, thread) = raylib::init().title("Noha").size(w, h).build();

    rl.set_target_fps(60);
    rl.set_exit_key(None);

    let mut screen = ScreenData::new(w, h);

    let mut assets = AssetStorage::new(&mut rl, &thread);

    let mut player = Player::new(&mut assets, &mut rl, &thread);

    let grid = systems::procgen::Grid::new();
    let grid_image = systems::worlddata::temp_draw_grid(&grid);
    let grid_texture = rl.load_texture_from_image(&thread, &grid_image).unwrap();

    while !rl.window_should_close() {
        let time = std::time::Instant::now();
        if rl.is_window_resized() {
            let (w, h) = (rl.get_screen_width(), rl.get_screen_height());
            screen.resize(w, h);
        }

        player.update(&rl, &thread);

        screen.move_to(player.position);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_texture(&grid_texture, 0, 0, Color::WHITE);
        {
            let mut d2 = d.begin_mode2D(screen.camera);

            player.draw(&assets, &mut d2);

            d2.draw_rectangle(-300, assets.player.height, 1000, 1000, Color::RED);
        }

        d.draw_fps(10, 10);

        player.inventory.draw(&mut d, &assets);

        let frametime = time.elapsed().as_millis();

        d.draw_text(
            &format!("Frametime: {}ms", frametime),
            10,
            30,
            20,
            Color::BLACK,
        );
    }
}
