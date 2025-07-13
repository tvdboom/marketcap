use bevy::prelude::*;

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AppState {
    #[default]
    Game,
    MainMenu,
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    InGameMenu,
    Settings,
    GameEnd,
}
