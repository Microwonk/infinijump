use std::fmt::Debug;

use bevy::prelude::*;

pub const TILE_HEIGHT: f32 = 32.0;
pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_SCALE: f32 = 32.0;

// TODO: this is just overengineered, should use a struct with data and act on that data seperately for visuals
// was very useful for learning purposes though (dyn and traits)
pub trait Tile
where
    Self: Debug + Send + Sync + 'static,
{
    fn make_sprite_bundle(&self) -> SpriteBundle;
    fn pos(&self) -> (i32, i32);
}

#[derive(Debug)]
pub struct ColorTile {
    pos: (i32, i32),
    pub(crate) color: &'static str,
    pub(crate) z_index: i32,
}

impl Tile for ColorTile {
    fn make_sprite_bundle(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::hex(self.color).expect("could not parse tile color"),
                ..default()
            },
            transform: Transform::from_xyz(
                self.pos.0 as f32 * TILE_WIDTH,
                self.pos.1 as f32 * TILE_HEIGHT,
                self.z_index as f32,
            )
            .with_scale(Vec3::splat(TILE_SCALE)),
            ..default()
        }
    }

    fn pos(&self) -> (i32, i32) {
        self.pos
    }
}

impl ColorTile {
    pub fn new(pos: (i32, i32), color: &'static str, z_index: i32) -> Self {
        Self {
            pos,
            color,
            z_index,
        }
    }
}

#[derive(Debug)]
pub struct TexturedTile {
    pos: (i32, i32),
    pub tex_index: u32,
    pub z_index: i32,
}

impl Tile for TexturedTile {
    fn make_sprite_bundle(&self) -> SpriteBundle {
        todo!()
    }

    fn pos(&self) -> (i32, i32) {
        self.pos
    }
}

impl TexturedTile {
    pub fn new(pos: (i32, i32), tex_index: u32, z_index: i32) -> Self {
        Self {
            pos,
            tex_index,
            z_index,
        }
    }
}
