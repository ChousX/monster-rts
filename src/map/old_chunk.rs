use bevy::prelude::*;
use crate::map::{
    tile::{Tile, TILE_SIZE}, 
    Seed,
    map_atlas::MapTextureAtlasHandles
};
use noise::{OpenSimplex, NoiseFn, Seedable};

pub const CHUNK_SIZE: (usize, usize) = (10, 10);
#[derive(Component)]
pub struct Chunk(pub [[Entity; CHUNK_SIZE.0]; CHUNK_SIZE.1]);

#[derive(Component)]
pub struct ChunkPos(f64, f64);

impl Chunk {
    pub fn new(commands: &mut Commands,
        chunk_pos: (usize, usize), 
        seed: u32, 
        atlas_handles: &Res<MapTextureAtlasHandles>
    ) -> Self{
        let zero_zero: (f64, f64) = (
            (chunk_pos.0 * CHUNK_SIZE.0) as f64 * TILE_SIZE.0,
            (chunk_pos.1 * CHUNK_SIZE.1) as f64 * TILE_SIZE.1

        );
        //if any thing goes wrong i am checking this 
        let mut chunk: [[Entity; CHUNK_SIZE.0]; CHUNK_SIZE.1] = unsafe {
            let mut arr: [[Entity; CHUNK_SIZE.0]; CHUNK_SIZE.1] = std::mem::uninitialized();
            let mut noise = OpenSimplex::new();
            noise.set_seed(seed);
            for (y, row) in &mut arr[..].iter_mut().enumerate() {
                let y_pos: f64 = y as f64 * TILE_SIZE.1 + zero_zero.1;
                for (x, entry) in &mut row[..].iter_mut().enumerate(){
                    let x_pos = x as f64 * TILE_SIZE.0 + zero_zero.0;
                    let val = noise.get([
                        x_pos,
                        y_pos
                    ]);
                    let tile = Tile::from(val);
                    let sprite = tile.get_sprite(
                        &atlas_handles.tile_handles,
                        &atlas_handles.handle,
                        Vec3::new(x_pos as f32 , y_pos as f32, 1.0)
                    );
                    let entity = commands
                        .spawn_bundle(sprite)
                        .insert(tile)
                        .id();
                    std::ptr::write(entry, entity);
                }
            }
            arr
        };

        Self(chunk)
    }
}



