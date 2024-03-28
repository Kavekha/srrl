use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Disabled,
    Initialise,
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
