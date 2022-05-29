use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

mod texture;
use crate::share::{GameState, AssetChecker};

#[derive(Default)]
pub struct MapTexterHandle(pub Handle<Image>);

pub struct MapPlugin;
impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(start_up_map))
            //.add_system_set(SystemSet::on_enter(GameState::MainGame).with_system())
            .add_system(texture::set_texture_filters_to_nearest)
            .add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(pre_start_up));
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
    mut checker:  ResMut<AssetChecker>
){
    let handle: Handle<Image> = asset_server.load(r"textures/iso_color.png");
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
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //making a new entity to store map stuff
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    
    let mut settings = LayerSettings::new(
        map_settings.map_size,
        ChunkSize(16, 32),
        TileSize(64.0, 32.0),
        TextureSize(384.0, 32.0),
    );
    settings.mesh_type = TilemapMeshType::Isometric(IsoType::Staggered);

    let (mut layer, later_entity) = 
        LayerBuilder::<TileBundle>::new(&mut commands, settings.clone(), 0u16, 0u16);
    layer.set_all(
        Tile {
            texture_index: 1,
            ..Default::default()
        }.into()
    );
// Make 2 layers on "top" of the base map.
// for z in 0..5 {
//     let (mut layer_builder, layer_entity) =
//         LayerBuilder::new(&mut commands, settings.clone(), 0u16, z + 1);
//     map.add_layer(&mut commands, z + 1, layer_entity);

//     let mut random = thread_rng();

//     for _ in 0..1000 {
//         let position = TilePos(random.gen_range(0..128), random.gen_range(0..128));
//         // Ignore errors for demo sake.
//         let _ = layer_builder.set_tile(
//             position,
//             TileBundle {
//                 tile: Tile {
//                     texture_index: 0 + z + 1,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         );
//     }

//     map_query.build_layer(&mut commands, layer_builder, texture_handle.0.clone());
//     }
    map_query.build_layer(&mut commands, layer, texture_handle.0.clone());
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-1024.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}