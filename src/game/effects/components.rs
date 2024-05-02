use bevy::prelude::*;

use crate::vectors::Vector2Int;

pub enum EffectType { 
    Damage { amount : i32 }
}

#[derive(Clone)]
pub enum Targets {
    Single { target : Entity },
    List { target: Vec<Entity> },
    Tile { target: Vector2Int }
}

#[derive(Event)]
pub struct NewEffectEvent {
    pub creator : Option<Entity>,
    pub effect_type : EffectType,
    pub targets : Targets
}