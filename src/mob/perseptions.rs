use bevy::{prelude::*, reflect::erased_serde::private::serde::__private::de};
use std::collections::VecDeque;
use super::{Mob};

pub type PerseptionData = Option<()>;
#[derive(Default)]
pub struct MobPerseptionEvent(pub VecDeque<VecDeque<PerseptionData>>);

#[derive(Default, Component)]
pub struct Sight{
    rang: f32
}

pub fn vision(
    mob: Query<(&Sight, &Transform, Entity), With<Mob>>,
    output: EventWriter<MobPerseptionEvent>
){
    for (sight, transform, entity) in mob.iter(){
        let pos = transform.translation;
        //then we need to get the what tile we are on
        //then we need to get every tile with in a range
        //and save that data in PerseptionData
        //lets start off with getting the tiles in a box shape Then work on making it a circle
    }
}

