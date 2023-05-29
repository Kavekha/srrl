use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::actions::Action;


#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
// movement behaviour for non-player pieces
pub struct Walk; 