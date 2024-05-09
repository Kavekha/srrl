use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct PlayerCreation {
    pub kind: String,
    pub model: String,
}
impl PlayerCreation {
    pub fn new() -> PlayerCreation {
        PlayerCreation { 
            kind : "".to_string(),
            model : "".to_string()
        }
    }
}

#[derive(Component)]
pub struct KindProposition {
    pub kind: String,
    pub model: String,
}

#[derive(Component)]
pub struct MenuKindDisplay {
    pub model: String
}