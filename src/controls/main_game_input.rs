use bevy::prelude::*;
use bevy::app::AppExit;

use crate::controls::KeyBindings;
use crate::camera::CamraMoveEvent;
pub fn keyboard_input_ingest(
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut camera_move_event: EventWriter<CamraMoveEvent>,
    mut exit: EventWriter<AppExit>,
){
    //camera
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

    for key in key_bindings.escape.iter(){
        if keyboard.pressed(*key){
            //for now idk
            exit.send(AppExit);
        }
    }

}

pub struct GamepadHolder(Gamepad);

pub fn gamepad_connections(
    mut commands: Commands,
    gamepad: Option<Res<GamepadHolder>>,
    mut gamepad_evr: EventReader<GamepadEvent>
){
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                if gamepad.is_none() {
                    commands.insert_resource(GamepadHolder(*id));
                }
            },
            GamepadEventType::Disconnected => {
                if let Some(GamepadHolder(old_id)) = gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<GamepadHolder>();
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    gamepad: Option<Res<GamepadHolder>>,
){
    let gamepad = if let Some(gp) = gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate axis for X and Y

    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
    let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)){
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);

        if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
            // do something
        }
    }

    let axis_rx = GamepadAxis(gamepad, GamepadAxisType::RightStickX);
    let axis_ry = GamepadAxis(gamepad, GamepadAxisType::RightStickY);

    if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)){
        // combine X and Y into one vector
        let right_stick_pos = Vec2::new(x, y);

        if right_stick_pos.length() > 0.9 && right_stick_pos.y > 0.5 {
            // do something
        }
    }

    let jump_button = GamepadButton(gamepad, GamepadButtonType::South);
    let heal_button = GamepadButton(gamepad, GamepadButtonType::East);

    if buttons.just_pressed(jump_button) {
        // button just pressed: make the player jump
    }

}

/*
fn gamepad_input_events(
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        if id.0 != gamepad.0 {
            // event not from our gamepad
            continue;
        }

        use GamepadEventType::{AxisChanged, ButtonChanged};

        match kind {
            AxisChanged(GamepadAxisType::RightStickX, x) => {
                // Right Stick moved (X)
            }
            AxisChanged(GamepadAxisType::RightStickY, y) => {
                // Right Stick moved (Y)
            }
            ButtonChanged(GamepadButtonType::DPadDown, val) => {
                // buttons are also reported as analog, so use a threshold
                if *val > 0.5 {
                    // button pressed
                }
            }
            _ => {} // don't care about other inputs
        }
    }
}
 */