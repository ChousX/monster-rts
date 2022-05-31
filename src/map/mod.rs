use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

mod texture;
mod tile_tpye;
use crate::map::tile_tpye::TileType;
use crate::share::{GameState, AssetChecker, range_mapping};
use noise::{OpenSimplex, NoiseFn, Seedable};

pub const TILE_SIZE: (f32, f32) = (64.0, 32.0);
pub const CHUNK_SIZE: (u32, u32) = (16, 32);



#[derive(Default)]
pub struct MapTexterHandle(pub Handle<Image>);

pub struct MapPlugin;
impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(start_up_map))
            
            .add_system(texture::set_texture_filters_to_nearest)
            .add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(pre_start_up));
    }
}

pub struct MyMapSettings{
    pub map_size: MapSize,
    pub seed: u32,
}

impl Default for MyMapSettings{
    fn default() -> Self {
        Self{
            map_size: MapSize(5, 10),
            seed: 0,
        }
    }
}



pub fn pre_start_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut checker:  ResMut<AssetChecker>
){
    let handle: Handle<Image> = asset_server.load(crate::share::paths::MAP_ISO);
    let un_ref = handle.clone_untyped();
    checker.0.push(un_ref);



    commands.insert_resource(MyMapSettings::default());
    commands.insert_resource(MapTexterHandle(handle));
}

pub fn start_up_map(
    mut commands: Commands, 
    texture_handle: Res<MapTexterHandle>,
    mut map_query: MapQuery,
    map_settings: Res<MyMapSettings>
){
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //making a new entity to store map stuff
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    
    let mut settings = LayerSettings::new(
        map_settings.map_size,
        ChunkSize(CHUNK_SIZE.0, CHUNK_SIZE.1),
        TileSize(TILE_SIZE.0, TILE_SIZE.1),
        TextureSize(384.0, 32.0),
    );
    settings.mesh_type = TilemapMeshType::Isometric(IsoType::Staggered);

    let (mut layer, _layer_entity) = 
        LayerBuilder::<TileBundle>::new(&mut commands, settings.clone(), 0u16, 0u16);

    //so if you dont do this then there are no tiles to iter over
    layer.set_all(Tile{
        texture_index: 0,
        ..Default::default()
    }.into());
    //TODO: Move the tile setting stuff to another function
    let mut noise = OpenSimplex::new();
    noise.set_seed(map_settings.seed);

    let mut x = 0;
    let mut y: u32 = 0;
    layer.for_each_tiles_mut(|entity, tile_bundle|{
        if let Some(tile_bundle) = tile_bundle{
            let val = noise.get([x as f64, y as f64]);
            let index = range_mapping((-1.0, 1.0), (0.0, 5.0), val) as u16;
            
            if let Some(entity) = entity{
                commands.entity(*entity).insert(TileType::from(index));
            }

            tile_bundle.tile.texture_index = index;
        }
        if x > settings.map_size.0 * settings.chunk_size.0{
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    });


    map_query.build_layer(&mut commands, layer, texture_handle.0.clone());
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-1024.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}