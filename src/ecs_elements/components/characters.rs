use bevy::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Player;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Npc;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Monster;