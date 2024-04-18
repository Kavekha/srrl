use std::cmp;
use bresenham::Bresenham;

use bevy::{prelude::*, utils::HashSet};

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{gamelog::LogEvent, movements::components::{CancelMoveEvent, MoveEvent}, 
    pieces::{components::{Npc, Occupier}, spawners::spawn_npc_marker}, 
    player::Player, 
    tileboard::components::{BoardPosition, Tile}, visibility::components::Marked}, 
    map_builders::map::Map, vectors::Vector2Int};

use super::components::{ChangeVisibility, ChangeVisibilityStatus, HasBeenSeenEvent, Marker, OutOfSightEvent, View};

 
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


 // 0.20i "Si un NPC entre dans mon champ de vision, je suis informé." => Couvert par la fonction principale quand je me deplace, mais pas quand les NPC se deplacent.
 // Celle-ci réponds à ce besoin. 
 // On part d'un "on move event"
 pub fn update_character_view_on_npc_action(
    mut commands: Commands,
    mut ev_move_event: EventReader<MoveEvent>,
    view_q: Query<(Entity, &View), With<Player>>,
    mut ev_log: EventWriter<LogEvent>,
    name_q: Query<&Name>,
    mut ev_has_been_seen: EventWriter<HasBeenSeenEvent>,
 ) {
    for event in ev_move_event.read() {
        //println!("{:?} MoveTo {:?}, je suis à {:?} maintenant.", event.entity, event.previous, event.next);
        for (view_entity, view) in view_q.iter() {
            let mut was_in_view= false;
            let mut now_in_view= false;

            if view.visible_tiles.contains(&event.previous) {
                //println!("{:?} was in view", event.entity);
                was_in_view = true;
            }
            if view.visible_tiles.contains(&event.next) {
                //println!("{:?} is now in view", event.entity);
                now_in_view = true;
            }

            if (was_in_view && now_in_view) || (!was_in_view && !now_in_view) {
                //println!("npc {:?} was and still is in view or never was in view", entity);
                continue;
            } else if was_in_view {
                // Ne l'est plus.
                //println!("npc {:?} is not in view anymore", event.entity);
                commands.entity(event.entity).insert(ChangeVisibility { new_status: ChangeVisibilityStatus::Hidden});
            } else {
                // vu pour la première fois.
                //println!("npc {:?} is now in sight!", event.entity);
                commands.entity(event.entity).insert(ChangeVisibility { new_status: ChangeVisibilityStatus::Visible});
                if let Ok(name) = name_q.get(event.entity) {
                    ev_log.send(LogEvent { entry: format!("{:?} enters your line of view!", name)});
                    ev_has_been_seen.send(HasBeenSeenEvent { entity: event.entity, saw_by : view_entity});

                }
            }
        }
    }
 }


// Note: refacto possible en passant par un HashMap plutot que X list, contenant Entity : VisibilityStatus.
 // 0.20i : On devrait gèrer le reveal_tiles qui se trouve dans Map pour le moment. Mais vu que tout est Hidden par defaut, on ne fait jamais de retour à Hidden après avoir vu logiquement.
 // v0.5 (0.20h) On simplifie la methode de traitement car faire trop subtil ne marche pas bien avec la logique des 4 1/4 de tuiles en Dual Grid.
 // Avant: on ne remettait pas le statut "visible" des tiles qui restaient visibles. 
 // => C'etait bien, mais comme les tiles graphiques se chevauchent eteindre les 4 tuiles eteignaient 2 tuiles de logic tile restées visibles.
 pub fn update_character_view_with_blocked(
    mut commands: Commands,
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    mut board: ResMut<Map>,
    occupied_tiles_q: Query<&BoardPosition, (With<Occupier>, With<Tile>)>,
    npc_position_q: Query<(Entity, &BoardPosition), With <Npc>>,
    mut ev_log: EventWriter<LogEvent>,
    name_q: Query<&Name>,
    mut ev_cancel_move: EventWriter<CancelMoveEvent>,
    player_q: Query<Entity, With<Player>>,
    mut ev_out_of_sight: EventWriter<OutOfSightEvent>,   
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
                    commands.entity(*tile_logic_entity).insert(ChangeVisibility { new_status: ChangeVisibilityStatus::Hidden } );
                }
            }
        }
        // Rendre visible & ajouter dans la nouvelle vue.
        let mut new_view = Vec::new();
        for visible_tile in view_to_treat.iter() {
            new_view.push(* visible_tile);
            if board.entity_tiles.contains_key(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                if let Some(tile_logic_entity) = board.entity_tiles.get(&Vector2Int {x: visible_tile.x, y: visible_tile.y}) {
                    commands.entity(*tile_logic_entity).insert(ChangeVisibility { new_status: ChangeVisibilityStatus::Visible  } );
                }
            }
            board.revealing_tile(visible_tile.x, visible_tile.y);
        }

        // Voyons les NPC à présent!
        // Marche OK quand le PJ se deplace mais pas quand c'est le NPC.
        // C'est parce que quand le NPC se deplace, il entre dans la zone, on check s'il est dans la view actuelle - oui, il vient d'y entrer - et s'il est dans la suivante - oui, il y est evidemment.
        // Un autre systeme doit gerer ça.
        let all_npc_positions:&HashSet<(Entity, Vector2Int)> = &npc_position_q.iter().map(|(npc_entity, npc_position)| (npc_entity, npc_position.v)).collect();
        for (entity, position) in all_npc_positions{
            let mut was_in_view= false;
            let mut now_in_view= false;

            if view.visible_tiles.contains(&position) {
                was_in_view = true;
            }
            if new_view.contains(&position) {
                now_in_view = true;
            }

            if (was_in_view && now_in_view) || (!was_in_view && !now_in_view) {
                continue;
            } else if was_in_view {
                // Ne l'est plus.
                commands.entity(*entity).insert(ChangeVisibility { new_status: ChangeVisibilityStatus::Hidden});
                ev_out_of_sight.send(OutOfSightEvent { entity: *entity});
            } else {
                // vu pour la première fois.
                commands.entity(*entity).insert(ChangeVisibility { new_status: ChangeVisibilityStatus::Visible});
                if let Ok(name) = name_q.get(* entity) {
                    ev_log.send(LogEvent { entry: format!("You see: {:?}", name)});                  
                }
                // Cancel move du joueur
                if let Ok(player_entity) = player_q.get_single() {
                    ev_cancel_move.send(CancelMoveEvent { entity: player_entity });
                }
            }
        }
        // On mets la nouvelle view.
        view.visible_tiles = new_view;
    }
 }


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
) {
    info!("remove markers: a HasBeenSeenEvent has been received.");
    for event in ev_has_been_seen.read() {
        info!("{:?} has been seen by {:?}", event.entity, event.saw_by);
        if let Ok(marked) = marked_q.get(event.entity) {
            info!("{:?} is marked with marker {:?}.", event.entity, marked.marker_id);
          commands.entity(marked.marker_id).despawn_recursive();    // On efface le Marker.
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






