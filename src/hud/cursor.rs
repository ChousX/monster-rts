use bevy::{prelude::*, transform};
use crate::{share::{AssetChecker, paths::CURSOR}, controls::RightStickEvent, camera::UICamera};

pub struct CursorsHandle{
    handle: Handle<Image>
}


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
}

pub fn init_cursor(
    mut commands: Commands,
    cursors: Res<CursorsHandle>,
    ui_camera: Res<UICamera>
){
    let child = commands.spawn_bundle(SpriteBundle{
        texture: cursors.handle.clone(),
        transform: Transform::from_translation(Vec3::splat(0.0)),
        ..Default::default()
    }).insert(Cursor{}).id();
    commands.entity(ui_camera.0).add_child(child);
}

pub fn move_cursor_mouse(
    mut cursor_quere: Query<(&Cursor, &mut Transform)>,
    mut cursor_evr: EventReader<CursorMoved>,
){
    let (_, mut transform) = cursor_quere.single_mut();

    for  last_pos in  cursor_evr.iter(){
        println!("{}|{}", last_pos.position.x, last_pos.position.y);
        transform.translation.x = last_pos.position.x;
        transform.translation.y = last_pos.position.y;
    }
}

pub fn move_cursor_gamepad(
    cursor_quere: Query<(&Cursor, &mut Transform)>,
    mut right_stick: EventReader<RightStickEvent>
){
    //TODO:
}