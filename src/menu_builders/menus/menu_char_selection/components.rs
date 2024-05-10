use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct PlayerCreation {
    pub kind: String,
    pub model: String,
    pub job: HashMap<String, String>,
}
impl PlayerCreation {
    pub fn new() -> PlayerCreation {
        PlayerCreation { 
            kind : "".to_string(),
            model : "".to_string(),
            job: HashMap::new(),
        }
    }
}

#[derive(Component)]
pub struct KindProposition {
    pub kind: String,
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