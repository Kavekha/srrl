use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::game::{player::Player, pieces::components::{Health, Occupier, Stats}, tileboard::components::ExitMapTile};

use super::components::{Piece, Actor, Walk, Melee, Npc, Monster};


#[derive(Component, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Dwarf,
    Elf,
    Human,
    Orc,
    Troll,
    Ghoul   
}

/// TEMP : Renvoie infos rendus pour les differentes races jouables par le PJ.
pub fn get_random_kind(
) -> Kind {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..4);
    //TODO : Le size n'a plus de sens, c'etait une donnée "image" et toutes les images sont maintenant en 64x96.
    match rand {
        0 => { return Kind::Dwarf; }
        1 => { return Kind::Elf; }
        2 => { return Kind::Orc; }
        3 => { return Kind::Troll; }
        4 => { return Kind::Human; }
        _ => { return Kind::Human; }
    }
}


pub fn spawn_player(
    commands: &mut Commands
) -> Entity {
    // Random choice of kind.
    let kind = get_random_kind();

    let player = commands.spawn(Piece{kind: kind }).id();
    println!("Player is : {:?}", kind);
    commands
        .entity(player)
        .insert(Player)
        .insert(Name::new("The Shadowrunner"))
        //TODO : Shadowrun stats
        .insert(Stats {
            power: 3,         
            attack: 6,
            dodge: 6,
            resilience: 3
        })
        .insert(Actor::default(),)
        .insert(Health { max: 10, current: 10 })
        .insert(Melee { damage: 1 })
        .insert(Occupier)
        .id()  
}

pub fn spawn_npc(
    commands: &mut Commands,
) -> Entity {
    let npc = commands.spawn(Piece{kind: Kind::Ghoul }).id();
    commands
        .entity(npc)
        .insert(Name::new(format!("Ghoul")))
        .insert(Stats {
            power: 4,         
            attack: 4,
            dodge: 3,
            resilience: 4
        })
        .insert(Actor::default(),)
        .insert(Npc)
        .insert(Monster)
        .insert(Walk)
        .insert(Melee { damage: 2 })
        .insert(Health { max: 10, current: 10 })
        .insert(Occupier)
        .id()  
}


pub fn spawn_exit(
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn_empty()
        .insert(Name::new(format!("Exit")))
        .insert(ExitMapTile)
        .id()
}


