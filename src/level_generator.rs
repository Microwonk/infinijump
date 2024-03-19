use std::marker::PhantomData;

use crate::GameState;
use bevy::{prelude::*, utils::HashMap};
use bevy_xpbd_2d::prelude::*;
use rand::{thread_rng, Rng};

pub(crate) mod chunk;
pub(crate) mod perlin_generator;
pub(crate) mod tile;

use chunk::*;
use tile::*;

pub const DEFAULT_CHUNK_NEIGHBORS_3X3: [(i32, i32); 9] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (0, 0),
];

pub const DEFAULT_CHUNK_NEIGHBORS_5X5: [(i32, i32); 25] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (-2, -2),
    (-1, -2),
    (0, -2),
    (1, -2),
    (2, -2),
    (-2, -1),
    (2, -1),
    (-2, 0),
    (2, 0),
    (-2, 1),
    (2, 1),
    (-2, 2),
    (-1, 2),
    (0, 2),
    (1, 2),
    (2, 2),
];

#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub struct ChunkGenerationSet;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Seed(pub u32);

#[derive(Resource, Default)]
pub struct ChunkLayerInfo(HashMap<u32, &'static str>);

#[derive(Default)]
pub struct LevelGeneratorPlugin<L: LevelGenerator, F: Component> {
    seed: u32,
    chunk_layer_info: HashMap<u32, &'static str>,
    _phantom_l: PhantomData<L>,
    _phantom_f: PhantomData<F>,
}

impl<L: LevelGenerator, F: Component> LevelGeneratorPlugin<L, F> {
    pub fn seeded(seed: u32) -> Self {
        Self {
            seed,
            chunk_layer_info: HashMap::new(),
            _phantom_l: default(),
            _phantom_f: default(),
        }
    }

    pub fn with_layer_info(mut self, chunk_layer_info: HashMap<u32, &'static str>) -> Self {
        self.chunk_layer_info = chunk_layer_info;
        self
    }

    fn gen_chunks_around_focal_point(
        mut commands: Commands,
        seed: Res<Seed>,
        chunk_layer_info: Res<ChunkLayerInfo>,
        focal: Query<&Transform, With<F>>,
        existing_chunks: Query<&ChunkMarker>,
    ) {
        if focal.is_empty() {
            info!("No focal point found, skipping chunk generation. TIP: The Component must also have a Transform component.");
            return;
        }

        focal.for_each(|t| {
            let existing_chunks = existing_chunks
                .iter()
                .map(|ch| ch.ch_pos)
                .collect::<Vec<_>>();

            let focal_in_chunk_space = to_chunk_space((t.translation.x, t.translation.y));
            let (x, y) = focal_in_chunk_space;

            for (i, j) in DEFAULT_CHUNK_NEIGHBORS_5X5.iter() {
                let (x, y) = (x + *i, y + *j);
                if existing_chunks.contains(&(x, y)) {
                    continue;
                }

                let start = (x * CHUNK_WIDTH as i32, y * CHUNK_HEIGHT as i32);
                let chunk = L::generate_chunk(seed.0, start, chunk_layer_info.0.clone());

                commands
                    .spawn(ChunkMarker::new(chunk.ch_pos))
                    .insert(TransformBundle::default())
                    .insert(VisibilityBundle::default())
                    .with_children(|child_builder| {
                        // for each tile in the chunk, spawn a sprite
                        // TODO
                        chunk.data.iter().for_each(|tile| {
                            child_builder.spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::hex("#DAF7A6")
                                        .expect("could not parse tile color"),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    tile.pos().0 as f32 * TILE_WIDTH,
                                    tile.pos().1 as f32 * TILE_HEIGHT,
                                    tile.z_index() as f32,
                                )
                                .with_scale(Vec3::splat(TILE_SCALE)),
                                ..default()
                            });
                        });

                        // iterating over the generate_colliders hashmap, because if we change the algorithm for generating colliders,
                        // we don't have to change this (will not impact performance THAT much anyway)
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
        });
    }

    fn despawn_chunks_around_focal_point(
        mut commands: Commands,
        focal: Query<&Transform, With<F>>,
        existing_chunks: Query<(Entity, &ChunkMarker)>,
    ) {
        if focal.is_empty() {
            info!("No focal point found, skipping chunk despawning. TIP: The Component must also have a Transform component.");
            return;
        }

        focal.for_each(|t| {
            let focal_in_chunk_space = to_chunk_space((t.translation.x, t.translation.y));

            let (x, y) = focal_in_chunk_space;

            existing_chunks.for_each(|(ent, chunk)| {
                let (cx, cy) = chunk.ch_pos;
                if !DEFAULT_CHUNK_NEIGHBORS_5X5.contains(&(x - cx, y - cy)) {
                    commands.entity(ent).despawn_recursive();
                }
            });
        });
    }

    fn reset_seed(mut seed: ResMut<Seed>, input: Res<Input<KeyCode>>) {
        if input.just_pressed(KeyCode::R) {
            seed.0 = thread_rng().gen();
        }
    }
}

pub trait LevelGenerator
where
    Self: Send + Sync + 'static,
{
    fn generate_chunk(
        seed: u32,
        start: (i32, i32),
        chunk_layer_info: HashMap<u32, &'static str>,
    ) -> Chunk;
}

impl<L: LevelGenerator, F: Component> Plugin for LevelGeneratorPlugin<L, F> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Seed(self.seed))
            .insert_resource(ChunkLayerInfo(self.chunk_layer_info.clone()))
            .add_systems(
                Update,
                (
                    Self::reset_seed,
                    Self::gen_chunks_around_focal_point,
                    Self::despawn_chunks_around_focal_point,
                )
                    .chain()
                    .in_set(ChunkGenerationSet)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
