use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{
    game::{
        combat::abilities::{create_ability_attack, Abilities, AbilityType}, 
        pieces::components::{Health, Melee, Monster, Npc, Occupier, Piece, PlayerCharacterBundle, Stats, Walk}, 
        tileboard::components::{BoardPosition, ExitMapTile}
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
    //starting abilities
    let melee_ability = create_ability_attack(AbilityType::Melee);
    let ranged_ability = create_ability_attack(AbilityType::Ranged);
    let mut abilities = Abilities::new();
    abilities.add(melee_ability);
    abilities.add(ranged_ability);

    world.spawn(PlayerCharacterBundle{
        character: CharacterBundle{
            piece: piece, 
            position: BoardPosition{ v:player_starting_position },
            ..default()
        },
        abilities: abilities,
        ..default()
    });
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
            power: 4,         
            attack: 4,
            dodge: 3,
            resilience: 4
        },
        health: Health { max: 10, current: 10 },
        melee: Melee { damage: 2 },
        position: BoardPosition{ v:npc_spawning_position },
        occupier: Occupier,
    });

    npc
    .insert(Npc)
    .insert(Monster)
    .insert(Walk);

    println!("Npc created");
}


pub fn create_exit_map(world: &mut World, exit_position: Vector2Int){
    let mut exit = world.spawn_empty();
    exit 
    .insert(Name::new(format!("Exit")))
    .insert(ExitMapTile)
    .insert(BoardPosition{ v:exit_position});
}