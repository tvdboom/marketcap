use bevy::prelude::*;

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Settings,
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    StartGame,
    Running,
    Paused,
    InGameMenu,
    Settings,
    GameEnd,
}
