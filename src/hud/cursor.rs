use crate::{
    camera::UICamera,
    controls::RightStickEvent,
    share::{
        paths::{GAME_CURSOR, MENU_CURSOR},
        AssetChecker,
    },
};
use bevy::{prelude::*, transform};
//TODO: fix the cursor being under the buttons

pub struct CursorsHandle {
    game_handle: Handle<Image>,
    menu_handle: Handle<Image>,
}

#[derive(Component)]
pub struct Cursor;

pub fn load_cursors(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut checker: ResMut<AssetChecker>,
) {
    let menu_handle: Handle<Image> = assets_server.load(MENU_CURSOR);
    let game_handle: Handle<Image> = assets_server.load(GAME_CURSOR);
    checker.0.push(menu_handle.clone_untyped());
    checker.0.push(game_handle.clone_untyped());
    commands.insert_resource(CursorsHandle {
        game_handle,
        menu_handle,
    });
}

pub fn init_game_cursor(
    mut commands: Commands,
    cursors: Res<CursorsHandle>,
    ui_camera: Res<UICamera>,
) {
    let child = commands
        .spawn_bundle(SpriteBundle {
            texture: cursors.game_handle.clone(),
            transform: Transform::from_translation(Vec3::splat(0.0)),
            ..Default::default()
        })
        .insert(Cursor {})
        .id();
    commands.entity(ui_camera.0).add_child(child);
}

pub fn init_menu_cursor(
    mut commands: Commands,
    cursors: Res<CursorsHandle>,
    ui_camera: Res<UICamera>,
) {
    let child = commands
        .spawn_bundle(SpriteBundle {
            texture: cursors.menu_handle.clone(),
            transform: Transform::from_translation(Vec3::splat(0.0)),
            ..Default::default()
        })
        .insert(Cursor {})
        .id();
    commands.entity(ui_camera.0).add_child(child);
}

pub fn remove_menu_cursor(mut commands: Commands, mut cursor: Query<(Entity, &Cursor)>) {
    for (entity, _) in cursor.iter() {
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).despawn();
    }
}

pub fn move_cursor_mouse(
    mut cursor_quere: Query<(&Cursor, &mut Transform)>,
    mut cursor_evr: EventReader<CursorMoved>,
    window: Res<Windows>,
) {
    let window = window.get_primary().unwrap();
    let offset = (window.width() * 0.5, window.height() * 0.5);

    for (_, mut transform) in cursor_quere.iter_mut() {
        for last_pos in cursor_evr.iter() {
            transform.translation.x = last_pos.position.x - offset.0;
            transform.translation.y = last_pos.position.y - offset.1;
        }
    }
}

pub fn move_cursor_gamepad(
    mut cursor_quere: Query<(&Cursor, &mut Transform)>,
    mut right_stick: EventReader<RightStickEvent>,
    window: Res<Windows>,
    time: Res<Time>,
) {
    let (width, height) = {
        let window = window.get_primary().unwrap();
        (window.width() * 0.5, window.height() * 0.5)
    };
    for event in right_stick.iter() {
        for (_, mut transform) in cursor_quere.iter_mut() {
            let mut direction = Vec3::ZERO;
            direction.x += event.0.x;
            direction.y += event.0.y;
            let z = transform.translation.z;
            transform.translation += time.delta_seconds() * direction * 500.;
            transform.translation.z = z;

            //preventing cursor form leaving the screen
            if transform.translation.x > width {
                transform.translation.x = width;
            }
            if transform.translation.y > height {
                transform.translation.y = height;
            }
            if transform.translation.x < -width {
                transform.translation.x = -width;
            }
            if transform.translation.y < -height {
                transform.translation.y = -height;
            }
        }
    }
}
