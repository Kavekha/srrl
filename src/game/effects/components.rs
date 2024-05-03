use bevy::prelude::*;

use crate::vectors::Vector2Int;

pub enum EffectType { 
    Damage { amount : i32 },
    Bloodstain,
}

#[derive(Clone)]
pub enum Targets {
    Single { target : Entity },
    List { target: Vec<Entity> },
    Tile { target: Vector2Int }
}

