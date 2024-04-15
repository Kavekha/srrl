// ==> DOCUMENTATION 0.20
/*


v1  | 0.20a | 
*/

 use std::cmp;

use bevy::prelude::*;

use crate::{engine::render::components::GameMapRender, game::combat::rules::VISIBILITY_RANGE_PLAYER, map_builders::map::Map, vectors::Vector2Int};
use self::components::{ChangeTileVisibility, ChangeTileVisibilityStatus, ComputeFovEvent, View};

use super::{player::Player, states::GameState, tileboard::components::{BoardPosition, Tile}};

pub mod components;

 pub struct ViewPlugin;
 
 impl Plugin for ViewPlugin {
     fn build(&self, app: &mut App) {
         app
            // 0.20a
            .add_event::<ComputeFovEvent>()

            .add_systems(OnEnter(GameState::Running), init_compute_fov)
            .add_systems(Update, update_character_view.run_if(on_event::<ComputeFovEvent>()))
            .add_systems(Update, update_tile_visibility_render.after(update_character_view).run_if(on_event::<ComputeFovEvent>()))
            //.add_systems(Update, apply_visible_tiles.run_if(on_event::<ComputeFovEvent>()))
        ;   
     }
 }

 // 0.20d On lance ici au lieu de combat start car le combat peut etre lancé pendant l'initialisation et provoquer un crash.
 fn init_compute_fov(
    mut ev_fov: EventWriter<ComputeFovEvent>
 ){
    ev_fov.send(ComputeFovEvent);
 }

// 0.20c : Recupère les tuiles autour du personnage, en accord avec le range donné.
// NOTE: Ne se préocupe pas des obstacles pour le moment.
fn get_tiles_around_range(
    x: i32, 
    y: i32,
    range: i32,
    max_x: i32, // map width
    max_y: i32  // map height
 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    for x in (cmp::max(x - range, 0))..(cmp::min(x + range, max_x)) {
        for y in (cmp::max(y - range, 0))..(cmp::min(y + range, max_y)) {
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
 fn update_tile_visibility_render(
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut visibility_q: Query<&mut Visibility>,
 ){
    let game_map_render = game_map_render_q.single();
    let mut to_remove = Vec::new();
    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {

        if let Some(entity_floor) = get_floor_entity_at(game_map_render, position.v.x, position.v.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_floor = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_floor = Visibility::Hidden,
                    ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, position.v.x, position.v.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_wall = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_wall = Visibility::Hidden,
                    ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
        to_remove.push(entity);
    }
    for entity in to_remove {
        commands.entity(entity).remove::<ChangeTileVisibility>();
    }
 }

 
 // 0.20d visibility system with component. Only works for Logic Tile.
fn update_character_view(
    mut commands: Commands,
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    board: Res<Map>,
 ){
    for ( mut view, board_position) in player_view_q.iter_mut() {
        let mut view_to_treat = get_tiles_around_range(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1);

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

 // 0.20c visibility system with component.
fn update_character_visibility_old(
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    board: Res<Map>,
    game_map_render_q: Query<&GameMapRender>,
    mut visibility_q: Query<&mut Visibility>,
 ){
    let game_map_render = game_map_render_q.single();
    for ( mut view, board_position) in player_view_q.iter_mut() {
        //info!("I'm {:?} and my view visible tiles is : {:?}", entity, view.visible_tiles);

        let mut view_to_treat = get_tiles_around_range(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1);
        //info!(">> My Original view_to_treat is : {:?}", view_to_treat);

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
        //info!("After evaluating view.visible_tiles, I have");
        //info!("- current_view = {:?}", current_view);
        //info!("- to_hide = {:?}", to_hide);
        //info!("- treated = {:?}", treated);

        // Rendre invisible.
        for hiden_tile in to_hide.iter() {
            // On rends invisible.
            if let Some(entity_floor) = get_floor_entity_at(game_map_render, hiden_tile.x, hiden_tile.y ) {
                if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                    * visibility_floor = Visibility::Hidden;
                }                    
            }
            if let Some(entity_wall) = get_wall_entity_at(game_map_render, hiden_tile.x, hiden_tile.y ) {
                if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                    * visibility_wall = Visibility::Hidden;
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

        for tile in view_to_treat.iter() {
            current_view.push(*tile);
            //rendre visible.
            if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile.x, tile.y ) {
                if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                    * visibility_floor = Visibility::Visible;
                }                    
            }
            if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile.x, tile.y ) {
                if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                    * visibility_wall = Visibility::Visible;
                }                    
            }
        }
        //info!("My current view is now : {:?}", current_view);
        //info!("It should be the same that My Original view to treat");
        // On mets la nouvelle view.
        view.visible_tiles = current_view;
    }
}


 