// ==> CONCEPTION 0.20h
/*
La View du joueur est devenue sale => Il faut mettre à jour.   <== C'est le role de update_character_view_with_blocked.

J'ai la liste des tuiles logiques dans sa view.
Je determine celles que je dois mettre à jour.  <== Ce n'est pas le role de update_character_view_with_blocked, mais il le fait aujourd'hui.

Une nouvelle fonction doit prendre la main.
A partir des tuiles logiques devant être mise à jour, on determine quelles tuiles render doivent être changées et on gère les doublons avec des ordres contradictoires.
On peut partir sur le principe du Mask de la DualGrid => wall_corners

Je suis la tile logique 10,10 et je dois être visible.
Mes équivalents render sont 25% de : 
    let ne = (x, y) = (10,10)
    let se = (x, y + 1) = (10,11);
    let sw = (x - 1, y + 1) = (9,11);
    let nw = (x - 1, y) = (9,10);

Comme on doit plutot s'interresser aux angles, on va plutot regarder les tuiles logiques diagonales.
Si la tile logique NW (x -1, y -1) est hidden, alors la tuile render NW (x -1, y) de la tuile logique sera hidden.
Si la tile logique SW (x -1, y +1) est hidden, alors la tuile render SW (x -1, y +1) de la tuile logique sera hidden.
Si la tile logique NE (x +1, y -1) est hidden, alors la tuile render NE (x,y) de la tuile logique sera hidden.
Si la tile logique SE (x +1, y +1) est visible, alors seule la tuile render SE (x, y +1) de la tuile logique sera visible. 
==> En representation, cela donnera un angle "tournant".

Sauf que d'autres tuiles peuvent donner des informations contradictoires.
On peut alors donner un score de visibilité à chaque tile render. Si à 1+ alors visible, si à 0- alors Hidden. si au moins un visible => HiddenKnown.
    */

use std::cmp;

use bevy::{prelude::*, utils::{HashMap, HashSet}};

use crate::{engine::render::components::GameMapRender, game::{pieces::components::Npc, player::Player, tileboard::components::{BoardPosition, Tile}}, map_builders::map::Map, vectors::Vector2Int};

use super::components::{ChangeTileVisibility, ChangeTileVisibilityStatus, View};



 // 0.20c Get Entity from game_map_render pour floor ou wall.
 fn get_floor_entity_at(
    game_map_render:&GameMapRender,
    x: i32,
    y: i32
 ) -> Option<&Entity> {    
    if game_map_render.floor.contains_key(&Vector2Int {x, y}) {
        let option_entity_floor = game_map_render.floor.get(&Vector2Int {x, y});
        option_entity_floor
    } else {
        None
    }
 }

 fn get_wall_entity_at(
    game_map_render:&GameMapRender,
    x: i32,
    y: i32
 ) -> Option<&Entity> {    
    if game_map_render.wall.contains_key(&Vector2Int {x, y}) {
        let option_entity_wall = game_map_render.wall.get(&Vector2Int {x, y});
        option_entity_wall
    } else {
        None
    }
 }

// 0.20h Revision: mise à jour des tiles render.
pub fn update_tile_visibility_render(
    board: Res<Map>,
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut visibility_q: Query<&mut Visibility>,
 ){




    let game_map_render = game_map_render_q.single();
    let mut to_remove = Vec::new();
    let mut tiles_to_change = HashMap::new();   // On place les infos dans ce Hashmap pour eviter les doublons de vector.

    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        //info!("I have to render the following tiles : {:?}", position.v);
        // Une tuile logique = 25% d'une tuile graphique x 4 dû au DualGrid. La position x,y corresponds à la partie Nord Ouest de la tuile logique.
        // Il faut donc aussi traiter x-+1,y ; x, y+1 ; x+1,y+1. On ne doit pas depasser le board non plus.
        // ===> En fait seul { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y} permets d'avoir un affichage propre et ok avec le range. 
        // Le reste semble s'overdriver et s'écraser mutuellement.
        
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: position.v.x }, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y}, new_visibility);

        to_remove.push(entity);
    }

    for (tile_position, new_visibility) in tiles_to_change {
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_floor = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_floor = Visibility::Hidden,
                    ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_wall = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_wall = Visibility::Hidden,
                    ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
    }       
    for entity in to_remove {
        commands.entity(entity).remove::<ChangeTileVisibility>();
    }
 }

// 0.20d mise à jour des tiles render.
pub fn update_tile_visibility_render_older(
    board: Res<Map>,
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut visibility_q: Query<&mut Visibility>,
 ){
    let game_map_render = game_map_render_q.single();
    let mut to_remove = Vec::new();
    let mut tiles_to_change = HashMap::new();   // On place les infos dans ce Hashmap pour eviter les doublons de vector.

    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        //info!("I have to render the following tiles : {:?}", position.v);
        // Une tuile logique = 25% d'une tuile graphique x 4 dû au DualGrid. La position x,y corresponds à la partie Nord Ouest de la tuile logique.
        // Il faut donc aussi traiter x-+1,y ; x, y+1 ; x+1,y+1. On ne doit pas depasser le board non plus.
        // ===> En fait seul { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y} permets d'avoir un affichage propre et ok avec le range. 
        // Le reste semble s'overdriver et s'écraser mutuellement.
        
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: position.v.x }, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y}, new_visibility);

        to_remove.push(entity);
    }

    for (tile_position, new_visibility) in tiles_to_change {
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_floor = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_floor = Visibility::Hidden,
                    ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_wall = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_wall = Visibility::Hidden,
                    ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
    }       
    for entity in to_remove {
        commands.entity(entity).remove::<ChangeTileVisibility>();
    }
 }
 
  // 0.20e ici on modifie l'affichage. L'intelligence "Je suis pas visible" va dans les autres systèmes.
  pub fn update_npc_visibility_status(
    player_view_q: Query<&View, With<Player>>,
    npc_position_q: Query<(Entity, &BoardPosition), With <Npc>>,
    mut npc_visibility_q: Query<&mut Visibility, With<Npc>>,
 ){
    for view in player_view_q.iter() {
        let all_npc_positions:&HashSet<(Entity, Vector2Int)> = &npc_position_q.iter().map(|(npc_entity, npc_position)| (npc_entity, npc_position.v)).collect();
        
        //info!("My view is : {:?}", view.visible_tiles);
        for (entity, position) in all_npc_positions{
            let Ok(mut npc_visibility) = npc_visibility_q.get_mut(*entity) else { continue };
            if view.visible_tiles.contains(position) {
                //info!("Entity {:?} is in my view at {:?}", entity, position);                
                *npc_visibility = Visibility::Visible;
            } else {
                //info!("Entity {:?} is not in view sight, because at {:?}", entity, position);
                *npc_visibility = Visibility::Hidden;
            }            
        }
    }
 }
 
