use bevy::prelude::*;
use crate::{
    mob::{
        MobPerseptionEvent,
        actions::*
    }
};
use std::collections::VecDeque;

pub struct Memory(pub VecDeque<VecDeque<Option<TileType>>>);

pub fn basic
(
    percept: EventReader<MobPerseptionEvent>,
    memory: Query<Memory>,
){


}
