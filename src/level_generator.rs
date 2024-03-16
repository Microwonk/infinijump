use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashSet};

use crate::{
    config::{TILE_HEIGHT, TILE_SCALE, TILE_WIDTH},
    GameState,
};

pub(crate) mod perlin;

#[derive(Resource, Default)]
pub struct Seed(u32);

pub struct LevelGeneratorPlugin<T: LevelGenerator> {
    seed: u32,
    _phantom: PhantomData<T>,
}

impl<T: LevelGenerator> Default for LevelGeneratorPlugin<T> {
    fn default() -> Self {
        Self {
            seed: 0,
            _phantom: default(),
        }
    }
}

impl<T: LevelGenerator> LevelGeneratorPlugin<T> {
    pub fn seeded(seed: u32) -> Self {
        Self {
            seed,
            _phantom: default(),
        }
    }

    // TODO: is only temporary
    fn level_generator_system(mut commands: Commands, seed: Res<Seed>) {
        let chunk = T::generate(seed.0, (500, 500));
        for tile in chunk.data {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex(tile.color).expect("could not parse tile color"),
                    ..default()
                },
                transform: Transform::from_xyz(
                    tile.pos.0 as f32 * TILE_WIDTH,
                    tile.pos.1 as f32 * TILE_HEIGHT,
                    tile.z_index as f32,
                )
                .with_scale(Vec3::splat(TILE_SCALE)),
                ..default()
            });
        }
    }
}

pub trait LevelGenerator
where
    Self: Send + Sync + 'static,
{
    fn generate(seed: u32, dimensions: (u32, u32)) -> Chunk;
}

impl<T: LevelGenerator> Plugin for LevelGeneratorPlugin<T> {
    fn build(&self, app: &mut App) {
        // TODO: add actual systems like chunk despawning and dynamic loading when the player enters the space
        // (generic, so that when i implement new algorithms, it does not break lol)
        app.world.insert_resource(Seed(self.seed));
        app.add_systems(OnEnter(GameState::Playing), Self::level_generator_system);
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Tile {
    pos: (i32, i32),
    color: &'static str,
    z_index: i32,
}

impl Tile {
    pub fn new(pos: (i32, i32), color: &'static str, z_index: i32) -> Self {
        Tile {
            pos,
            color,
            z_index,
        }
    }
}

pub struct Chunk {
    pub width: u32,
    pub height: u32,
    pub data: HashSet<Tile>,
}
