use bevy::prelude::*;
use crate::{
    map::{chunk::Chunk, map_atlas::MapTextureAtlasHandles}, 
    share::range_mapping,
};

pub const TILE_SIZE: (f64, f64) = (1.0, 1.0);
#[derive(Clone, Copy, Debug, Component)]
pub enum Tile{
    Stone = 0,
    Water = 1,
    Dirt = 2,
}

impl Tile{
    pub fn id(&self) -> usize{
        *self as usize
    }

    fn from_id(input: usize) -> Tile{
        use Tile::*;
        match input{
            0 => Stone,
            1 => Water,
            2 => Dirt,
            _ => unreachable!()
        }
    }

    pub fn from(input: f64) -> Tile{
        let val = range_mapping((-1.0, 1.0), (0.0, 2.0), input).ceil() as usize;
        Tile::from_id(val)
    }

    pub fn get_sprite(&self, tile_handles: &Vec<usize>, atlas_handle: &Handle<TextureAtlas>, pos: Vec3) -> SpriteSheetBundle {
        let id = tile_handles[self.id()];
        SpriteSheetBundle { 
            sprite: TextureAtlasSprite::new(id), 
            texture_atlas: atlas_handle.clone(), 
            transform: Transform {
                translation: pos,
                scale: Vec3::new(TILE_SIZE.0 as f32, TILE_SIZE.1 as f32, 1.0),
                ..default()
            }, 
            ..Default::default()
        }
    }
}
