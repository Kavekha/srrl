use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Player;

