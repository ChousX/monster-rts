use bevy::prelude::*;
use crate::share::{GameState, asset_checker_init};
mod cursor;
pub struct HudPlugin;
impl Plugin for HudPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_exit(GameState::Pre).with_system(cursor::load_cursors))
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(cursor::init_game_cursor))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(cursor::init_menu_cursor))
            .add_system_set(SystemSet::on_exit(GameState::GameMenu).with_system(cursor::remove_menu_cursor))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(cursor::move_cursor_mouse))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(cursor::move_cursor_mouse))
            .add_system_set(SystemSet::on_update(GameState::GameMenu).with_system(cursor::move_cursor_mouse))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(cursor::move_cursor_gamepad))
            ;
    }
}