use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vectors::Vector2Int;


#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct ActionPoints {
    pub max: u32,
    pub current: u32
}

#[derive(Component)]
pub struct IsDead;

#[derive(Component)]
pub struct WantToHit{
    pub mode: AttackType,
    pub target: Vector2Int
}

#[derive(Component)]
pub struct TryHit{
    pub mode: AttackType,
    pub defender: Entity
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttackType{
    RANGED,
    MELEE
}

#[derive(Component)]
pub struct MissHit{
    pub mode: AttackType,
    pub defender: Entity
}

#[derive(Component)]
pub struct GetHit{
    pub attacker: Entity, 
    pub mode: AttackType,
    pub dmg: i32,
}

#[derive(Component)]
pub struct Die;

#[derive(Component)]
pub struct WantToForfeit;