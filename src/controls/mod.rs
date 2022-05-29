use bevy::prelude::*;
mod keybindings;
mod input;
//mod camrea_movment;

use crate::share::GameState;

pub use keybindings::{KeyBindings, init_key_bindings};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_key_bindings)
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(input::keyboard_input_ingest))
            
            ;
    }
}
