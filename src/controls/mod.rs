use bevy::prelude::*;
mod keybindings;
mod main_game_input;
//mod camrea_movment;

use crate::share::GameState;

pub use keybindings::{KeyBindings, init_key_bindings};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_key_bindings)
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(main_game_input::keyboard_input_ingest))
            
            ;
    }
}
