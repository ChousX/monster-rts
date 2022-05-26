use bevy::prelude::*;

use crate::controls::KeyBindings;
use crate::share::{MapView};


//TODO: look in to a more extendable and elegant selution 
pub fn move_camra_keyboard(
    mut map_view: Query<(&MapView, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
){
    let (map_view, mut transform) = map_view.single_mut();
    let speed = map_view.speed;
    //Modifier keys
    let alpha = {
        let mut out = false;
        for key in key_bindings.alpha.iter(){
            if keyboard.pressed(*key){
                out = true;
                break;
            }
        }
        out
    };
    let beta = {
        let mut out = false;
        for key in key_bindings.beta.iter(){
            if keyboard.pressed(*key){
                out = true;
                break;
            }
        }
        out
    };
    let gamma = {
        let mut out = false;
        for key in key_bindings.gamma.iter(){
            if keyboard.pressed(*key){
                out = true;
                break;
            }
        }
        out
    };
    let delta = {
        let mut out = false;
        for key in key_bindings.delta.iter(){
            if keyboard.pressed(*key){
                out = true;
                break;
            }
        }
        out
    };
    
    //Left
    for key in key_bindings.move_left.iter(){
        if keyboard.pressed(*key){
            transform.translation.x += speed * time.delta_seconds();
        }
    }
    //Right
    for key in key_bindings.move_right.iter(){
        if keyboard.pressed(*key){
            transform.translation.x -= speed * time.delta_seconds();

        }
    }
    //Up
    for key in key_bindings.move_up.iter(){
        if keyboard.pressed(*key){
            transform.translation.y -= speed * time.delta_seconds();

        }
    }
    //Down
    for key in key_bindings.move_down.iter(){
        if keyboard.pressed(*key){
            
            transform.translation.y += speed * time.delta_seconds();
        }
    }
}

//not working....
pub fn zoom_camra(
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut map_view: Query<(&MapView, &mut Transform)>,
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

