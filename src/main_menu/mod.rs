use crate::share::{paths::FONT, GameState, Menu, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use bevy::app::AppExit;
use bevy::prelude::*;

mod game_menu;
pub struct DeleteList([Entity; 3]);
#[derive(Component, Clone, Copy)]
pub enum MainMenu {
    Play,
    Quit,
    Settings,
}

impl Menu for MainMenu {
    fn text(&self) -> &str {
        use MainMenu::*;
        match *self {
            Play => "Play",
            Settings => "Settings",
            Quit => "Quit",
        }
    }

    fn build(commands: &mut Commands, asset_server: Res<AssetServer>) {
        let quite = Self::Quit.to_button(commands, asset_server.load(FONT));
        let play = Self::Play.to_button(commands, asset_server.load(FONT));
        let settings = Self::Settings.to_button(commands, asset_server.load(FONT));
        commands.insert_resource(DeleteList([quite, play, settings]));
    }
}

fn init(mut commands: Commands, assets: Res<AssetServer>) {
    MainMenu::build(&mut commands, assets);
}

pub fn update(
    mut state: ResMut<State<GameState>>,
    mut interaction: Query<
        (&Interaction, &mut UiColor, &MainMenu),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in interaction.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match button {
                    &MainMenu::Play => state.set(GameState::GameMenu).unwrap(),
                    &MainMenu::Quit => {
                        exit.send(AppExit);
                    }
                    &MainMenu::Settings => state.set(GameState::Settings).unwrap(),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn clean_up(mut commands: Commands, list: ResMut<DeleteList>) {
    for entity in list.0.into_iter() {
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<DeleteList>();
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(init))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(clean_up))
            .add_plugin(game_menu::GameMenuPlug);
    }
}
