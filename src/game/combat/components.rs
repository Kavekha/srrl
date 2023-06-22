use bevy::prelude::*;
use serde::{Deserialize, Serialize};


//TODO : Adapter à Shadowrun: Skill & Ability.
#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct ActionPoints {
    pub max: u32,
    pub current: u32
}


#[derive(Event)]
pub struct EndTurnEvent {
    pub entity: Entity
}