use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::actions::Action;

use super::spawners::Kind;

#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Piece {
    pub kind: Kind
}

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);    // The Action, Value of the Action for this NPC.

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Npc;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Monster;


#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
// movement behaviour for non-player pieces
pub struct Walk; 

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)] 
// there can be only a single occupier piece on the same tile
pub struct Occupier;


#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)] 
pub struct Health {
    pub max: u32,
    pub current: u32
}

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]  
pub struct Melee {
    pub damage: u32
}

//TODO : Adapter à Shadowrun: Skill & Ability.
#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    pub power: u32,         
    pub attack: u32,
    pub dodge: u32,
    pub resilience: u32
} 