use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Piece {
    pub model: String,
}


#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Npc;

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
pub struct Melee;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]  
pub struct Ranged;

//TODO : Adapter à Shadowrun: Skill & Ability.
#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    //pub power: u32,         
    //pub attack: u32,
    //pub dodge: u32,
    //pub resilience: u32
    //Stats
    pub strength: u32,  // melee dmg & resistance. HP = Str / 2 + 8.
    pub agility: u32,   // accuracy
    pub logic: u32,     // Logic + agility = dodge
    //Skills
    pub melee:u32,
    pub firearms:u32
} 


#[derive(Component)]
pub struct GameElement;

#[derive(Component)]
pub struct NavigationNode;


#[derive(Component)]
pub struct Vision {
    pub range_view: u32
}