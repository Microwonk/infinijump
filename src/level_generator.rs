use std::marker::PhantomData;

use crate::{
    config::{TILE_HEIGHT, TILE_SCALE, TILE_WIDTH},
    GameState,
};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_xpbd_2d::prelude::*;
use rand::{thread_rng, Rng};

pub(crate) mod perlin_generator;

// TODOs: chunking system and resetting stage from button input

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Seed(pub u32);

// TODO
pub struct SeedChangedEvent;

#[derive(Default)]
pub struct LevelGeneratorPlugin<T: LevelGenerator> {
    seed: u32,
    _phantom: PhantomData<T>,
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
        let chunk = T::generate_chunk(seed.0, (50, 50));

        commands
            .spawn(ChunkMarker)
            .insert(TransformBundle::default())
            .insert(VisibilityBundle::default())
            .with_children(|child_builder| {
                chunk.data.iter().for_each(|tile| {
                    child_builder.spawn(SpriteBundle {
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
                });

                chunk
                    .generate_colliders()
                    .iter()
                    .for_each(|(pos, collider)| {
                        child_builder.spawn((
                            TransformBundle::from_transform(Transform::from_xyz(
                                pos.0 as f32 * TILE_WIDTH,
                                pos.1 as f32 * TILE_HEIGHT,
                                0.,
                            )),
                            collider.clone(),
                            RigidBody::Static,
                        ));
                    });
            });
    }

    fn reset_seed(mut seed: ResMut<Seed>, input: Res<Input<KeyCode>>) {
        if input.just_pressed(KeyCode::R) {
            seed.0 = thread_rng().gen();
        }
    }

    fn reset_level(
        mut commands: Commands,
        chunks: Query<Entity, With<ChunkMarker>>,
        seed: Res<Seed>,
    ) {
        if seed.is_changed() {
            chunks.for_each(|chunk| commands.entity(chunk).despawn_recursive());
            // there has to be a better way LMAO
            Self::level_generator_system(commands, seed);
        }
    }
}

pub trait LevelGenerator
where
    Self: Send + Sync + 'static,
{
    fn generate_chunk(seed: u32, dimensions: (u32, u32)) -> Chunk;
}

pub trait LevelConfigurator: Send + Sync + 'static {}

impl<T: LevelGenerator> Plugin for LevelGeneratorPlugin<T> {
    fn build(&self, app: &mut App) {
        // TODO: add actual systems like chunk despawning and dynamic loading when the player enters the space
        // (generic, so that when i implement new algorithms, it does not break lol)
        app.insert_resource(Seed(self.seed))
            .add_systems(OnEnter(GameState::Playing), Self::level_generator_system)
            .add_systems(
                Update,
                (Self::reset_seed, Self::reset_level)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
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

impl Chunk {
    // TODO: make this more efficient?
    pub fn generate_colliders(&self) -> HashMap<(i32, i32), Collider> {
        let mut colliders = HashMap::new();
        for tile in &self.data {
            colliders.insert(tile.pos, Collider::cuboid(TILE_WIDTH, TILE_HEIGHT));
        }
        colliders
    }
}

#[derive(Component)]
pub struct ChunkMarker;
