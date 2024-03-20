use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Disabled,
    //NewGame,    // Nouvelle partie, setup Map & player creation //    v0.15.2
    Prerun,     // Partie logique de la map.. //  v0.15.2
    MapGeneration,
    GameMap,    // La map et le perso qui s'y balade.
    GameOverScreen,
    VictoryScreen,
    SaveGame,
    LoadGame,
}  


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MainMenuState {
    #[default]
    MainMenu,
    Settings,
    DisplayMenu,
    QuitConfirm,
    Disabled
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

