use bevy::prelude::*;
use bevy::app::AppExit;
use crate::share::{GameState, Menu, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, paths::FONT};
pub struct DeleteList([Entity; 3]);

pub struct GameMenuPlug;
impl Plugin for GameMenuPlug{
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(init))
        .add_system_set(SystemSet::on_update(GameState::GameMenu).with_system(update))
        .add_system_set(SystemSet::on_exit(GameState::GameMenu).with_system(clean_up))
        ;
    }
}

#[derive(Component, Clone, Copy)]
pub enum GameMenu{
    NewGame,
    LoadGame,
    Back,
}
impl Menu for GameMenu{
    fn text(&self) -> &str {
        use GameMenu::*;
        match *self{
            NewGame => "New Game",
            LoadGame => "Load Game",
            Back => "Back"
        }
    }

    fn build(commands: &mut Commands, asset_server: Res<AssetServer>) {
        let list = DeleteList([Self::LoadGame.to_button(commands, asset_server.load(FONT)),
        Self::NewGame.to_button(commands, asset_server.load(FONT)),
        Self::Back.to_button(commands, asset_server.load(FONT))]);
        commands.insert_resource(list);
    }
}

fn init(
    mut commands: Commands,
    assets: Res<AssetServer>,
){
    GameMenu::build(&mut commands, assets);
}

pub fn update(
    mut state: ResMut<State<GameState>>,
    mut interaction: Query<
        (&Interaction, &mut UiColor, &GameMenu),
        (Changed<Interaction>, With<Button>)
    >,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, mut color, button) in interaction.iter_mut() {
        match *interaction{
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match button{
                    &GameMenu::NewGame => state.set(GameState::GameLoad).unwrap(),
                    &GameMenu::LoadGame => state.set(GameState::GameLoad).unwrap(),
                    &GameMenu::Back => state.set(GameState::MainMenu).unwrap(),
                }
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn clean_up(mut commands: Commands, list: ResMut<DeleteList>){
    for entity in list.0.into_iter(){
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<DeleteList>();
}