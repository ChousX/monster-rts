use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct Health{
    pub current: f32,
    pub max: f32,
    pub recovery: f32,
}


#[derive(Component)]
pub struct Attribute{
    pub endurance: u16, //regeneration 
    pub perception: u16, 
    pub aphinity: u16, //Magic Power
    pub vigor: u16, //Health
    pub cognition: u16,
    pub strength: u16, 
    pub grace: u16
}
