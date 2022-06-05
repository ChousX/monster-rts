use bevy::prelude::Component;

#[derive(Component, Copy, Clone)]
pub enum TileType {
    Grass,
    Water,
    Swamp,
    Lava,
    Stone,
    Dirt,
}

impl From<u16> for TileType {
    fn from(input: u16) -> Self {
        use TileType::*;
        match input {
            0 => Grass,
            1 => Water,
            2 => Swamp,
            3 => Lava,
            4 => Stone,
            5 => Dirt,
            _ => unreachable!(),
        }
    }
}
