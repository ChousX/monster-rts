use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

//TODO: need to regester a lot of stuff like every thing
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}
