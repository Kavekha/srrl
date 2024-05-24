use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};



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
    pub max: i32,
    pub current: i32
}

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]  
pub struct Melee;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]  
pub struct Ranged;

/* 0.21c : Remplacé par Attributes 
//TODO : Adapter à Shadowrun: Skill & Ability.
#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    pub strength: u32,  // melee dmg & resistance. HP = Str / 2 + 8.
    pub agility: u32,   // accuracy
    pub logic: u32,     // Logic + agility = dodge
    //Skills
    pub melee:u32,
    pub firearms:u32
} 
*/

#[derive(Component)]
pub struct Vision {
    pub range_view: u32
}


#[derive(Component)]
pub struct GameElement;


#[derive(Debug, Component)]
pub struct Attribute {
    pub base: i32,
    pub modifiers: i32,
    pub max: i32
}

#[derive(Debug, Component)]
pub struct Attributes {
    pub strength: Attribute,
    pub agility: Attribute,
    pub logic: Attribute 
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Skill {
    UnarmedCombat,
    FireArms,
    CloseCombat
}

#[derive(Component)]
pub struct Skills {
    pub skills: HashMap<Skill, i32>
}