use bevy::{prelude::*, render::camera::ScalingMode};
use rand::prelude::*;
use crate::share::{GameState, RESOLUTION, asset_load_checker, add_camera};

use crate::map::tile::TileSize;

mod map_atlas;
mod chunk;
mod tile;

pub struct MapPlugin;
impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::GameLoad).with_system(asset_load_checker))
            .add_system_set(SystemSet::on_exit(GameState::GameLoad).with_system(map_atlas::init_texture_atlas))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(map_atlas::load_atlas_textures))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(init))
            .add_system_set(SystemSet::on_exit(GameState::GameLoad).with_system(add_camera))
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(build_map))
            .add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(init_map_settings))
            ;
    }
}
pub struct Seed(u32);

pub fn init(mut commands: Commands){
    commands.insert_resource(Seed(thread_rng().gen()));
    commands.init_resource::<tile::TileSize>();
}



pub struct MapSettings{
    pub size: (usize, usize)
}

pub fn init_map_settings(mut commands: Commands){
    commands.insert_resource(MapSettings{
        size: (2, 2)
    });
    if cfg!(debug_assertions){println!("MapSettings: Inited")}
}

pub fn build_map(
    mut commands: Commands,  
    settings: Res<MapSettings>, 
    seed: Res<Seed>, 
    atlas_handles: Res<map_atlas::MapTextureAtlasHandles>, 
    tile_size: Res<TileSize>
){
    for chunk_y in 0..settings.size.1{
        for chunk_X in 0..settings.size.0{
            chunk::make_chunk(
                &mut commands, 
                (chunk_X, chunk_y), 
                &atlas_handles, 
                seed.0, 
                tile_size.0
            );
        }
    }
    if cfg!(debug_assertions){println!("Map Built")}
}