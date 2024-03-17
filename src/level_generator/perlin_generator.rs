use noise::{NoiseFn, Perlin};

use super::*;

pub const PERLIN_NOISE_SCALE: f64 = 12.5;

pub struct PerlinLevelGenerator;

impl LevelGenerator for PerlinLevelGenerator {
    fn generate_chunk(seed: u32, (width, height): (u32, u32)) -> Chunk {
        let mut data = HashSet::new();
        let perlin = Perlin::new(seed);
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                let value =
                    perlin.get([x as f64 / PERLIN_NOISE_SCALE, y as f64 / PERLIN_NOISE_SCALE]);
                if value > 0.2 {
                    data.insert(Tile::new((x, y), "#FF5733", 0));
                }
                if value > 0.4 {
                    data.insert(Tile::new((x, y), "#FFC300", 1));
                }
                if value > 0.6 {
                    data.insert(Tile::new((x, y), "#DAF7A6", 2));
                }
                if value > 0.8 {
                    data.insert(Tile::new((x, y), "#C70039", 3));
                }
            }
        }
        Chunk {
            width,
            height,
            data,
        }
    }
}
