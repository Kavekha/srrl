use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{game::actions::Action, vectors::Vector2Int};

use super::spawners::Kind;

#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Piece {
    pub kind: Kind,
    pub size: i32
}

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);    // The Action, Value of the Action for this NPC.

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
// movement behaviour for non-player pieces
pub struct Walk; 

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)] 
// there can be only a single occupier piece on the same tile
pub struct Occupier;


#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)] 
pub struct Health;  // Can receive dmg

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]  
pub struct Melee;   // Can melee

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone)]   // TODO : Add to save.
pub struct PathTo{
    pub pathing: Vec<Vector2Int>
}