use noise::{NoiseFn, Perlin};

use super::*;

const NOISE_SCALE: f64 = 12.5;

pub struct SimplePerlinLevelGenerator;
pub struct TexturedPerlinLevelGenerator;

impl LevelGenerator for SimplePerlinLevelGenerator {
    fn generate_chunk(seed: u32, (start_x, start_y): (i32, i32)) -> Chunk {
        let mut data = Vec::<Box<dyn Tile>>::new();
        let perlin = Perlin::new(seed);
        let (end_x, end_y) = (start_x + CHUNK_WIDTH as i32, start_y + CHUNK_HEIGHT as i32);
        for x in start_x - 1..end_x + 1 {
            for y in start_y - 1..end_y + 1 {
                let value = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
                if value > 0.2 {
                    data.push(Box::new(ColorTile::new((x, y), "#FF5733", 0)));
                }
                if value > 0.4 {
                    data.push(Box::new(ColorTile::new((x, y), "#FFC300", 1)));
                }
                if value > 0.6 {
                    data.push(Box::new(ColorTile::new((x, y), "#DAF7A6", 2)));
                }
                if value > 0.8 {
                    data.push(Box::new(ColorTile::new((x, y), "#C70039", 3)));
                }
            }
        }
        Chunk {
            ch_pos: (start_x / CHUNK_WIDTH as i32, start_y / CHUNK_HEIGHT as i32),
            width: CHUNK_WIDTH as u32,
            height: CHUNK_HEIGHT as u32,
            data,
        }
    }
}

impl LevelGenerator for TexturedPerlinLevelGenerator {
    fn generate_chunk(seed: u32, (start_x, start_y): (i32, i32)) -> Chunk {
        let mut data = Vec::<Box<dyn Tile>>::new();
        let perlin = Perlin::new(seed);
        let (end_x, end_y) = (start_x + CHUNK_WIDTH as i32, start_y + CHUNK_HEIGHT as i32);
        for x in start_x - 1..end_x + 1 {
            for y in start_y - 1..end_y + 1 {
                let value = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
                if value > 0.2 {
                    data.push(Box::new(TexturedTile::new((x, y), 0, 0)));
                }
                if value > 0.4 {
                    data.push(Box::new(TexturedTile::new((x, y), 0, 1)));
                }
                if value > 0.6 {
                    data.push(Box::new(TexturedTile::new((x, y), 0, 2)));
                }
                if value > 0.8 {
                    data.push(Box::new(TexturedTile::new((x, y), 0, 3)));
                }
            }
        }
        Chunk {
            ch_pos: (start_x / CHUNK_WIDTH as i32, start_y / CHUNK_HEIGHT as i32),
            width: CHUNK_WIDTH as u32,
            height: CHUNK_HEIGHT as u32,
            data,
        }
    }
}
