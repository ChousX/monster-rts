use bevy::prelude::*;


mod debug;
mod share;
mod mob;
mod map;
mod main_menu;
mod hud;
mod controls;

use share::GameState;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(ui_cam_init)
        .add_startup_system(share::asset_checker_init)
        .add_state(GameState::MainMenu)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(map::MapPlugin)
        //.add_plugin(controls::ControlsPlugin)
        .run();
}

fn ui_cam_init(mut commands: Commands){
    commands.spawn_bundle(UiCameraBundle::default());
}


/*
mod map{
    use bevy::{ecs::entity::Entity, prelude::*};
    use crate::GameState;

    fn make_map(mut commands: Commands){
        commands.spawn().insert(Plain);
    }

    pub const PLAIN_CHUNK_COUNT: (usize, usize) = (10, 10);
    
    #[derive(Component)]
    pub struct Plain;

    pub const CHUNK_SIZE: (usize, usize) = (10, 10);

    pub struct PlainerChunk{
        arena: [[Entity; CHUNK_SIZE.0]; CHUNK_SIZE.1],

    }

    pub enum Tile{
        Dirt,
        Water,
        Grass,
        Void,
    }

    pub enum TileProperty{

    }
}




mod main_menu{
    use bevy::prelude::*;
    use crate::GameState;

    use crate::menu::MenuFont;

    pub struct MainMenuPlugin;
    impl Plugin for MainMenuPlugin {
        fn build(&self, app: &mut App) {
            
        }
    }

    fn setup_menu(mut commands: Commands, assets: Res<AssetServer>, menu_font: Res<MenuFont>){
        commands.spawn_bundle(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section("Play", 
                TextStyle { 
                    font: menu_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.0, 0.5)
                },
                Default::default(),
            ),
            focus_policy: bevy::ui::FocusPolicy::Pass,
            ..Default::default()
            
        });
    });
    }

}

mod pre_game_load{
    use bevy::prelude::*;
    use bevy::asset::LoadState;
    use crate::GameState;
    use GameState::MainMenu as mm;

    #[derive(Default)]
    pub struct AssetsLoading(Vec<HandleUntyped>);

    pub fn init_loading (mut commands: Commands){
        commands.init_resource::<AssetsLoading>();
    }

    pub fn check_assets_ready(
        mut commands: Commands,
        server: Res<AssetServer>,
        loading: Res<AssetsLoading>,
        mut state: ResMut<State<GameState>>,
    )
    {
        match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
            LoadState::Failed => {
                // one of our assets had an error
            }
            LoadState::Loaded => {
                // all assets are now ready
                state.set(GameState::MainMenu).unwrap();
                
            }
            _ => {
                // NotLoaded/Loading: not fully ready yet
            }
        }
        

    }

    pub fn clean_up(mut commands: Commands){
        commands.remove_resource::<AssetsLoading>();
    }
}

mod pre_game_options{
    use bevy::prelude::*;
    use crate::GameState;
    use crate::GameState::PreGameOptions as pgo;
    use crate::GameState::PreGameLoad as pgl;
    use crate::menu::{MenuItem, menu_prelude};
    use crate::pre_game_load::{clean_up, check_assets_ready, init_loading};

    pub struct  PreGamePlugin;

    impl Plugin for PreGamePlugin {
        fn build(&self, app: &mut App) {
            app
                .add_system_set(SystemSet::on_enter(pgo).with_system(init_loading))

                .add_system_set(SystemSet::on_enter(pgo).with_system(menu_prelude))
                .add_system_set(SystemSet::on_enter(pgo).with_system(setup))
                .add_system_set(SystemSet::on_exit(pgo).with_system(exit))
                .add_system_set(SystemSet::on_update(pgo).with_system(update))

                .add_system_set(SystemSet::on_update(pgl).with_system(check_assets_ready))
                .add_system_set(SystemSet::on_exit(pgl).with_system(clean_up))
                ;
        }

        fn name(&self) -> &str {
            "Pre Game Settings Plugin:"
        }
    }
    pub struct PreGameOptions{
        resolution: Option<()>,
        
    }

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
        //commands.spawn_bundle(UiCameraBundle::default());
    }

    pub fn update(mut commands: Commands, mut state: ResMut<State<GameState>>){
        state.set(GameState::PreGameLoad).unwrap();
    }

    pub fn exit(mut commands: Commands){

    }

}

mod menu{
    use bevy::prelude::*;
    use std::path::Path;

    use crate::pre_game_load::AssetsLoading;
    pub trait MenuItem{
        fn text(&self) -> &str;
    }

    

    #[derive(Default)]
    pub struct MenuFont(pub Handle<Font>);
    const MENU_FONT_PATH: &str = r"fonts\OpenDyslexicMono-Regular.otf";

    pub fn menu_prelude(mut commands: Commands, asset_server: Res<AssetServer>, loading: ResMut<AssetsLoading>){
        commands.spawn_bundle(UiCameraBundle::default());

        let menu_font = asset_server.load(MENU_FONT_PATH);

        commands.insert_resource(MenuFont(menu_font));
    }

}
*/