mod asset_checker;
mod game_state;
mod menu;
pub mod paths;

//mod camera;
pub use asset_checker::{check as asset_load_checker, init as asset_checker_init, AssetChecker};
pub use game_state::GameState;
pub use menu::*;
pub const RESOLUTION: (f32, f32) = (1920.0, 1080.0);

use std::ops::{Add, Div, Mul, Sub};
//https://rosettacode.org/wiki/Map_range#Rust
pub fn range_mapping<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
