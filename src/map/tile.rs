use bevy::prelude::*;
use crate::{
    map::{chunk::Chunk, map_atlas::MapTextureAtlasHandles}, 
    share::range_mapping,
};

pub struct TileSize(pub f32);
impl Default for TileSize{
    fn default() -> Self {
        Self(5.0)
    }
}

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

    //TODO: There lurks an error in this one | not 100% sure that this is mapping the ranges right idk
    pub fn from(input: f64) -> Tile{
        let val = range_mapping((-1.0, 1.0), (0.0, 2.0), input).ceil() as usize;
        Tile::from_id(val)
    }
    //TODO: There lurks an error in this one | I think the sizing is funky not sure
    pub fn get_sprite(&self,
        tile_handles: &Vec<usize>, 
        atlas_handle: &Handle<TextureAtlas>, 
        pos: Vec3,
        tile_size: f32
    ) -> SpriteSheetBundle {
        let id = tile_handles[self.id()];
        SpriteSheetBundle { 
            sprite: TextureAtlasSprite::new(id), 
            texture_atlas: atlas_handle.clone(), 
            transform: Transform {
                translation: pos,
                scale: Vec3::new(tile_size, tile_size, 1.0),
                ..default()
            }, 
            ..Default::default()
        }
    }
}
