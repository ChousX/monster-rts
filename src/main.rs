use bevy::prelude::*;


mod debug;
mod share;
mod mob;
mod map;
mod main_menu;
mod hud;
mod controls;

use share::GameState;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(ui_cam_init)
        .add_startup_system(share::asset_checker_init)
        .add_system_set(SystemSet::on_enter(GameState::GameLoad).with_system(share::asset_load_checker))
        .add_state(GameState::MainMenu)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(controls::ControlsPlugin)
        .run();
}

fn ui_cam_init(mut commands: Commands){
    commands.spawn_bundle(UiCameraBundle::default());
}