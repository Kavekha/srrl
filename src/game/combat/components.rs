use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vectors::Vector2Int;



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

#[derive(Component)]
pub struct WantToHit{
    pub source: Entity,
    pub mode: AttackType,
    pub target: Vector2Int
}

#[derive(Component)]
pub struct TryHit{
    pub attacker: Entity,
    pub mode: AttackType,
    pub defender: Entity
}

#[derive(Clone)]
pub enum AttackType{
    RANGED,
    MELEE
}

#[derive(Component)]
pub struct MissHit{
    pub attacker: Entity, 
    pub mode: AttackType,
    pub defender: Entity
}

