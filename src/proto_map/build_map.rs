use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*};
use rand::{thread_rng, Rng};

pub struct MapSettings{
    map_size: MapSize,
    seed: u32,
}
impl Default for MapSettings{
    fn default() -> Self {
        Self { 
            map_size: MapSize(10, 10),
            seed: 0u32
        }
    }
}

pub fn build_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
    
){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    println!("111");
    let texture_handle = asset_server.load(r"textures/iso_color.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let mut map_settings = LayerSettings::new(
        MapSize(2, 2),
        ChunkSize(32, 32),
        TileSize(64.0, 32.0),
        TextureSize(384.0, 32.0),
    );

    map_settings.mesh_type = TilemapMeshType::Isometric(IsoType::Diamond);

    let (mut layer_0, layer_0_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings, 0u16, 0u16);

        map_query.build_layer(
            &mut commands, 
            layer_0, 
            texture_handle.clone());

        for z in 0..5 {
            let (mut layer_builder, layer_entity) =
                LayerBuilder::new(&mut commands, map_settings.clone(), 0u16, z + 1);
            map.add_layer(&mut commands, z + 1, layer_entity);
    
            let mut random = thread_rng();
    
            for _ in 0..1000 {
                let position = TilePos(random.gen_range(0..128), random.gen_range(0..128));
                // Ignore errors for demo sake.
                let _ = layer_builder.set_tile(
                    position,
                    TileBundle {
                        tile: Tile {
                            texture_index: 0 + z + 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                );
            }
    
            map_query.build_layer(
                &mut commands, 
                layer_builder, 
                texture_handle.clone());
        }
    
        // Spawn Map
        // Required in order to use map_query to retrieve layers/tiles.
        commands
            .entity(map_entity)
            .insert(map)
            .insert(Transform::from_xyz(0.0, 1024.0, 0.0))
            .insert(GlobalTransform::default());
}
 