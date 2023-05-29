use bevy::prelude::*;

use crate::game::actions::Action;


#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);