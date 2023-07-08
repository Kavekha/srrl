use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{
    globals::{SIZE_GHOUL, SIZE_HUMAN, SIZE_ELF, SIZE_TROLL, SIZE_DWARF, SIZE_ORC}, 
    game::{player::Player, pieces::components::{Health, Occupier, Stats}, tileboard::components::ExitMapTile}};

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
) -> (Kind, i32) {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..4);
    //TODO : Le size n'a plus de sens, c'etait une donnée "image" et toutes les images sont maintenant en 64x96.
    match rand {
        0 => { return (Kind::Dwarf, SIZE_TROLL); }
        1 => { return (Kind::Elf, SIZE_TROLL); }
        2 => { return (Kind::Orc, SIZE_TROLL);}
        3 => { return (Kind::Troll, SIZE_TROLL);}
        4 => { return (Kind::Human, SIZE_TROLL);}
        _ => { return (Kind::Human, SIZE_TROLL);}
    }
}


pub fn spawn_player(
    commands: &mut Commands
) -> Entity {
    // Random choice of kind.
    let (kind, size) = get_random_kind();

    let player = commands.spawn(Piece{kind: kind, size: size }).id();
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
    let npc = commands.spawn(Piece{kind: Kind::Ghoul, size: SIZE_GHOUL}).id();
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


