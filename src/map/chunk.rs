use bevy::{prelude::*, transform};
use noise::{OpenSimplex, NoiseFn, Seedable};

use crate::map::{
    tile::{Tile, TileSize}, 
    Seed,
    map_atlas::MapTextureAtlasHandles
};


use super::tile;

pub const CHUNK_SIZE: (usize, usize) = (10, 10);

#[derive(Component)]
pub struct Chunk;
pub fn make_chunk(
    commands: &mut Commands,
    chunk_pos: (usize, usize),
    atlas_handles: &Res<MapTextureAtlasHandles>,
    seed: u32,
    tile_size: f32
){
    let transform = Transform::from_xyz(
        chunk_pos.0 as f32 * tile_size * CHUNK_SIZE.0 as f32,
        chunk_pos.1 as f32 * tile_size * CHUNK_SIZE.1 as f32,
        1.0
    );
    
    let chunk = commands
        .spawn()
        .insert_bundle(TransformBundle::from_transform(transform))
        .insert(Chunk)
        .id();

    let mut noise = OpenSimplex::new();
    noise.set_seed(seed);

    for y in 0..CHUNK_SIZE.1{
        let f32_y = y as f32 * tile_size;
        for x in 0..CHUNK_SIZE.0{
            let f32_x = x as f32 * tile_size;
            let val = noise.get([f32_x as f64, f32_y as f64]);
            let tile = Tile::from(val);
            let sprite = tile.get_sprite(
                &atlas_handles.tile_handles, 
                &atlas_handles.handle, 
                Vec3::new(f32_x, f32_y, 1.0), 
                tile_size);

            let tile_id = commands
                .spawn_bundle(sprite)
                .insert(tile)
                .id(); 
            commands.entity(chunk).push_children(&[tile_id]);
        }
    }
}