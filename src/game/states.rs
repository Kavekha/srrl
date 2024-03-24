use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Disabled,
    Running,
    Unavailable,    // En etat de pause du à un Menu ou autre.
    Processing,     // Sauvegarde ou load en cours.
    SaveGame,
    LoadGame,
}  


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Splashscreen,
    Disabled,
    Open
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

