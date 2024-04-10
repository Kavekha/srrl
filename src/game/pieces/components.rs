use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{game::tileboard::components::BoardPosition, vectors::Vector2Int};

use super::spawners::Kind;


#[derive(Bundle)]
pub struct CharacterBundle {    
    pub piece: Piece,
    pub name: Name,
    pub stats: Stats,
    pub health: Health,
    pub position: BoardPosition,
    pub occupier: Occupier,
}
impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            piece: Piece { kind: Kind::Human },
            name: Name::new("Nobody"),  //TODO change i guess
            stats: Stats {
                strength: 3,
                agility: 3,
                logic: 2,
                melee: 1,
                firearms: 1,
            },
            health: Health { max: 10, current: 10 },
            position: BoardPosition { v: Vector2Int { x:0, y: 0 } },
            occupier: Occupier
        }
    }
}



#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Piece {
    pub kind: Kind
}


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
