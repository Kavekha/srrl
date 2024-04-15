use std::cmp;
use bresenham::Bresenham;

use bevy::{prelude::*, utils::{HashMap, HashSet}};

use crate::{engine::render::components::GameMapRender, game::{pieces::components::{Npc, Occupier}, player::Player, tileboard::components::{BoardPosition, Tile}}, map_builders::map::Map, vectors::Vector2Int};

use super::components::{ChangeTileVisibility, ChangeTileVisibilityStatus, View};


// 0.20c : Recupère les tuiles autour du personnage, en accord avec le range donné.
// NOTE: Ne se préocupe pas des obstacles pour le moment.
// BUG : Range de 10; Gauche et haut donnent bien -10 par rapport à ma position. Bas / Droite donnent seulement +9. TopLeft donne +10 les autres +9.
// ==> OEUF CORSE. Range a...b : a inclusif, b exclusif...
fn get_tiles_around_range(
    x: i32, 
    y: i32,
    range: i32,
    max_x: i32, // map width
    max_y: i32  // map height
 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    for x in (cmp::max(x - range, 0))..(cmp::min(x + range, max_x) +1) {
        for y in (cmp::max(y - range, 0))..(cmp::min(y + range, max_y) +1) {
            tiles_around_range.push(Vector2Int {x, y} )
        }
    }
    return tiles_around_range
 }

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

 
 // 0.20d mise à jour des tiles render.
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

 // 0.20f
 fn get_tiles_around_range_obstacles_break_view(  
    x: i32, 
    y: i32,
    range: i32,
    map_width: i32, // map width -1
    map_height: i32,  // map height -1
    obstacle_position_list: &HashSet< Vector2Int>

 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    // On part du centre - le perso - et on regarde chaque tuile du bord du cadre.
    let min_x = cmp::max(x - range, 0);
    let max_x = cmp::min(x + range, map_width);
    let min_y = cmp::max(y - range, 0);
    let max_y = cmp::min(y + range, map_height);
  
    let mut borders = Vec::new();
    // On ajoute les 4 angles d'abord.
    borders.push(Vector2Int { x: min_x, y: min_y });
    borders.push(Vector2Int { x: max_x, y: min_y });
    borders.push(Vector2Int { x: max_x, y: max_y });
    borders.push(Vector2Int { x: min_x, y: max_y });

    //Rangée top  : (x entre min x et max x),  min y    // On devrait faire min_x+1..max_x-1 si a..b avait b inclusif, mais b est exclusif dans un for i in a..b.
    for border_x in min_x+1..max_x {
        borders.push(Vector2Int { x: border_x, y: min_y})
    }
    // Rangée bottom : (x entre min x et max x), max y 
    for border_x in min_x+1..max_x {
        borders.push(Vector2Int { x: border_x, y: max_y})
    }
    //Rangée left : min x, (y entre min y et max y)
    for border_y in min_y+1..max_y {
        borders.push(Vector2Int { x: min_x, y: border_y})
    }
    //Rangée right : max x, y entre min y et max y 
    for border_y in min_y+1..max_y {
        borders.push(Vector2Int { x: max_x, y: border_y})
    }  

    //println!("VIEW BLOCKED : x {x}, y {y}");
    for vector in borders {
        //println!("On regarde la bordure {:?}", vector);
        for (pos_x, pos_y) in Bresenham::new((x.try_into().unwrap(), y.try_into().unwrap()), (vector.x.try_into().unwrap(), vector.y.try_into().unwrap())) {
            //println!("On intègre le vector {:?}", vector); 
            tiles_around_range.push(Vector2Int { x: pos_x as i32, y: pos_y as i32});
            if obstacle_position_list.contains(&Vector2Int { x: pos_x as i32, y: pos_y as i32}) {
                break;
            }
        }
    } 
    tiles_around_range.sort();
    tiles_around_range.dedup();

    return tiles_around_range
 }


 // 0.20f : Rework de get_tiles_around_range, n'affiche pas les obstacles. Pas utile, methode de debug.
fn get_tiles_around_range_with_obstacles(
    x: i32, 
    y: i32,
    range: i32,
    max_x: i32, // map width
    max_y: i32,  // map height
    obstacle_position_list: &HashSet< Vector2Int>

 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    // Rappel : for x in a..b, b est exclusif.
    for x in (cmp::max(x - range, 0))..(cmp::min(x + range, max_x) +1) {
        for y in (cmp::max(y - range, 0))..(cmp::min(y + range, max_y) +1) {
            if !obstacle_position_list.contains(&Vector2Int { x, y}) {
                tiles_around_range.push(Vector2Int {x, y} )
            }  
        }
    }
    return tiles_around_range
 }

 // 0.20f
 pub fn update_character_view_with_blocked(
    mut commands: Commands,
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    board: Res<Map>,
    occupied_tiles_q: Query<&BoardPosition, (With<Occupier>, With<Tile>)>,
    //occupied_tiles_q: Query<(&Occupier, &Tile)>,
 ) {
    for ( mut view, board_position) in player_view_q.iter_mut() {
        let all_wall_position:&HashSet< Vector2Int> = &occupied_tiles_q.iter().map(|tile_position| tile_position.v).collect();
        let mut view_to_treat = get_tiles_around_range_obstacles_break_view(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1, all_wall_position);
        //println!("WITH OBSTACLE : {:?}", view_to_treat);

        let mut current_view: Vec<Vector2Int> = Vec::new();
        let mut to_hide: Vec<Vector2Int> = Vec::new();
        let mut treated: Vec<Vector2Int> = Vec::new();

        // On pop chaque element de view.visible_tiles et on regarde si présente dans view_to_treat.
        // Si c'est le cas, elle reste visible, on l'ajoute à current_view et on la retire à view_to_treat. Sinon, on la hide.
        // A la fin on prends chaque element restant dans view_to_treat et on les passe en visible, et on les ajoute à current_view.
        for eval_tile in view.visible_tiles.iter() {
            if view_to_treat.contains(&eval_tile) {
                current_view.push(*eval_tile);  // Deja visible.
            } else {
                to_hide.push(*eval_tile);   // A rendre invisible.
            }
            treated.push(*eval_tile);   // Est ce que to_hide garde son contenu après deferencement? // TOLEARN
        }

        // Rendre invisible.
        for hiden_tile in to_hide.iter() {
            if board.entity_tiles.contains_key(&Vector2Int {x: hiden_tile.x, y: hiden_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: hiden_tile.x, y: hiden_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Hidden } );
                }
            }
        }
        // On retire de view to treat tous les elements déjà traités, qui etait dans la view.visible_tiles. Ces elements doivent être passé à visible.
        view_to_treat = view_to_treat.iter().filter_map(|val|{
            if treated.contains(val) {
                return None
            }
            Some(*val)
        }).collect();
        //info!("Here, I have removed treated from view_to_treat. I have now in view_to_treat: {:?}", view_to_treat);

        for visible_tile in view_to_treat.iter() {
            current_view.push(*visible_tile);
            //rendre visible.            
            if board.entity_tiles.contains_key(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Visible } );
                }
            }
        }
        // On mets la nouvelle view.
        view.visible_tiles = current_view;
    }
 }

 // 0.20d visibility system with component. Only works for Logic Tile.
pub fn update_character_view(
    mut commands: Commands,
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    board: Res<Map>,
 ){
    // Pour utiliser get_tiles_around_range_with_obstacles, qui n'affiche pas les tiles Obstacles.
    //let all_wall_position:&HashSet< Vector2Int> = &occupied_tiles_q.iter().map(|tile_position| tile_position.v).collect();
        
    for ( mut view, board_position) in player_view_q.iter_mut() {
        // Pour utiliser get_tiles_around_range_with_obstacles
        //let mut view_to_treat = get_tiles_around_range_with_obstacles(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1, all_wall_position);

        let mut view_to_treat = get_tiles_around_range(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1);
        //info!("My view to treat is : {:?}", view_to_treat);

        let mut current_view: Vec<Vector2Int> = Vec::new();
        let mut to_hide: Vec<Vector2Int> = Vec::new();
        let mut treated: Vec<Vector2Int> = Vec::new();

        // On pop chaque element de view.visible_tiles et on regarde si présente dans view_to_treat.
        // Si c'est le cas, elle reste visible, on l'ajoute à current_view et on la retire à view_to_treat. Sinon, on la hide.
        // A la fin on prends chaque element restant dans view_to_treat et on les passe en visible, et on les ajoute à current_view.
        for eval_tile in view.visible_tiles.iter() {
            if view_to_treat.contains(&eval_tile) {
                current_view.push(*eval_tile);  // Deja visible.
            } else {
                to_hide.push(*eval_tile);   // A rendre invisible.
            }
            treated.push(*eval_tile);   // Est ce que to_hide garde son contenu après deferencement? // TOLEARN
        }

        // Rendre invisible.
        for hiden_tile in to_hide.iter() {
            if board.entity_tiles.contains_key(&Vector2Int {x: hiden_tile.x, y: hiden_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: hiden_tile.x, y: hiden_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Hidden } );
                }
            }
        }
        // On retire de view to treat tous les elements déjà traités, qui etait dans la view.visible_tiles. Ces elements doivent être passé à visible.
        view_to_treat = view_to_treat.iter().filter_map(|val|{
            if treated.contains(val) {
                return None
            }
            Some(*val)
        }).collect();
        //info!("Here, I have removed treated from view_to_treat. I have now in view_to_treat: {:?}", view_to_treat);

        for visible_tile in view_to_treat.iter() {
            current_view.push(*visible_tile);
            //rendre visible.
            if board.entity_tiles.contains_key(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Visible } );
                }
            }
        }
        // On mets la nouvelle view.
        view.visible_tiles = current_view;
    }
}
