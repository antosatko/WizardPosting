use raylib::{prelude::*, ffi::{GenImageColor, LoadTextureFromImage}};

use super::procgen::Tiles;

impl Tiles {
    pub fn get_color(&self) -> Color {
        match self {
            Tiles::Grass => Color::GREEN,
            Tiles::Dirt => Color::BROWN,
            Tiles::Stone => Color::GRAY,
            Tiles::Leaves => Color::GREEN,
            Tiles::Air => Color::BLANK,
            Tiles::HardStone => Color::DARKGRAY,
            Tiles::IronOre => Color { r: 255, g: 0, b: 0, a: 255 },
            Tiles::GoldOre => Color { r: 255, g: 255, b: 0, a: 255 },
            Tiles::RuneOre => Color { r: 0, g: 255, b: 255, a: 255 },
            Tiles::Ruby => Color { r: 255, g: 128, b: 128, a: 255 },
            Tiles::Emerald => Color { r: 128, g: 255, b: 128, a: 255 },
            Tiles::Sapphire => Color { r: 128, g: 128, b: 255, a: 255 },
            Tiles::Diamond => Color { r: 255, g: 255, b: 255, a: 255 },
            Tiles::Log => Color::BROWN,
        }
    }
}

pub fn temp_draw_grid(
    grid: &super::procgen::Grid,
) -> Image {
    let mut d = Image::gen_image_color(1600, 900, Color::BLANK);

    let (tile_width, tile_height) = (1600. / grid.width as f64, 900. / grid.height as f64);

    for x in 0..grid.width {
        for y in 0..grid.height {
            let tile = grid.get_tile(x, y);
            if tile.tile != Tiles::Air {
                d.draw_rectangle(
                    (x as f64 * tile_width) as i32,
                    (y as f64 * tile_height) as i32,
                    tile_width as i32 + 1,
                    tile_height as i32 + 1,
                    tile.tile.get_color(),
                );
            }
        }
    }

    d
}