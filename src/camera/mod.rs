use bevy::{prelude::*, render::settings};
use rand::prelude::*;
use crate::share::GameState;
mod movment;
pub use movment::CameraMoveEvent;
use crate::map::{CHUNK_SIZE, TILE_SIZE, MyMapSettings};
pub struct UICamera(pub Entity);
pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<CameraMoveEvent>()
            .add_startup_system(init_ui_camera)
            .add_startup_system(add_camera)
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(movment::move_camera).with_system(movment::move_camera_gamepad).label("camera_postion_update"))
            .add_system_set(SystemSet::on_update(GameState::MainGame).after( "camera_postion_update").with_system(camra_bounding))
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(move_camera_to_random_pos).label("rand_move_camera"))
            ;
    }
}

pub fn camra_bounding(
    window: Res<Windows>,
    mut query: Query<&mut Transform, With<Camera>>,
    map_settings: Res<MyMapSettings>
){
    let (bottom, top) = {  
        let window = window.get_primary().unwrap();
        let offset = (window.width() * 0.5, window.height() * 0.5);

        let chunk_size = (
            CHUNK_SIZE.0 as f32 * TILE_SIZE.0 * map_settings.map_size.0 as f32,
            CHUNK_SIZE.1 as f32 * TILE_SIZE.1 * map_settings.map_size.1 as f32 * 0.5
        );

        ((0. + offset.0, -chunk_size.1 + offset.1), (chunk_size.0 - offset.0, 0. ))
    };

    for mut transform in query.iter_mut(){
        //x
        let subject = &mut  transform.translation.x;
        if *subject < bottom.0{
            *subject = bottom.0;
        } else if *subject > top.0{
            *subject = top.0;
        }

        //y
        let subject = &mut transform.translation.y;
        if *subject < bottom.1{
            *subject = bottom.1;
        } else if *subject > top.1{
            *subject = top.1;
        }
    }
}

pub fn init_ui_camera(mut commands: Commands){
    let id = commands.spawn_bundle(UiCameraBundle::default()).id();
    commands.insert_resource(UICamera(id));
}

pub fn add_camera(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn move_camera_to_random_pos(
    window: Res<Windows>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    map_settings: Res<MyMapSettings>,
){
    let (x, y) = {
        let (bottom, top) = {  
            let window = window.get_primary().unwrap();
            let offset = (window.width() * 0.5, window.height() * 0.5);
    
            let chunk_size = (
                CHUNK_SIZE.0 as f32 * TILE_SIZE.0 * map_settings.map_size.0 as f32,
                CHUNK_SIZE.1 as f32 * TILE_SIZE.1 * map_settings.map_size.1 as f32 * 0.5
            );
    
            ((0. + offset.0, -chunk_size.1 + offset.1), (chunk_size.0 - offset.0, 0. ))
        };
        let mut rng = thread_rng();
        let x = rng.gen_range((bottom.0)..(top.0));
        let y = rng.gen_range((bottom.1)..(top.1));
        (x, y)
    };

    for (mut transform, _) in query.iter_mut(){
        transform.translation.x = x;
        transform.translation.y = y;
    }
}