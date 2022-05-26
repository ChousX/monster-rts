use bevy::prelude::*;
mod keybindings;
mod camrea_movment;

use crate::share::{GameState, add_camera_speed};

pub use keybindings::{KeyBindings, init_key_bindings};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(camrea_movment::move_camra_keyboard))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(camrea_movment::zoom_camra))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(add_camera_speed))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(init_key_bindings));
        
    }
}
