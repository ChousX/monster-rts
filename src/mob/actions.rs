use bevy::prelude::*;
use super::*;
pub struct MobMoveEvent(pub Vec3, Entity);
pub fn move_mob(
    mut mob: Query<&mut Transform, With<Mob>>,
    mut move_event: EventReader<MobMoveEvent>,
)
{
    for event in move_event.iter(){
        if let Ok(mut transform) = mob.get_mut(event.1){
            transform.translation += event.0;
        }
    }
}