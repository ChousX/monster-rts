use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct Health{
    pub current: f32,
    pub max: f32,
    pub recovery: f32,
}