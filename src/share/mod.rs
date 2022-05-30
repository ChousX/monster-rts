mod game_state;
mod asset_checker;
mod menu;
pub mod paths;


//mod camera;
pub use menu::*;
pub use game_state::GameState;
pub use asset_checker::{AssetChecker, init as asset_checker_init, check as asset_load_checker};






pub const RESOLUTION: f32 = 1080.0;

use std::ops::{Add, Sub, Mul, Div};

//https://rosettacode.org/wiki/Map_range#Rust
pub fn range_mapping<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T 
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Div<T, Output=T>
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
