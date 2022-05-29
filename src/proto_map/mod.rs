use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tilemap::TilemapPlugin;
use rand::prelude::*;
use crate::share::{GameState, RESOLUTION, asset_load_checker};

mod build_map;
mod texture;
pub struct MapPlugin;
impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            //.add_system_set(SystemSet::on_exit(GameState::GameLoad).with_system(add_camera))
            
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(build_map::build_map))
            .add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(init_map_settings))
            .add_startup_system(texture::set_texture_filters_to_nearest)
            ;
    }
}


pub struct MapSettings{
    pub size: (usize, usize)
}

pub fn init_map_settings(mut commands: Commands){
    commands.init_resource::<build_map::MapSettings>();
    if cfg!(debug_assertions){println!("MapSettings: Inited")}
}


