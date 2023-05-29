use bevy::prelude::*;

use crate::game::actions::Action;


#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

#[derive(Component)]
// movement behaviour for non-player pieces
pub struct Walk;