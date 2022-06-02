use crate::share::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Default)]
pub struct AssetChecker(pub Vec<HandleUntyped>);

pub fn init(mut commands: Commands) {
    if cfg!(debug_assertions) {
        println!("AssetChecker: Inited")
    }
    commands.init_resource::<AssetChecker>();
}

//this does not seem to be running
pub fn check(
    mut commands: Commands,
    server: Res<AssetServer>,
    assets: Res<AssetChecker>,
    mut state: ResMut<State<GameState>>,
) {
    if cfg!(debug_assertions) {
        println!("AssetChecker: Checking")
    }

    if let LoadState::Loaded = server.get_group_load_state(assets.0.iter().map(|h| h.id)) {
        if cfg!(debug_assertions) {
            println!("AssetChecker: Loadded")
        }
        commands.remove_resource::<AssetChecker>();
        state.set(GameState::MainGame).unwrap();
    }
}
