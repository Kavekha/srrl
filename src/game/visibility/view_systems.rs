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


// Note: refacto possible en passant par un HashMap plutot que X list, contenant Entity : VisibilityStatus.
 // 0.20i : On devrait gèrer le reveal_tiles qui se trouve dans Map pour le moment. Mais vu que tout est Hidden par defaut, on ne fait jamais de retour à Hidden après avoir vu logiquement.
 // v0.5 (0.20h) On simplifie la methode de traitement car faire trop subtil ne marche pas bien avec la logique des 4 1/4 de tuiles en Dual Grid.
 // Avant: on ne remettait pas le statut "visible" des tiles qui restaient visibles. 
 // => C'etait bien, mais comme les tiles graphiques se chevauchent eteindre les 4 tuiles eteignaient 2 tuiles de logic tile restées visibles.
 pub fn update_character_view_with_blocked(
    mut commands: Commands,
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    board: Res<Map>,
    occupied_tiles_q: Query<&BoardPosition, (With<Occupier>, With<Tile>)>,
    //occupied_tiles_q: Query<(&Occupier, &Tile)>,
 ) {
    for ( mut view, board_position) in player_view_q.iter_mut() {
        // La nouvelle vue.
        let all_wall_position:&HashSet< Vector2Int> = &occupied_tiles_q.iter().map(|tile_position| tile_position.v).collect();
        let view_to_treat = get_tiles_around_range_obstacles_break_view(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1, all_wall_position);
        let mut to_hide: Vec<Vector2Int> = Vec::new();
        
        // On pop chaque element de view.visible_tiles et on regarde si présente dans view_to_treat.
        // Si c'est le cas, la tuile reste visible et elle sera traitée plus tard avec les nouvelles de view_to_treat.
        // Si elle n'est plus présente dans la vue, c'est qu'elle etait visible et qu'elle ne doit plus l'être : on la hide.
        for eval_tile in view.visible_tiles.iter() {
            if !view_to_treat.contains(&eval_tile) {
                to_hide.push(*eval_tile);   // A rendre invisible.               
            }
        }
        // Rendre caché.
        for hiden_tile in to_hide.iter() {
            if board.entity_tiles.contains_key(&Vector2Int {x: hiden_tile.x, y: hiden_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: hiden_tile.x, y: hiden_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Hidden } );
                }
            }
        }

        // Rendre visible & ajouter dans la nouvelle vue.
        let mut new_view = Vec::new();
        for visible_tile in view_to_treat.iter() {
            new_view.push(* visible_tile);
            if board.entity_tiles.contains_key(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeTileVisibility { new_status: ChangeTileVisibilityStatus::Visible  } );
                }
            }
        }
        // On mets la nouvelle view.
        view.visible_tiles = new_view;
    }
 }