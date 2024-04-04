use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{game::player::cursor::CursorMode, vectors::Vector2Int};



#[derive(Default, Resource)]
pub struct CombatTurnQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct EntityEndTurnEvent {
    pub entity: Entity
}

#[derive(Event)]
pub struct CombatTurnEndEvent;

#[derive(Event)]
pub struct CombatTurnNextEntityEvent;

#[derive(Event)]
pub struct CombatTurnStartEvent;

#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct Turn;


#[derive(Event)]
pub struct EntityMoveEvent {
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>,    
    pub target: Option<Vector2Int>,
}

#[derive(Event)]
pub struct EntityTryMoveEvent {
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>,
    pub target: Option<Vector2Int>,
    //pub destination: Vector2Int
}


#[derive(Event)]
pub struct EntityHitTryRangedEvent {
    pub entity: Entity,
    pub target: Vector2Int,
}


#[derive(Event)]
pub struct EntityHitTryEvent {
    pub entity: Entity,
    pub target: Vector2Int,
}

#[derive(Event)]
pub struct EntityHitMissEvent {
    pub entity: Entity,
    pub defender: Entity
}

#[derive(Event)]
pub struct EntityGetHitEvent {
    pub entity: Entity,
    pub attacker: Entity,
    pub dmg: u32,
}

#[derive(Event)]
pub struct EntityDeathEvent {
    pub entity: Entity,
    pub attacker: Entity
}

#[derive(Event)]
pub struct RefreshActionCostEvent;


// 0.19 Refacto
#[derive(Event)]
pub struct WantToHitEvent {
    pub source: Entity,
    pub target: Vector2Int,
    pub mode: CursorMode    // TO CHANGE : Demonstration ici du ridicule d'avoir le mode sur le curseur.    
}