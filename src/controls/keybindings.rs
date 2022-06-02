use bevy::prelude::*;

type Key = Vec<KeyCode>;
pub struct KeyBindings {
    //Modifers
    pub alpha: Key,
    pub beta: Key,
    pub gamma: Key,
    pub delta: Key,
    //Camra
    //movment
    pub move_left: Key,
    pub move_right: Key,

    pub move_up: Key,
    pub move_down: Key,

    //zoom
    pub zoom_in: Key,
    pub zoom_out: Key,

    pub escape: Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        use KeyCode::*;
        Self {
            //Modifers
            alpha: vec![RShift, LShift],
            beta: vec![RAlt, LAlt],
            gamma: vec![],
            delta: vec![],
            //Camra
            move_left: vec![A, Left],
            move_right: vec![D, Right],
            move_up: vec![W, Up],
            move_down: vec![S, Down],
            zoom_in: vec![K],
            zoom_out: vec![L],

            escape: vec![Escape],
        }
    }
}

pub fn init_key_bindings(mut commands: Commands) {
    commands.init_resource::<KeyBindings>();
}
