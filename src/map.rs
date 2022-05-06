use std::fs::DirEntry;

use noise::{NoiseFn, OpenSimplex, Seedable};

pub struct MapBuilder{
    x: Option<u32>,
    y: Option<u32>,
    seed: Option<u32>, 
}

impl MapBuilder{
    pub fn new() -> Self{
        Self{
            x: None,
            y: None,
            seed: None,
        }
    }

    pub fn x(&mut self, max: u32) -> &mut Self{
        self.x = Some(max);
        self
    }

    pub fn y(&mut self, max: u32) -> &mut Self{
        self.y = Some(max);
        self
    }

    pub fn seed(&mut self, input: u32) -> &mut Self{
        self.seed = Some(input);
        self
    }

    pub fn build(&self){
        let mut noise = OpenSimplex::new();
        if let Some(seed) = self.seed{
            noise.set_seed(seed);
        }

        //I want None to indecate infent but that hole thing is a todo!
        //also I am going to assum for now that x and y are divisable by the chunk size
        let max_x = self.x.unwrap_or(10);
        let max_y = self.y.unwrap_or(10);
        let mut chunks = Vec::new();// I should pre alocat the size but not now todo!
        for chunk_y in 0..(max_y as usize /CHUNK_SIZE.1){
            for chunk_x in 0..(max_x as usize /CHUNK_SIZE.0){
                let mut chunk = Chunk::default();
                for y in 0..CHUNK_SIZE.1{
                    for x in 0..CHUNK_SIZE.0{
                        let pre_tile = noise.get([(chunk_x * x) as f64 * TILE_SIZE.0, (chunk_y * y) as f64 * TILE_SIZE.1]);
                        chunk.set(x, y, Tile::from(pre_tile));
                    }
                }
                chunks.push(chunk)
            }
        }


    } 
}

pub const TILE_SIZE: (f64, f64) = (100.0, 100.0);
pub const TILE_LEN: usize = 4;
#[derive(Clone, Copy, Debug)]
pub enum Tile{
    Stone = 0,
    Dirt = 1,
    Water = 2,
    Grass = 3,
}

impl Default for Tile{
    fn default() -> Self {
        Self::Stone
    }
}

impl Tile{
    pub fn from(input: f64) -> Self{
        // trying to convert the range from -1..1 to 0..n n= number of tile vareations
        const OLD_RANGE: f64 = 2.0;
        const NEW_RANGE: f64 = TILE_LEN as f64; 
        let subject = ((input  +  1.0) * NEW_RANGE) / OLD_RANGE;
        match subject as usize{
            0 => Self::Stone,
            1 => Self::Dirt,
            2 => Self::Water,
            3 => Self::Grass,
            _ => unreachable!()
        }
    }
}

pub const CHUNK_SIZE: (usize, usize) = (5, 5);
pub struct Chunk{
    arena: [[Tile; CHUNK_SIZE.0]; CHUNK_SIZE.1],
}

impl Default for Chunk{
    fn default() -> Self {
        Self{
            arena: [[Tile::default(); CHUNK_SIZE.0]; CHUNK_SIZE.1]
        }
    }
}

impl Chunk{
    //I am not checking the bounds on this for the speed
    pub fn get(&self, x: usize, y: usize) -> Tile{
        self.arena[y][x]
    }
    
    //same with this one
    pub fn set(&mut self, x: usize, y: usize, new: Tile){
        self.arena[y][x] = new;
    }
}