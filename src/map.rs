use std::{fs::DirEntry, collections::btree_map::Range};

use noise::{NoiseFn, OpenSimplex, Seedable};

pub struct MapBuilder{
    x: Option<u32>,
    y: Option<u32>,
    z: Option<u32>,
    seed: Option<u32>, 
}

impl MapBuilder{
    pub fn new() -> Self{
        Self{
            x: None,
            y: None,
            z: None,
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
        let max_z = self.z.unwrap_or(10);
        let mut map = Vec::with_capacity(max_y as usize/ CHUNK_SIZE.1);
        for chunk_y in 0..(max_y as usize /CHUNK_SIZE.1){
            let mut chunks = Vec::new();// I should pre alocat the size but not now todo!
            for chunk_x in 0..(max_x as usize /CHUNK_SIZE.0){
                let mut chunk = Chunk::new(max_z as usize);
                for y in 0..CHUNK_SIZE.1{
                    for x in 0..CHUNK_SIZE.0{
                        let pre = noise.get([(chunk_x * x) as f64 * TILE_SIZE.0, (chunk_y * y) as f64 * TILE_SIZE.1]);
                        let elevation = aux(pre, max_z as f64) as usize;
                        for z in 0..elevation{
                            chunk.set(x, y, z, Tile::Stone);
                        }
                    }
                }
                chunks.push(chunk)
            }
            map.push(chunks);
        }
    } 

    
}

pub const TILE_SIZE: (f64, f64) = (100.0, 100.0);
pub const TILE_LEN: usize = 5;
#[derive(Clone, Copy, Debug)]
pub enum Tile{
    Air = 0,
    Dirt = 1,
    Water = 2,
    Grass = 3,
    Stone = 4,
}

impl Default for Tile{
    fn default() -> Self {
        Self::Air
    }
}

impl Tile{
    pub fn from(input: f64) -> Self{
        // trying to convert the range from -1..1 to 0..n n= number of tile vareations
        let subject: u32 = aux(input, TILE_LEN as f64) as u32;
        match subject as usize{
            0 => Self::Air,
            1 => Self::Dirt,
            2 => Self::Water,
            3 => Self::Grass,
            4 => Self::Stone,
            _ => unreachable!()
        }
    }
}

fn aux(input: f64, range: f64) -> f64{
    let subject = ((input  +  1.0) * range) / 2.0;
    subject
}
pub const CHUNK_SIZE: (usize, usize) = (5, 5);
pub struct Chunk{
    arena: Vec<[[Tile; CHUNK_SIZE.0]; CHUNK_SIZE.1]>,
}

impl Default for Chunk{
    fn default() -> Self {
        Self{
            arena: Vec::new()
        }
    }
}

impl Chunk{
    pub fn new(height: usize) -> Self{
        let mut output = Vec::with_capacity(height);
        for _ in 0..height{
            output.push([[Tile::Air; CHUNK_SIZE.0]; CHUNK_SIZE.1])
        }

        Self{
            arena: output
        }
    }
    //I am not checking the bounds on this for the speed
    pub fn get(&self, x: usize, y: usize, z: usize) -> Tile{
        (self.arena[z])[y][x]
    }
    
    //same with this one
    pub fn set(&mut self, x: usize, y: usize, z: usize, new: Tile){
        (self.arena[z])[y][x] = new;
    }
}