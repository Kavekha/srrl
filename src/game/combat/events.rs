use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vectors::Vector2Int;



#[derive(Default, Resource)]
pub struct CombatTurnQueue(pub VecDeque<Entity>);

/* 
#[derive(Event)]
pub struct EntityEndTurnEvent {
    pub entity: Entity
}
*/

#[derive(Event)]
pub struct CombatTurnEndEvent;

#[derive(Event)]
pub struct CombatTurnNextEntityEvent;

#[derive(Event)]
pub struct CombatTurnStartEvent;

#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct Turn;


#[derive(Event)]
pub struct RefreshActionCostEvent;

// 0.19 Refacto
#[derive(Event)]
pub struct WantToHitEvent {
    pub source: Entity,
    pub target: Vector2Int      
}

#[derive(Event)]
pub struct TickEvent;