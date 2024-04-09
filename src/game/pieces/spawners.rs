use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{
    game::{
        pieces::components::{Health, Melee, Monster, Npc, Occupier, Piece, Stats, Walk}, player::Player, tileboard::components::{BoardPosition, ExitMapTile}
    }, 
    vectors::Vector2Int};

use super::components::CharacterBundle;


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

// 0.19 adds Abilities there.
pub fn create_player(world: &mut World, player_starting_position: Vector2Int){
    println!("Player: Starting position = {:?}", player_starting_position);
    let kind = get_random_kind();
    let piece = Piece{kind: kind};
    println!("Player kind i {:?}", kind);

    // Stats base on Kind.
    let stats:Stats;
    let health:Health;
    match kind {
        Kind::Human => {
            stats = Stats {
                strength: 3,
                agility: 3,
                logic: 3,
                melee: 3,
                firearms: 3,
            };
            health = Health { max: 10, current: 10 };
        },
        Kind::Dwarf => {
            stats = Stats {
                strength: 4,
                agility: 3,
                logic: 3,
                melee: 3,
                firearms: 2,
            };
            health = Health { max: 10, current: 10 };
        },
        Kind::Elf => {
            stats = Stats {
                strength: 2,
                agility: 4,
                logic: 3,
                melee: 2,
                firearms: 4,
            };
            health = Health { max: 9, current: 9 };
        },
        Kind::Orc => {
            stats = Stats {
                strength: 4,
                agility: 2,
                logic: 2,
                melee: 4,
                firearms: 3,
            };
            health = Health { max: 10, current: 10 };
        },
        Kind::Troll => {
            stats = Stats {
                strength: 6,
                agility: 2,
                logic: 1,
                melee: 5,
                firearms: 1,
            };
            health = Health { max: 11, current: 11 };
        },
        Kind::Ghoul => {
            stats = Stats {
                strength: 2,
                agility: 4,
                logic: 1,
                melee: 3,
                firearms: 0,
            };
            health = Health { max: 9, current: 9 };
        }
    };

    let player = world.spawn(CharacterBundle{
            piece: piece, 
            position: BoardPosition{ v:player_starting_position },
            name: Name::new("the ShadowRunner"),
            stats: stats,
            health: health,
            melee: Melee { damage: 0 },
            occupier: Occupier
        }).id();
    //player.insert(Player);
    world.entity_mut(player).insert(Player);
    let stats = world.entity(player).get::<Stats>().unwrap();
    println!("Player stats are {:?}", stats);
}

pub fn spawn_npcs(world: &mut World, entities_pos: Vec<Vector2Int>){
    for entity_position in entities_pos {
        println!("NPC: Starting position = {:?}", entity_position);
        spawn_npc(world, entity_position);
    }
}

fn spawn_npc(world: &mut World, npc_spawning_position: Vector2Int
){
    let mut npc = world.spawn(CharacterBundle {
        piece: Piece{kind: Kind::Ghoul},
        name: Name::new(format!("Ghoul")),
        stats: Stats {
            strength: 2,
            agility: 4,
            logic: 1,
            melee: 3,
            firearms: 0,
        },
        health: Health { max: 10, current: 10 },
        melee: Melee { damage: 2 },
        position: BoardPosition{ v:npc_spawning_position },
        occupier: Occupier,
    });

    // TODO: Clean up sur les Components non utilisés.
    npc
    .insert(Npc)
    .insert(Monster)
    .insert(Walk)
    ;

    println!("Npc created");
}


pub fn create_exit_map(world: &mut World, exit_position: Vector2Int){
    let mut exit = world.spawn_empty();
    exit 
    .insert(Name::new(format!("Exit")))
    .insert(ExitMapTile)
    .insert(BoardPosition{ v:exit_position});
}