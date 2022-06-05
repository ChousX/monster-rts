use bevy::prelude::*;
use crate::{
    mob::{
        MobPerseptionEvent,
        actions::*
    }
};

pub fn basic
(
    percept: EventReader<MobPerseptionEvent>,
    mobs: Query<()>,

){


}
