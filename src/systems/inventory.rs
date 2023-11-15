use raylib::prelude::*;

use crate::assets::AssetStorage;

use super::{
    attacks::{AttackNodes, Attacks},
    units::{Stats, Summons},
    weapons::{Weapon, Weapons},
};

pub const INVENTORY_SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
pub struct Assets {
    staff: usize,
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: [Option<Item>; INVENTORY_SIZE],
    pub selected: usize,
    pub assets: Assets,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: Items,
}

#[derive(Debug, Clone)]
pub enum Items {
    Weapon { kind: Weapon },
}

impl Items {
    pub fn get_texture<'a>(
        &'a self,
        assets: &'a AssetStorage,
        loaded_assets: &Assets,
    ) -> &Texture2D {
        match self {
            Items::Weapon { kind } => match kind.kind {
                Weapons::Staff { .. } => &assets.server[loaded_assets.staff].tex,
                Weapons::Sword { .. } => todo!(),
            },
        }
    }
}

impl Inventory {
    pub fn new(assets: &mut AssetStorage, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            items: [
                None,
                None,
                None,
                Some(Item {
                    kind: Items::Weapon {
                        kind: Weapon {
                            kind: Weapons::Staff {
                                primary: Some(Attacks::Magicball {
                                    unit_hit: Some(Box::new(AttackNodes::Summon(Summons::Bat {
                                        stats: Stats::new(),
                                        effects: vec![],
                                        on_attack: None,
                                        on_death: None,
                                    }))),
                                    environment_hit: None,
                                }),
                                secondary: None,
                            },
                        },
                    },
                }),
                None,
            ],
            selected: 0,
            assets: Assets {
                staff: assets.load_texture_dyn(rl, thread, "assets/staff.png"),
            },
        }
    }

    pub fn scroll(&mut self, direction: i32) {
        self.selected =
            (self.selected as i32 + direction).rem_euclid(INVENTORY_SIZE as i32) as usize;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &AssetStorage) {
        const SLOT_SPACE: i32 = 50;
        const PADDING: i32 = 3;
        d.draw_rectangle(
            0,
            900 - SLOT_SPACE,
            SLOT_SPACE * INVENTORY_SIZE as i32,
            SLOT_SPACE,
            Color::BLACK,
        );
        for i in 0..INVENTORY_SIZE {
            let x = i as i32 * SLOT_SPACE + PADDING;
            let y = 900 - SLOT_SPACE + PADDING;
            let tint = if i == self.selected {
                Color::WHITE
            } else {
                Color::GRAY
            };
            d.draw_rectangle(
                x,
                y,
                SLOT_SPACE - PADDING * 2,
                SLOT_SPACE - PADDING * 2,
                tint,
            );
            if let Some(item) = &self.items[i] {
                d.draw_texture(
                    item.kind.get_texture(assets, &self.assets),
                    x + PADDING,
                    y + PADDING,
                    tint,
                );
            }
        }
    }
}
