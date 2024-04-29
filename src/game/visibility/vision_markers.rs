use bevy::prelude::*;

use crate::{
    commons::get_world_position, engine::asset_loaders::GraphicsAssets, 
    game::{game_generation::character_creation::components::GameElement, tileboard::components::BoardPosition, visibility::components::{Marked, Marker}}, 
    globals::{ORDER_MARKER, SPRITE_MARKER}, vectors::Vector2Int};

use super::components::{HasBeenSeenEvent, OutOfSightEvent, View};




pub fn remove_markers_when_seen(
    mut commands: Commands,
    view_q: Query<&View>,
    marker_position_q: Query<(Entity, &BoardPosition, &Marker)>,
){
    let Ok(view) = view_q.get_single() else { return };
    let mut to_remove = Vec::new();
    for (entity, marker_position, marker) in marker_position_q.iter() {
        if view.visible_tiles.contains(&marker_position.v) {
            commands.entity(marker.marked_id).remove::<Marked>();   //("Can't found a marked entity for this marker.");
            to_remove.push(entity);
            println!("Un marqueur est retiré.");
        }
    }
    for entity in to_remove {
        commands.entity(entity).despawn_recursive();    // Petite entité n'a plus de raison d'exister.
    }
}

pub fn remove_markers_when_marked_is_seen(
    mut commands: Commands,
    mut ev_has_been_seen: EventReader<HasBeenSeenEvent>,
    marked_q : Query<&Marked>,  
    marker_q: Query<&Marker> 
) {
    info!("remove markers: a HasBeenSeenEvent has been received.");
    for event in ev_has_been_seen.read() {
        info!("{:?} has been seen by {:?}", event.entity, event.saw_by);
        if let Ok(marked) = marked_q.get(event.entity) {
            info!("{:?} is marked with marker {:?}.", event.entity, marked.marker_id);
            if let Ok(_marker_still_exist) = marker_q.get(marked.marker_id) {
                // Des cas existent où le marker est déjà detruit, ce qui provoque un panic.
                commands.entity(marked.marker_id).despawn_recursive();    // On efface le Marker.
            }  
        } else { continue };        
    }
}

pub fn put_markers_when_out_of_sight(
    mut commands: Commands,
    mut ev_out_of_sight: EventReader<OutOfSightEvent>,  
    position_q: Query<&BoardPosition>,
    graph_assets: ResMut<GraphicsAssets>,
) {
    for event in ev_out_of_sight.read() {
        println!("{:?} is out of sight : Leave a Marker.", event.entity);
        if let Ok(position) = position_q.get(event.entity) {
            let marker = spawn_npc_marker(&mut commands, &graph_assets, event.entity, position.v);
            commands.entity(event.entity).insert(Marked { marker_id : marker });
        }        
    }
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