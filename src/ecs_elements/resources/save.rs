use bevy::prelude::*;


#[derive(Resource)]
pub struct ShouldSave {
    pub to_save: bool
}