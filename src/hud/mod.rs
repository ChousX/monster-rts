use bevy::prelude::*;
use crate::share::GameState;
mod cursor;
pub struct HudPlugin;
impl Plugin for HudPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(cursor::load_cursors)
            .add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(cursor::init_cursor))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(cursor::move_cursor_mouse))
            .add_system_set(SystemSet::on_update(GameState::MainGame).with_system(cursor::move_cursor_gamepad))
            ;
    }
}