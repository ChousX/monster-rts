use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
mod camera;
mod controls;
mod debug;
mod hud;
mod in_game_menu;
mod main_menu;
mod map;
mod mob;
mod share;

use share::GameState;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: share::RESOLUTION.0,
            height: share::RESOLUTION.1,
            cursor_locked: false,
            cursor_visible: false,
            resizable: false,
            present_mode: PresentMode::Fifo,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::on_enter(GameState::GameLoad).with_system(share::asset_load_checker),
        )
        .add_system_set(SystemSet::on_enter(GameState::Pre).with_system(share::asset_checker_init))
        .add_system_set(SystemSet::on_update(GameState::Pre).with_system(move_to_menu))
        .add_startup_system(share::asset_checker_init)
        .add_state(GameState::Pre)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(controls::ControlsPlugin)
        .add_plugin(hud::HudPlugin)
        .add_plugin(mob::MobPlugin)
        .run();
}

fn move_to_menu(mut state: ResMut<State<GameState>>) {
    state.set(GameState::MainMenu);
}
