use bevy::prelude::*;

pub const TILE_HEIGHT: f32 = 32.0;
pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_SCALE: f32 = 32.0;

pub trait Tile
where
    Self: Sized + Send + Sync + 'static,
{
    fn make_sprite_bundle(&self) -> SpriteBundle;
    fn pos(&self) -> (i32, i32);
}

#[derive(Eq, PartialEq, Debug)]
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
pub enum TexturedTile {
    Atlas {
        pos: (i32, i32),
        texture_path: &'static str,
        tex_index: u32,
        z_index: i32,
    },
    Single {
        pos: (i32, i32),
        texture_path: &'static str,
        z_index: i32,
    },
}

impl Tile for TexturedTile {
    fn make_sprite_bundle(&self) -> SpriteBundle {
        todo!()
    }

    fn pos(&self) -> (i32, i32) {
        todo!()
    }
}

impl TexturedTile {
    pub fn atlas(
        pos: (i32, i32),
        texture_path: &'static str,
        tex_index: u32,
        z_index: i32,
    ) -> Self {
        Self::Atlas {
            pos,
            texture_path,
            tex_index,
            z_index,
        }
    }

    pub fn single(pos: (i32, i32), texture_path: &'static str, z_index: i32) -> Self {
        Self::Single {
            pos,
            texture_path,
            z_index,
        }
    }
}
