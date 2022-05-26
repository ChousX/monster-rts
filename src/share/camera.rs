use bevy::{prelude::*};

#[derive(Component, Clone, Copy)]
pub struct MainMapView;   
//adding a camera 
pub fn add_camera(mut commands: Commands){
    let mut camera = OrthographicCameraBundle::new_2d();
    //first normilizing the cordinites 
    /*
    use crate::share::RESOLUTION;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = render::camera::ScalingMode::None;
    */
    commands
    .spawn_bundle(camera)
    .insert(MainMapView);
    if cfg!(debug_assertions){println!("Camrea Added")}
}

pub struct MapCamraSpeed(pub f32);
pub fn add_camera_speed(mut commands: Commands){
    commands.insert_resource(MapCamraSpeed(1.5));
}