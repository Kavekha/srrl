use std::cmp;
use bresenham::Bresenham;

use bevy::{prelude::*, utils::HashSet};

use crate::{ game::{pieces::components:: Occupier, player::Player, tileboard::components::{BoardPosition, Tile}}, map_builders::map::Map, vectors::Vector2Int};

use super::components::{ChangeTileVisibility, ChangeTileVisibilityStatus, View};

 
 // 0.20f
 fn get_tiles_around_range_obstacles_break_view(  
    x: i32, 
    y: i32,
    range: i32,
    map_width: i32, // map width -1
    map_height: i32,  // map height -1
    obstacle_position_list: &HashSet< Vector2Int>

 ) -> Vec<Vector2Int> {
    // Bug fix dégueu dû au fait que brehensam ignore le dernier element : si je demande de x,y vers x+10,y+10, je n'aurai de reponse que jusqu'à x+9,y+9.
    let range = range +1;

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
    for border_x in min_x+1..max_x{
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

    // REMINDER: Une Bresenham::new le start est pris en compte mais pas la fin.
    // Pour repondre à ça on augmente le range de 1.
    /* 
    println!("DEBUG: J'ai 40,22 je fais une bresenham vers 37,19");
    for (pos_x, pos_y) in Bresenham::new((40 as isize, 22 as isize), (37 as isize, 19 as isize)) {
        println!("{:?},{:?}", pos_x, pos_y);
    }
    println!("fin");
    ==> 
        DEBUG: J'ai 40,22 je fais une bresenham vers 37,19
        40,22
        39,21
        38,20
        fin
    */    
    for vector in borders {
        for (pos_x, pos_y) in Bresenham::new((x.try_into().unwrap(), y.try_into().unwrap()), (vector.x.try_into().unwrap(), vector.y.try_into().unwrap())) {              
            if obstacle_position_list.contains(&Vector2Int { x: pos_x as i32, y: pos_y as i32}) {
                break;
            }  
            tiles_around_range.push(Vector2Int { x: pos_x as i32, y: pos_y as i32});         
        }
    } 
    tiles_around_range.sort();
    tiles_around_range.dedup();

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
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Hidden, visibility: 0, hidden: 0 } );
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
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Visible, visibility: 0, hidden: 0  } );
                }
            }
        }
        // On mets la nouvelle view.
        view.visible_tiles = current_view;
    }
 }



// DEBUG : On affiche les tuiles autour, sans aucune contrainte.
// 0.20c : Recupère les tuiles autour du personnage, en accord avec le range donné.
// NOTE: Ne se préocupe pas des obstacles pour le moment.
// BUG : Range de 10; Gauche et haut donnent bien -10 par rapport à ma position. Bas / Droite donnent seulement +9. TopLeft donne +10 les autres +9.
// ==> OEUF CORSE. Range a...b : a inclusif, b exclusif...
// Check with 0.20g => OK pour le Range respecté des deux cotés.
fn get_tiles_around_range(
    x: i32, 
    y: i32,
    range: i32,
    max_x: i32, // map width
    max_y: i32,  // map height,
    obstacle_position_list: &HashSet< Vector2Int>       // Non utilisé.
 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    for x in (cmp::max(x - range, 0))..(cmp::min(x + range, max_x) +1) {
        for y in (cmp::max(y - range, 0))..(cmp::min(y + range, max_y) +1) {
            tiles_around_range.push(Vector2Int {x, y} )
        }
    }
    return tiles_around_range
 }


 // DEBUG : On cache les obstacles de la vue, mais on ne prends pas en compte la ligne de vue : ce qui est derrière est affiché.
 // 0.20f : Rework de get_tiles_around_range, n'affiche pas les obstacles. Pas utile, methode de debug.
 // Check with 0.20g => OK pour le Range respecté des deux cotés.
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


 // Original version. Remplacée par update_character_view_with_blocked
 // 0.20d visibility system with component. Only works for Logic Tile.     
 /* 
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
*/