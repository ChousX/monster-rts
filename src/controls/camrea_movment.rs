use bevy::prelude::*;

use crate::controls::KeyBindings;
use crate::share::{MainMapView, MapCamraSpeed};



pub fn move_camra_keyboard(
    mut map_view: Query<(&MainMapView, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    speed: Res<MapCamraSpeed>
){
    let (_, mut transform) = map_view.single_mut();
    //Left
    for key in key_bindings.move_left.iter(){
        if keyboard.pressed(*key){
            transform.translation.x += speed.0;

        }
    }
    //Right
    for key in key_bindings.move_right.iter(){
        if keyboard.pressed(*key){
            transform.translation.x -= speed.0;

        }
    }
    //Up
    for key in key_bindings.move_up.iter(){
        if keyboard.pressed(*key){
            transform.translation.y -= speed.0;

        }
    }
    //Down
    for key in key_bindings.move_down.iter(){
        if keyboard.pressed(*key){
            
            transform.translation.y += speed.0;
        }
    }
}

//not working....
pub fn zoom_camra(
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut map_view: Query<(&MainMapView, &mut Transform)>,
){
    let (_, mut transform) = map_view.single_mut();

    for key in key_bindings.zoom_in.iter(){
        if keyboard.pressed(*key){
            transform.translation.z += 0.5;
        }
    }

    for key in key_bindings.zoom_out.iter(){
        if keyboard.pressed(*key){
            transform.translation.z -= 0.5;
        }
    }
}

