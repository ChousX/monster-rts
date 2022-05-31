use bevy::prelude::*;
pub struct MobPlugin;
impl Plugin for MobPlugin{
    fn build(&self, app: &mut App) {
        
    }
}
//TODO: construct sprites for mobs I am thinking compositing later on for now just a square will do  
//TODO: Spawn mobs and attach sprites
//TODO: Mob minimum movement logic can only walk on tiles
