// 0.20n v0.9 : Séparation en deux fonctions: conversion logic > render, et traitement du render.
// 0.20n v0.8 : On considère comme "hide" une render_tile dont les autres tuiles logiques qu'elles représentent sont non visibles.
// 0.20i v0.7 : on ne Hide plus la tuile, on change sa couleur. // REMEMBER : Si d'autres jouent avec la couleur, ca va foutre la merde.
// 0.20h v0.6
// Cas #0 : Range 0 tile = On ne voit rien, pas même la place du joueur. => Non, on voit le joueur + 0.5
// Cas #1 : Range 1 tile = placé sur le joueur. => Non, on voit la tile joueur
// Cas #2 : Range 1 visibility => Est-ce que les bonnes tiles logiques sont marquées comme visibles? Oui. J'en ai 9.
// Cas #2b : Range 1 visibility => les bonnes tiles logiques sont marquées comme devant rester visibles? Oui.
// Cas #2c : Range 1 visibility => Quand je me deplace, les cases qui ne sont plus visibles sont bien signalées comme hidden? Oui.


use bevy::{prelude::*, utils::HashMap};

use crate::{engine::render::components::GameMapRender, game::{pieces::components::Npc, tileboard::components::{BoardPosition, Tile}}, vectors::Vector2Int};

use super::components::{ChangeVisibility, ChangeVisibilityStatus, View};


 // RENDER_SW corresponds à 0,0.
 const RENDER_SW:(i32, i32) = (0, 0);
 const RENDER_NW:(i32, i32) = (0, -1);
 const RENDER_NE:(i32, i32) = (1, -1);
 const RENDER_SE:(i32, i32) = (1, 0);

const LOGIC_TILE_E:(i32, i32) = (1, 0);
const LOGIC_TILE_SE:(i32, i32) = (1, 1);
const LOGIC_TILE_S:(i32, i32) = (0, 1);
const LOGIC_TILE_SW:(i32, i32) = (-1, 1);
const LOGIC_TILE_W:(i32, i32) = (1, 0);
const LOGIC_TILE_NW:(i32, i32) = (1, -1);
const LOGIC_TILE_N:(i32, i32) = (0, -1);
const LOGIC_TILE_NE:(i32, i32) = (1, -1);

const RENDER_SW_COVER:[(i32,i32); 3] = [LOGIC_TILE_SW, LOGIC_TILE_S, LOGIC_TILE_W];
const RENDER_NW_COVER:[(i32,i32); 3] = [LOGIC_TILE_NW, LOGIC_TILE_N, LOGIC_TILE_W];
const RENDER_NE_COVER:[(i32,i32); 3] = [LOGIC_TILE_NE, LOGIC_TILE_N, LOGIC_TILE_E];
const RENDER_SE_COVER:[(i32,i32); 3] = [LOGIC_TILE_SE, LOGIC_TILE_S, LOGIC_TILE_E];


#[derive(Component)]
pub struct RenderVisibilityTile {
    pub visibility_score: i32
}


// Cette fonction prends les tuiles logiques dont la visibilité doit changer, et fait les calculs pour determiner quelles tuiles render sont concernées.
pub fn update_convert_logic_tile_visibility_to_render(
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    view_q: Query<&View>,
) {
    let Ok(game_map_render) = game_map_render_q.get_single() else { return; };
    
    let mut component_to_delete = Vec::new();
        
    // Je recupère les entités logiques
    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        component_to_delete.push(entity);
        // Je recupere le nouveau statut.
        let mut visible_status;
        match new_visibility.new_status {
            ChangeVisibilityStatus::Visible => visible_status = 1,
            ChangeVisibilityStatus::Hidden => visible_status = -1,
        }

        let view = view_q.single();
        // Si visible, on regarde pour chaque Tuile graphique si les autres tuiles logiques qu'elles couvrent sont visibles.
        // Sinon, on a le comportement habituel.
        // SW
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_SW_COVER {
                if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_SW.0, y:position.v.y + RENDER_SW.1 }) {
            commands.entity(* render_tile_floor_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_SW.0, y:position.v.y + RENDER_SW.1 }) {
            commands.entity(* render_tile_wall_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }
        //RENDER_NW
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_NW_COVER {
                if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_NW.0, y:position.v.y + RENDER_NW.1 }) {
            commands.entity(* render_tile_floor_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_NW.0, y:position.v.y + RENDER_NW.1 }) {
            commands.entity(* render_tile_wall_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }

        //RENDER_NE        
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_NE_COVER {
                if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_NE.0, y:position.v.y + RENDER_NE.1 }) {
            commands.entity(* render_tile_floor_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_NE.0, y:position.v.y + RENDER_NE.1 }) {
            commands.entity(* render_tile_wall_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }

        //RENDER_SE
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_SE_COVER {
                if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_SE.0, y:position.v.y + RENDER_SE.1 }) {
            commands.entity(* render_tile_floor_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_SE.0, y:position.v.y + RENDER_SE.1 }) {
            commands.entity(* render_tile_wall_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
        }
    }

    for entity in component_to_delete {
        commands.entity(entity).remove::<ChangeVisibility>();
    }
}




// v 0.8b Cette fonctionnalité prends les tuiles render précédemment marquées, et les mets à jour.
pub fn update_tile_visibility_render(
    mut commands: Commands,
    render_visibility_q: Query<(Entity, &RenderVisibilityTile)>,    
    mut visibility_q: Query<&mut Visibility>,
    mut sprite_q: Query<&mut Sprite>,  
 ){
    let mut component_to_delete = Vec::new();
    for (entity, render_tile) in render_visibility_q.iter() {
        if let Ok(mut visibility) = visibility_q.get_mut(entity) {
            * visibility = Visibility::Visible;
            // Hidden but known
            if render_tile.visibility_score < 0 {
                if let Ok(mut sprite) = sprite_q.get_mut(entity){
                    sprite.color.set_a(0.5);
                    sprite.color.set_r(0.5);
                    sprite.color.set_g(0.5);
                    sprite.color.set_b(0.5);
                }
            // visible
            } else {
                if let Ok(mut sprite) = sprite_q.get_mut(entity){
                    sprite.color.set_a(1.0);
                    sprite.color.set_r(1.0);
                    sprite.color.set_g(1.0);
                    sprite.color.set_b(1.0);
                }
            }
        }
        component_to_delete.push(entity);
    }
    for entity in component_to_delete {
        commands.entity(entity).remove::<RenderVisibilityTile>();
    }
}




pub fn update_tile_visibility_render_v8a(
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut sprite_q: Query<&mut Sprite>,
    mut visibility_q: Query<&mut Visibility>,
    view_q: Query<&View>,
 ){
    let Ok(game_map_render) = game_map_render_q.get_single() else { return; };
    
    let mut component_to_delete = Vec::new();
    let mut entity_change_status = HashMap::new();
    
    // Je recupère les entités logiques
    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        component_to_delete.push(entity);
          // Je recupere le nouveau statut.
        let mut visible_status;
         match new_visibility.new_status {
            ChangeVisibilityStatus::Visible => visible_status = 1,
            ChangeVisibilityStatus::Hidden => visible_status = -1,
        }

        let view = view_q.single();
        // Si visible, on regarde pour chaque Tuile graphique si les autres tuiles logiques qu'elles couvrent sont visibles.
        // Sinon, on a le comportement habituel.
        // SW
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_SW_COVER {
                   if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_SW.0, y:position.v.y + RENDER_SW.1 }) {
            if let Some(render_tile_floor_entity_entry) = entity_change_status.get_mut(render_tile_floor_entity) {
                *render_tile_floor_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_floor_entity, visible_status);
            }
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_SW.0, y:position.v.y + RENDER_SW.1 }) {
            if let Some(render_tile_wall_entity_entry) = entity_change_status.get_mut(render_tile_wall_entity) {
                *render_tile_wall_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_wall_entity, visible_status);                
            }
        }

        //RENDER_NW
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_NW_COVER {
                   if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_NW.0, y:position.v.y + RENDER_NW.1 }) {
            if let Some(render_tile_floor_entity_entry) = entity_change_status.get_mut(render_tile_floor_entity) {
                *render_tile_floor_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_floor_entity, visible_status);
            }
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_NW.0, y:position.v.y + RENDER_NW.1 }) {
            if let Some(render_tile_wall_entity_entry) = entity_change_status.get_mut(render_tile_wall_entity) {
                *render_tile_wall_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_wall_entity, visible_status);                
            }
        }

        //RENDER_NE        
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_NE_COVER {
                   if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_NE.0, y:position.v.y + RENDER_NE.1 }) {
            if let Some(render_tile_floor_entity_entry) = entity_change_status.get_mut(render_tile_floor_entity) {
                *render_tile_floor_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_floor_entity, visible_status);
            }
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_NE.0, y:position.v.y + RENDER_NE.1 }) {
            if let Some(render_tile_wall_entity_entry) = entity_change_status.get_mut(render_tile_wall_entity) {
                *render_tile_wall_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_wall_entity, visible_status);                
            }
        }

        //RENDER_SE
        if visible_status > 0 {
            let mut score = 0;
            for logic_tile_around in RENDER_SE_COVER {
                   if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                    score += 1;
                    break;
                }
            }
            if score == 0 {
                visible_status = -1;    // Elle sera "dans l'obscurité".
            }
        }
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + RENDER_SE.0, y:position.v.y + RENDER_SE.1 }) {
            if let Some(render_tile_floor_entity_entry) = entity_change_status.get_mut(render_tile_floor_entity) {
                *render_tile_floor_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_floor_entity, visible_status);
            }
        }
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + RENDER_SE.0, y:position.v.y + RENDER_SE.1 }) {
            if let Some(render_tile_wall_entity_entry) = entity_change_status.get_mut(render_tile_wall_entity) {
                *render_tile_wall_entity_entry += visible_status;
            } else {
                entity_change_status.insert(render_tile_wall_entity, visible_status);                
            }
        }
    }
    for (entity, score) in entity_change_status {
        if let Ok(mut visibility) = visibility_q.get_mut(*entity) {
            * visibility = Visibility::Visible;
            // Hidden but known
            if score < 0 {
                if let Ok(mut sprite) = sprite_q.get_mut(*entity){
                    sprite.color.set_a(0.5);
                    sprite.color.set_r(0.5);
                    sprite.color.set_g(0.5);
                    sprite.color.set_b(0.5);
                }
            // visible
            } else {
                if let Ok(mut sprite) = sprite_q.get_mut(*entity){
                    sprite.color.set_a(1.0);
                    sprite.color.set_r(1.0);
                    sprite.color.set_g(1.0);
                    sprite.color.set_b(1.0);
                }
            }
        }
    }
    for entity in component_to_delete {
        commands.entity(entity).remove::<ChangeVisibility>();
    }
 }



 // 0.20k L'ordre de se cacher vient du system view.
// 0.20e ici on modifie l'affichage. L'intelligence "Je suis pas visible" va dans les autres systèmes.
pub fn update_npc_visibility_status(
    mut npc_visibility_q: Query<&mut Visibility, With<Npc>>,
    npc_with_change_order_q: Query<(Entity, &ChangeVisibility), With<Npc>>,
){
    for (entity, new_visibility) in npc_with_change_order_q.iter() {           
        let Ok(mut npc_visibility) = npc_visibility_q.get_mut(entity) else { continue };
        match new_visibility.new_status {
            ChangeVisibilityStatus::Visible => *npc_visibility = Visibility::Visible,
            ChangeVisibilityStatus::Hidden => *npc_visibility = Visibility::Hidden,
        }         
    }
 }
 
