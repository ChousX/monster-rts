use bevy::{prelude::*, sprite};
use crate::share::AssetChecker;


//after loading
pub fn init_texture_atlas(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_handles: ResMut<LoadingTextureAtlasHandles>
){
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in texture_handles.0.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();
    
    // think about changing this form a vec to a hash map for ese of use
    //Note: The order must be the same as there id
    let tile_list = vec!["stone", "water", "dirt"];
    let mut tile_handles = Vec::with_capacity(tile_list.len());
    for t in tile_list{
        let path = format!(r"textures\temp\{}.png", t);
        let handle = asset_server.get_handle(path);
        let index = texture_atlas.get_texture_index(&handle).unwrap();
        tile_handles.push(index);
    }

    let atlas_handle = texture_atlases.add(texture_atlas);

    let map_texture_atlas_handles = MapTextureAtlasHandles {
        handle: atlas_handle,
        texture: texture_atlas_texture,
        tile_handles,
    };

    commands.insert_resource(map_texture_atlas_handles);
    if cfg!(debug_assertions){println!("Map Texture atlas has been made")}
    
    commands.remove_resource::<LoadingTextureAtlasHandles>();

}
pub struct MapTextureAtlasHandles{
    pub handle: Handle<TextureAtlas>,
    pub texture: Handle<Image>,
    pub tile_handles: Vec<usize>
}

pub struct LoadingTextureAtlasHandles(pub Vec<HandleUntyped>);

pub fn load_atlas_textures(mut commands: Commands, mut asset_checker: ResMut<AssetChecker>, asset_server: Res<AssetServer>) {
    if cfg!(debug_assertions){println!("Started to load the texter atlas")}
    let mut textures = asset_server.load_folder(r"textures\temp").expect("failed to find temp");
    commands.insert_resource(LoadingTextureAtlasHandles(textures.clone()));
    asset_checker.0.append(&mut textures);
}
