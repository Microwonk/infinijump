use noise::{NoiseFn, Perlin};

use super::*;

const NOISE_SCALE: f64 = 12.5;

pub struct SimplePerlinLevelGenerator;

impl LevelGenerator for SimplePerlinLevelGenerator {
    fn generate_chunk(
        seed: u32,
        (start_x, start_y): (i32, i32),
        chunk_layer_info: HashMap<u32, &'static str>,
    ) -> Chunk {
        let mut data = Vec::new();
        let perlin = Perlin::new(seed);
        let (end_x, end_y) = (start_x + CHUNK_WIDTH as i32, start_y + CHUNK_HEIGHT as i32);
        for x in start_x - 1..end_x + 1 {
            for y in start_y - 1..end_y + 1 {
                let value = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
                if value > 0.2 {
                    data.push(Tile::Generic {
                        pos: (x, y),
                        z_index: 0,
                    });
                }
                if value > 0.4 {
                    data.push(Tile::Generic {
                        pos: (x, y),
                        z_index: 1,
                    });
                }
                if value > 0.6 {
                    data.push(Tile::Generic {
                        pos: (x, y),
                        z_index: 2,
                    });
                }
                if value > 0.8 {
                    data.push(Tile::Generic {
                        pos: (x, y),
                        z_index: 3,
                    });
                }
            }
        }
        Chunk {
            ch_pos: (start_x / CHUNK_WIDTH as i32, start_y / CHUNK_HEIGHT as i32),
            layer_info: chunk_layer_info,
            width: CHUNK_WIDTH as u32,
            height: CHUNK_HEIGHT as u32,
            data,
        }
    }
}
