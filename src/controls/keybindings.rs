use bevy::prelude::*;

type Key = Vec<KeyCode>;
pub struct KeyBindings{
    //Camra
    //movment
    pub move_left: Key,
    pub move_right: Key,

    pub move_up: Key,
    pub move_down: Key,
    
    //zoom
    pub zoom_in: Key,
    pub zoom_out: Key,
}

impl Default for KeyBindings{
    fn default() -> Self {
        use KeyCode::*;
        Self { 
            move_left: vec![A, Left], 
            move_right: vec![D, Right], 
            move_up: vec![W, Up], 
            move_down: vec![S, Down],
            zoom_in: vec![], 
            zoom_out: vec![] 
        }
    }
}

pub fn init_key_bindings(mut commands: Commands){
    commands.init_resource::<KeyBindings>();
}
