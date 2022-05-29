use bevy::prelude::*;
use crate::controls::KeyBindings;
use crate::camera::CamraMoveEvent;
pub fn keyboard_input_ingest(
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut camera_move_event: EventWriter<CamraMoveEvent>,
){
    {  
        let mut inputs = [false;6];
        //left
        for key in key_bindings.move_left.iter(){
            if keyboard.pressed(*key){
                inputs[0] = true;
                break;
            }
        }
        //right
        for key in key_bindings.move_right.iter(){
            if keyboard.pressed(*key){
                inputs[1] = true;
                break;
            }
        }
        //up
        for key in key_bindings.move_up.iter(){
            if keyboard.pressed(*key){
                inputs[2] = true;
                break;
            }
        }
        //down
        for key in key_bindings.move_down.iter(){
            if keyboard.pressed(*key){
                inputs[3] = true;
                break;
            }
        }
        //zoom in
        for key in key_bindings.zoom_in.iter(){
            if keyboard.pressed(*key){
                inputs[4] = true;
                break;
            }
        }
        //zoom out
        for key in key_bindings.zoom_out.iter(){
            if keyboard.pressed(*key){
                inputs[5] = true;
                break;
            }
        }
        camera_move_event.send(CamraMoveEvent{
            inputs
        });    
    }

}