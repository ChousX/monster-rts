use bevy::prelude::*;
mod gamepad_bindings;
mod gamepad_settings;
mod keybindings;
mod main_game_input;
//mod camrea_movment;

use crate::share::GameState;

pub use keybindings::{init_key_bindings, KeyBindings};

pub use crate::controls::main_game_input::{LeftStickEvent, RightStickEvent};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RightStickEvent>()
            .add_event::<LeftStickEvent>()
            .add_startup_system(init_key_bindings)
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(main_game_input::keyboard_input_ingest),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(main_game_input::gamepad_connections),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(main_game_input::gamepad_input),
            );
    }
}
