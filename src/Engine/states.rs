use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Disabled,
    NewGame,    // Nouvelle partie, setup Map & player creation
    Prerun,     // Partie logique de la map..
    MapGeneration,
    GameMap,    // La map et le perso qui s'y balade.
    GameOverScreen,
    VictoryScreen,
    SaveGame,
    LoadGame,
}  

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MainMenuState {
    #[default]
    Disabled,
    MainMenu,
    Settings,
    DisplayMenu,
    QuitConfirm
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum EngineState {
    #[default]
    None,
    //PlayerInput,
    //TurnUpdate
}



#[derive(SystemSet, Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TurnSet {
    #[default]
    Logic,
    //Animation,
    //Tick
}

