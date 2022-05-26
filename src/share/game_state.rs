#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState{
    MainMenu,
    Settings,
    GameLoad,
    MainGame,
    GameMenu,
    Pause,
}