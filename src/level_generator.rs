use std::marker::PhantomData;

use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::{thread_rng, Rng};

pub(crate) mod chunk;
pub(crate) mod perlin_generator;
pub(crate) mod tile;

use chunk::*;
use tile::*;

pub const DEFAULT_CHUNK_NEIGHBORS: [(i32, i32); 9] = [
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

#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub struct ChunkGenerationSet;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Seed(pub u32);

#[derive(Default)]
pub struct LevelGeneratorPlugin<L: LevelGenerator, F: Component> {
    seed: u32,
    _phantom_l: PhantomData<L>,
    _phantom_f: PhantomData<F>,
}

impl<L: LevelGenerator, F: Component> LevelGeneratorPlugin<L, F> {
    pub fn seeded(seed: u32) -> Self {
        Self {
            seed,
            _phantom_l: default(),
            _phantom_f: default(),
        }
    }

    fn gen_chunks_around_focal_point(
        mut commands: Commands,
        seed: Res<Seed>,
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

            for (i, j) in DEFAULT_CHUNK_NEIGHBORS.iter() {
                let (x, y) = (x + *i, y + *j);
                if existing_chunks.contains(&(x, y)) {
                    continue;
                }

                let start = (x * CHUNK_WIDTH as i32, y * CHUNK_HEIGHT as i32);
                let chunk = L::generate_chunk(seed.0, start);

                commands
                    .spawn(ChunkMarker::new(chunk.ch_pos))
                    .insert(TransformBundle::default())
                    .insert(VisibilityBundle::default())
                    .with_children(|child_builder| {
                        // for each tile in the chunk, spawn a sprite
                        chunk
                            .data
                            .iter()
                            .for_each(|tile| _ = child_builder.spawn(tile.make_sprite_bundle()));

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
            info!("No focal point found, skipping chunk despawning. The Component must also have a Transform component.");
            return;
        }

        focal.for_each(|t| {
            let focal_in_chunk_space = to_chunk_space((t.translation.x, t.translation.y));

            let (x, y) = focal_in_chunk_space;

            existing_chunks.for_each(|(ent, chunk)| {
                let (cx, cy) = chunk.ch_pos;
                if !DEFAULT_CHUNK_NEIGHBORS.contains(&(x - cx, y - cy)) {
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
    type Tile: Tile;
    fn generate_chunk(seed: u32, start: (i32, i32)) -> Chunk<Self::Tile>;
}

impl<L: LevelGenerator, F: Component> Plugin for LevelGeneratorPlugin<L, F> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Seed(self.seed)).add_systems(
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
