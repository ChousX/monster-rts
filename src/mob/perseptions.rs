use crate::map::{TILE_SIZE, TileType};

use super::Mob;
use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, TilePos};
use std::collections::VecDeque;

pub type PerseptionData = Option<TileType>;
#[derive(Default)]
pub struct MobPerseptionEvent(pub VecDeque<VecDeque<PerseptionData>>);

#[derive(Default, Component)]
pub struct Sight {
    rang: u32,
}
pub fn mob_vision(
    mob: Query<(&Sight, &Transform, Entity), With<Mob>>,
    tiles: Query<&TileType>,
    mut map: MapQuery,
    mut output: EventWriter<MobPerseptionEvent>,
) {
    for (sight, transform, entity) in mob.iter() {
        let mut pos = transform.translation;
        pos.x /= TILE_SIZE.0;
        pos.y /= TILE_SIZE.1;
        let pos = TilePos(pos.x as u32, pos.y as u32);
        let (x_1, y_1, x_2, y_2) = {
            let (_, layer) = map.get_layer(0, 0).expect("the map has not been inited");
            let layer_size = layer.get_layer_size_in_tiles();
            (
                if pos.0 > sight.rang{
                    0
                } else {
                    pos.0 - sight.rang
                },

                if pos.1 > sight.rang{
                    0
                } else {
                    pos.1 - sight.rang
                },

                if (pos.0 + sight.rang) >= layer_size.0{
                    layer_size.0
                } else {
                    pos.0 + sight.rang
                },

                if (pos.1 + sight.rang) >= layer_size.1{
                    layer_size.1
                } else {
                    pos.1 + sight.rang
                }
            )
        };
        let mut seen: VecDeque<VecDeque<Option<TileType>>> = VecDeque::with_capacity((y_2 - y_1) as usize);
        for y in y_1..y_2{
            let mut row_seen: VecDeque<Option<TileType>> = VecDeque::with_capacity((x_2 - x_1) as usize);
            for x in x_1..x_2{
                if let Ok(entity) = map.get_tile_entity(TilePos(x, y), 0, 0){
                    if let Ok(tile_type) = tiles.get(entity){
                        row_seen.push_back(Some(*tile_type));
                    } else {
                        row_seen.push_back(None);
                    }
                }
            }
            seen.push_back(row_seen);
        }
        output.send(MobPerseptionEvent(seen))
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
