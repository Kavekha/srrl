use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct ActionPoints {
    pub max: u32,
    pub current: u32
}


#[derive(Default, Resource)]
pub struct CurrentEntityTurnQueue(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct CombatInfos {
    pub turn: u32,
    pub current_entity: Option<Entity>
}

#[derive(Component)]
pub struct IsDead;
