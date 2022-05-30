use bevy::prelude::*;
use crate::share::{AssetChecker, paths::CURSOR};

pub struct CursorsHandle{
    handle: Handle<Image>
}
pub struct CursorMouseBlock(bool);

#[derive(Component)]
pub struct Cursor;

pub fn load_cursors(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut checker: ResMut<AssetChecker>
){
    let handle: Handle<Image> = assets_server.load(CURSOR);
    checker.0.push(handle.clone_untyped());
    commands.insert_resource(CursorsHandle{handle});
    commands.insert_resource(CursorMouseBlock(false));
}

pub fn init_cursor(
    mut commands: Commands,
    cursors: Res<CursorsHandle>,
){
    //TODO
}

pub fn move_cursor_mouse(
    cursor_quere: Query<(&Cursor, &mut Transform)>,
    mut mouse_block: ResMut<CursorMouseBlock>,
){
    //TODO:
}

pub fn move_cursor_gamepad(
    cursor_quere: Query<(&Cursor, &mut Transform)>,
    mut mouse_block: ResMut<CursorMouseBlock>,
){
    //TODO:
}