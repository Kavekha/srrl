use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{game::{combat::abilities::Abilities, player::Player, tileboard::components::BoardPosition}, vectors::Vector2Int};

use super::spawners::Kind;


#[derive(Bundle)]
pub struct CharacterBundle {    
    pub piece: Piece,
    pub name: Name,
    pub stats: Stats,
    pub health: Health,
    pub melee: Melee,
    pub position: BoardPosition,
    pub occupier: Occupier
}
impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            piece: Piece { kind: Kind::Human },
            name: Name::new("Nobody"),  //TODO change i guess
            stats: Stats {
                power: 1,         
                attack: 1,
                dodge: 1,
                resilience: 1
            },
            health: Health { max: 10, current: 10 },
            melee: Melee { damage: 0 },
            position: BoardPosition { v: Vector2Int { x:0, y: 0 } },
            occupier: Occupier
        }
    }
}

#[derive(Bundle)]
pub struct PlayerCharacterBundle {
    pub character : CharacterBundle,
    pub player: Player,
    pub abilities: Abilities
}
impl Default for PlayerCharacterBundle {
    fn default() -> Self {
        Self {
            character: CharacterBundle {
                name: Name::new("the ShadowRunner"),
                stats: Stats {
                    power: 3,         
                    attack: 6,
                    dodge: 6,
                    resilience: 3
                },
                ..default()
            },
            player: Player, 
            abilities: Abilities::new()
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
pub struct Melee {
    pub damage: u32
}

//TODO : Adapter à Shadowrun: Skill & Ability.
#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    pub power: u32,         
    pub attack: u32,
    pub dodge: u32,
    pub resilience: u32
} 
