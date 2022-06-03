use super::Mob;
use bevy::prelude::*;
use std::collections::VecDeque;

pub type PerseptionData = Option<()>;
#[derive(Default)]
pub struct MobPerseptionEvent(pub VecDeque<VecDeque<PerseptionData>>);

#[derive(Default, Component)]
pub struct Sight {
    rang: f32,
}

pub fn mob_vision(
    mob: Query<(&Sight, &Transform, Entity), With<Mob>>,
    output: EventWriter<MobPerseptionEvent>,
) {
    for (sight, transform, entity) in mob.iter() {
        let pos = transform.translation;
        //TODO
        //then we need to get the what tile we are on
        //then we need to get every tile with in a range
        //and save that data in PerseptionData
        //lets start off with getting the tiles in a box shape Then work on making it a circle
    }
}

#[derive(Default, Component)]
pub struct Hearing{
    
}

pub fn mob_listening(
    mob: Query<(&Hearing, &Transform, Entity), With<Mob>>,
    output: EventWriter<MobPerseptionEvent>,
){
    for (hearing, transform, entity) in mob.iter() {
        let pos = transform.translation;
    }

}