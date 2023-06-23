use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Default, Resource)]
pub struct CombatTurnQueue(pub VecDeque<Entity>);



#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct ActionPoints {
    pub max: u32,
    pub current: u32
}


#[derive(Event)]
pub struct EntityEndTurnEvent {
    pub entity: Entity
}

#[derive(Event)]
pub struct CombatTurnEndEvent;

#[derive(Event)]
pub struct CombatTurnNextEntityEvent;