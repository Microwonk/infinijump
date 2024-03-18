use bevy::{prelude::*, utils::HashMap};
use bevy_xpbd_2d::components::Collider;

use super::{Tile, TILE_HEIGHT, TILE_WIDTH};

pub const CHUNK_WIDTH: f32 = 16.0;
pub const CHUNK_HEIGHT: f32 = 16.0;

#[derive(Debug, Default)]
pub struct Chunk {
    pub ch_pos: (i32, i32),
    pub width: u32,
    pub height: u32,
    pub data: Vec<Box<dyn Tile>>,
}

impl Chunk {
    // TODO: make this more efficient?
    pub fn generate_colliders(&self) -> HashMap<(i32, i32), Collider> {
        let mut colliders = HashMap::new();
        for tile in &self.data {
            colliders.insert(tile.pos(), Collider::cuboid(TILE_WIDTH, TILE_HEIGHT));
        }
        colliders
    }
}

pub fn to_chunk_space((x, y): (f32, f32)) -> (i32, i32) {
    (
        (x / CHUNK_WIDTH / TILE_WIDTH).floor() as i32,
        (y / CHUNK_HEIGHT / TILE_HEIGHT).floor() as i32,
    )
}

#[derive(Component)]
pub struct ChunkMarker {
    pub ch_pos: (i32, i32),
}

impl ChunkMarker {
    pub fn new(ch_pos: (i32, i32)) -> Self {
        Self { ch_pos }
    }
}
