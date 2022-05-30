use bevy::prelude::*;
use crate::share::GameState;
mod movment;
pub use movment::CameraMoveEvent;

pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<CameraMoveEvent>()
            .add_startup_system(init_ui_camera)
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(add_camera).label("camra"))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(movment::move_camera).after("camra"))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(movment::move_camera_gamepad))
            ;
    }
}

pub fn init_ui_camera(mut commands: Commands){
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn add_camera(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}