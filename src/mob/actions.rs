use super::*;
use bevy::prelude::*;
pub struct MobMoveEvent(pub Vec3, Entity);
pub fn move_mob(
    mut mob: Query<&mut Transform, With<Mob>>,
    mut move_event: EventReader<MobMoveEvent>,
) {
    for event in move_event.iter() {
        if let Ok(mut transform) = mob.get_mut(event.1) {
            transform.translation += event.0;
        }
    }
}

//TODO Things like damage types and armor 
pub struct DamageEvent(pub f32, pub Entity);
pub fn damage_mob(
    mut reader: EventReader<DamageEvent>,
    mut mobs: Query<&mut Health, With<Mob>>,
){
    for event in reader.iter(){
        if let Ok(mut health) = mobs.get_mut(event.1){
            health.current -= event.0;
        }
    }

}



pub fn mob_death(
    mut commands: Commands,
    mobs: Query<(&Health, Entity), With<Mob>>
){
    for (health, entity) in mobs.iter(){
         if health.current <= 0.0{
            commands.entity(entity).despawn_descendants();
            commands.entity(entity).despawn();
        }
    }
}