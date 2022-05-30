use bevy::prelude::Component;

#[derive(Component)]
pub enum TileType{
    Stone,
    Grass,
    Water,
    Dirt,
    Mud,
    Sand,
}

impl From<u16> for TileType{
    fn from(input: u16) -> Self {
        use TileType::*;
        match input{
            0 => Stone,
            1 => Grass,
            2 => Water,
            3 => Dirt,
            4 => Mud,
            5 => Sand,
            _ => unreachable!()
        }
    }
}