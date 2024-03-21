use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{game::{pieces::components::{Health, Melee, Monster, Npc, Occupier, Piece, Stats, Walk}, player::Player, tileboard::components::{BoardPosition, ExitMapTile}}, vectors::Vector2Int};


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



pub fn create_player(world: &mut World, player_starting_position: Vector2Int){
    //if let Some(game_infos) = world.get_resource::<GameInfos>(){
        //let player_starting_position = game_infos.starting_position;
        println!("Player: Starting position = {:?}", player_starting_position);
        let kind = get_random_kind();
        let piece = Piece{kind: kind};

        let mut player = world.spawn_empty();
        
        player
            .insert(piece)
            .insert(Player)
            .insert(Name::new("The Shadowrunner"))
            //TODO : Shadowrun stats
            .insert(Stats {
                power: 3,         
                attack: 6,
                dodge: 6,
                resilience: 3
            })
            //.insert(Actor::default(),)
            .insert(Health { max: 10, current: 10 })
            .insert(Melee { damage: 1 })
            .insert(BoardPosition{ v:player_starting_position })
            .insert(Occupier);
    //}
}


pub fn spawn_npcs(world: &mut World, entities_pos: Vec<Vector2Int>){
    for entity_position in entities_pos {
        println!("NPC: Starting position = {:?}", entity_position);
        spawn_npc(world, entity_position);
    }
}

fn spawn_npc(world: &mut World, npc_spawning_position: Vector2Int
) {
        let mut npc = world.spawn_empty();
        
        npc
        .insert(Name::new(format!("Ghoul")))
        .insert(Piece{kind: Kind::Ghoul })
        .insert(Stats {
            power: 4,         
            attack: 4,
            dodge: 3,
            resilience: 4
        })
        //.insert(Actor::default(),)
        .insert(Npc)
        .insert(Monster)
        .insert(Walk)
        .insert(Melee { damage: 2 })
        .insert(Health { max: 10, current: 10 })
        .insert(BoardPosition{ v:npc_spawning_position })
        .insert(Occupier);
    println!("Npc created");
}

pub fn create_exit_map(world: &mut World, exit_position: Vector2Int){
    let mut exit = world.spawn_empty();
    exit 
    .insert(Name::new(format!("Exit")))
    .insert(ExitMapTile)
    .insert(BoardPosition{ v:exit_position});
}