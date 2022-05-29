use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::share::{GameState, AssetChecker};

pub struct MapTexterHandle(pub Handle<Image>);

pub struct MapPlugin;
impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(pre_start_up)
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(start_up_map));
    }
}

pub struct MyMapSettings{
    pub map_size: MapSize
}

impl Default for MyMapSettings{
    fn default() -> Self {
        Self{
            map_size: MapSize(2, 2)
        }
    }
}

pub fn pre_start_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    checker: Res<AssetChecker>
){
    let handle: Handle<Image> = asset_server.load(r"textures/iso_color.png");

    checker.0.push(handle.clone_untyped());



    commands.insert_resource(MyMapSettings::default());
    commands.insert_resource(handle);
}

pub fn start_up_map(
    mut commands: Commands, 
    texture_handle: Res<MapTexterHandle>,
    mut map_query: MapQuery,
    map_settings: Res<MyMapSettings>
){
    //making a new entity to store map stuff
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    
    let settings = LayerSettings::new(
        map_settings.map_size,
        ChunkSize(16, 32),
        TileSize(64.0, 32.0),
        TextureSize(384.0, 32.0),
    );

}