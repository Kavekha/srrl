use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::actions::Action;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Piece {
    pub size: i32
}

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);    // The Action, Value of the Action for this NPC.

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
// movement behaviour for non-player pieces
pub struct Walk; 

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]   //TODO : Add to save process.
// there can be only a single occupier piece on the same tile
pub struct Occupier;