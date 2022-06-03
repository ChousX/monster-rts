use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod texture;
mod tile_tpye;
mod seed;
use crate::map::tile_tpye::TileType;
use crate::share::{range_mapping, AssetChecker, GameState};
use rand::{prelude::*, Rng, SeedableRng};


pub const TILE_SIZE: (f32, f32) = (64.0, 32.0);
pub const CHUNK_SIZE: (u32, u32) = (16, 32);

#[derive(Default)]
pub struct MapTexterHandle(pub Handle<Image>);
//TODO Get things running in the right order
pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
        .add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(pre_start_up).label("map init"))
            .add_system_set(SystemSet::on_enter(GameState::GameLoad).with_system(start_up_map).label("map build"))
            .add_system(texture::set_texture_filters_to_nearest)
            
            ;
    }
}

pub type MyRNG = rand_pcg::Mcg128Xsl64;


pub struct MyMapSettings {
    pub map_size: MapSize,
    pub rivers: Option<(usize, usize)>,
    pub seed: u64,
}

impl Default for MyMapSettings {
    fn default() -> Self {
        Self {
            map_size: MapSize(5, 5),
            rivers: Some((1, 3)),
            seed: 0,
        }
    }
}



pub fn pre_start_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut checker: ResMut<AssetChecker>,
) {
    let handle: Handle<Image> = asset_server.load(crate::share::paths::MAP_ISO);
    let un_ref = handle.clone_untyped();
    checker.0.push(un_ref);
    let settings = MyMapSettings::default();
    let rng = rand_pcg::Pcg64Mcg::seed_from_u64(settings.seed);

    commands.insert_resource(rng);
    commands.insert_resource(settings);
    commands.insert_resource(MapTexterHandle(handle));
}

pub fn start_up_map(
    mut commands: Commands,
    texture_handle: Res<MapTexterHandle>,
    mut map_query: MapQuery,
    map_settings: Res<MyMapSettings>,
) {
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
    layer.set_all(
        Tile {
            texture_index: 0,
            ..Default::default()
        }
        .into(),
    );
    //TODO: Move the tile setting stuff to another function
    // let mut noise = OpenSimplex::new();
    // noise.set_seed(map_settings.seed);

    // let mut x = 0;
    // let mut y: u32 = 0;
    // layer.for_each_tiles_mut(|entity, tile_bundle| {
    //     if let Some(tile_bundle) = tile_bundle {
    //         let val = noise.get([x as f64, y as f64]);
    //         let index = range_mapping((-1.0, 1.0), (0.0, 5.0), val) as u16;

    //         if let Some(entity) = entity {
    //             commands.entity(*entity).insert(TileType::from(index));
    //         }

    //         tile_bundle.tile.texture_index = index;
    //     }
    //     if x > settings.map_size.0 * settings.chunk_size.0 {
    //         x = 0;
    //         y += 1;
    //     } else {
    //         x += 1;
    //     }
    // });

    map_query.build_layer(&mut commands, layer, texture_handle.0.clone());
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}


pub fn add_rivers(
    mut commands: Commands,
    mut map: MapQuery,
    map_settings: Res<MyMapSettings>,
    mut rng: ResMut<MyRNG>
){
    use TileType::*;


    if let Some((min, max)) = map_settings.rivers{
        let map_size = {
            let (_, layer) = map.get_layer(0, 0).unwrap();
             layer.get_layer_size_in_tiles()
        };
        for _ in 0..rng.gen_range(min..max){
            let p1 = (
                rng.gen_range(0..map_size.0),
                rng.gen_range(0..map_size.1),
            );
            let p2 = (
                rng.gen_range(0..map_size.0),
                rng.gen_range(0..map_size.1),
            );
            
            for y in p1.1..p2.1{
                for x in p1.0..p2.0{       
                    map.set_tile(&mut commands, TilePos(x, y), Tile { texture_index: Water as u16, ..Default::default()}, 0, 0);
                }
            }

        }
    }
}