#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState{
    Pre,
    MainMenu,
    Settings,
    GameLoad,
    MainGame,
    GameMenu,
    Pause,
}