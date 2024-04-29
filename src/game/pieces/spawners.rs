use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{
    commons::get_world_position, engine::asset_loaders::GraphicsAssets, game::{
        tileboard::components::{BoardPosition, ExitMapTile}, visibility::components::Marker
    }, globals::{ORDER_MARKER, SPRITE_MARKER}, vectors::Vector2Int};

use super::components::{GameElement, NavigationNode};


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
    let texture = graph_assets.textures[SPRITE_MARKER].clone();
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