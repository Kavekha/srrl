use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct PlayerCreation {
    pub kind: (String, String),
    pub model: String,
    pub job: (String, String),  // reference, string.
}
impl PlayerCreation {
    pub fn new() -> PlayerCreation {
        PlayerCreation { 
            kind : ("".to_string(), "".to_string()),    // Ref, Name.
            model : "".to_string(),
            job: ("".to_string(), "".to_string()),  // Ref, Name.
        }
    }
}

#[derive(Component)]
pub struct KindProposition {
    pub kind: String,
    pub reference: String,
    pub model: String,
}

#[derive(Component)]
pub struct JobProposition {
    pub job: String,
    pub reference: String
}

#[derive(Component)]
pub struct MenuKindDisplay {
    pub model: String
}

#[derive(Component)]
pub struct SelectedOptionJob;

#[derive(Component)]
pub struct MenuStats {
    pub dirty: bool
}

#[derive(Component)]
pub struct MenuSkills {
    pub dirty: bool
}