use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::log::LogPlugin;

//TODO: need to regester a lot of stuff like every thing
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_plugin(WorldInspectorPlugin::new())
                ;
                //bevy_mod_debugdump::print_schedule(app);
        }
    }
}
