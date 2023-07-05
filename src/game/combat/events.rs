use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vectors::Vector2Int;



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
pub struct AnimateEvent {
    //anim_type?
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>
}

#[derive(Event)]
pub struct OnClickEvent {
    pub entity: Entity,
    pub tile: Vector2Int,
}

#[derive(Event)]
pub struct EntityHitTryEvent {
    pub entity: Entity,
    pub target: Vector2Int,
}

#[derive(Event)]
pub struct EntityGetHitEvent {
    pub entity: Entity,
    pub attacker: Entity,
    pub dmg: u32,
}

#[derive(Event)]
pub struct EntityDeathEvent {
    pub entity: Entity
}

#[derive(Event)]
pub struct RefreshActionCostEvent;