use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{
    commons::get_world_position, engine::asset_loaders::GraphicsAssets, game::{
        combat::rules::{NPC_CHANCE_TO_BE_RANGED, VISIBILITY_RANGE_PLAYER}, 
        pieces::components::{Health, Melee, Npc, Occupier, Piece, Ranged, Stats, Walk}, player::Player, tileboard::components::{BoardPosition, ExitMapTile}, visibility::components::{Marker, View}
    }, globals::{ORDER_MARKER, SPRITE_PLAYER_HUMAN}, vectors::Vector2Int};

use super::components::{CharacterBundle, GameElement, NavigationNode};


#[derive(Component, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Dwarf,
    Elf,
    Human,
    Orc,
    Troll,
    Ghoul,
    GhoulRanged
}

/// TEMP : Renvoie infos rendus pour les differentes races jouables par le PJ.
fn get_random_kind(
) -> Kind {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..4);
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
        },
        Kind::GhoulRanged => {
            stats = Stats {
                strength: 1,
                agility: 5,
                logic: 2,
                melee: 0,
                firearms: 2,
            };
            health = Health { max: 9, current: 9 };
        },
    };

    let player = world.spawn(CharacterBundle{
            piece: piece, 
            position: BoardPosition{ v:player_starting_position },
            name: Name::new("the ShadowRunner"),
            stats: stats,
            health: health,
            ..default()
        }).id();
    //player.insert(Player);
    world.entity_mut(player)
    .insert(Player)
    .insert(Melee)
    .insert(Ranged)
    .insert(View { 
        visible_tiles: Vec::new(),
        range: VISIBILITY_RANGE_PLAYER
    })
    ;
}

pub fn spawn_npcs(world: &mut World, entities_pos: Vec<Vector2Int>){
    for entity_position in entities_pos {
        println!("NPC: Starting position = {:?}", entity_position);
        spawn_npc(world, entity_position);
        break;  //DEBUG
    }
}

fn spawn_npc(world: &mut World, npc_spawning_position: Vector2Int
){
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..100);

    let piece:Piece;
    let stats: Stats;
    if rand <= NPC_CHANCE_TO_BE_RANGED {
        //ranged
        piece = Piece{kind: Kind::GhoulRanged};
        stats = Stats {
            strength: 1,
            agility: 5,
            logic: 2,
            melee: 0,
            firearms: 2,
        };
    } else {
        piece = Piece{kind: Kind::Ghoul};
        stats = Stats {
            strength: 2,
            agility: 4,
            logic: 1,
            melee: 3,
            firearms: 0,
        };
    };
    let mut npc = world.spawn(CharacterBundle {
        piece: piece,
        name: Name::new(format!("Ghoul")),
        stats: stats,
        health: Health { max: 10, current: 10 },
        position: BoardPosition{ v:npc_spawning_position },
        occupier: Occupier,
    });

    npc
    .insert(Npc)
    .insert(Walk)
    .insert(Melee)
    ;
    if rand <= NPC_CHANCE_TO_BE_RANGED { 
        npc.insert(Ranged);
    }

    println!("Npc created");
}


pub fn create_exit_map(world: &mut World, exit_position: Vector2Int){
    let mut exit = world.spawn_empty();
    exit 
    .insert(Name::new(format!("Exit")))
    .insert(ExitMapTile)
    .insert(BoardPosition{ v:exit_position});
info!("Exit map created");
}


pub fn spawn_npc_marker(
    commands: &mut Commands,
    //mut ev_spawn_marker: EventReader<SpawnMarkerEvent>,
    graph_assets: &GraphicsAssets,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    entity: Entity,
    position: Vector2Int
) -> Entity {
 //for event in ev_spawn_marker.read() {
    let texture = graph_assets.textures[SPRITE_PLAYER_HUMAN].clone();
    let translation= get_world_position(&position); 
    let order_z = ORDER_MARKER;
    let visibility = Visibility::Visible;
    let sprite = Sprite {
        color: Color::Rgba { red:0.5, green:0.5, blue:0.5, alpha:0.5 },
        ..default()
    };

    let marker = commands.spawn((
         SpriteBundle {
             transform: Transform {
                 translation: Vec3::new(translation.0, translation.1, order_z),
                 scale: Vec3::splat(1.0),
                 ..default()
             },
             texture,
             sprite: sprite,
             visibility: visibility,
             ..default()
         },
         Marker { marked_id: entity },
         GameElement,
     )).id();
    
    commands.entity(marker).insert(BoardPosition {v: position});

    return marker
 }

 pub fn create_nodes(
    world: &mut World, 
    nodes_list: Option<Vec<Vector2Int>>,
 ) {
    match nodes_list {
        Some(nodes) => {
            for node in nodes {        
                let mut nod = world.spawn_empty();
                nod.insert(NavigationNode);
                nod.insert(BoardPosition { v: node });
            }
        },
        None => {},
    }
 }