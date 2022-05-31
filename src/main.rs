use bevy::prelude::*;


mod debug;
mod share;
mod mob;
mod map;
mod main_menu;
mod hud;
mod controls;
mod camera;
mod in_game_menu;

use share::GameState;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(GameState::GameLoad).with_system(share::asset_load_checker))
        .add_startup_system(share::asset_checker_init)
        .add_state(GameState::MainMenu)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(controls::ControlsPlugin)
        .add_plugin(hud::HudPlugin)
        .run();
}
