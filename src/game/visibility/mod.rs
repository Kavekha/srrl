// ==> DOCUMENTATION 0.20
/*


v1  | 0.20a | 
*/

 use std::cmp;

use bevy::prelude::*;

use crate::{engine::render::components::{GameMapRender, TileRender, TileRendered}, map_builders::map::Map, vectors::Vector2Int};
use self::components::ComputeFovEvent;

use super::{player::Player, states::GameState, tileboard::components::BoardPosition, ui::events::ReloadUiEvent};

pub mod components;

const VISIBILITY_PLAYER_RANGE:i32 = 20;

 pub struct ViewPlugin;
 
 impl Plugin for ViewPlugin {
     fn build(&self, app: &mut App) {
         app
            // 0.20a
            .add_event::<ComputeFovEvent>()

            //.add_systems(OnEnter(GameState::Running), apply_visible_tiles)
            .add_systems(Update, apply_visible_tiles.run_if(on_event::<ComputeFovEvent>()))
        ;   
     }
 }

 

// 0.20b Visibility : post refacto spawn_map_render.
fn apply_visible_tiles(
    board: Res<Map>,
    player_position_q: Query<&BoardPosition, With<Player>>,
    logic_tile_position_q: Query<(&BoardPosition, &TileRendered)>,
    game_map_render_q: Query<&GameMapRender>,
    mut visibility_q: Query<&mut Visibility>,
 ){
    println!("Let's check player visibility and update tiles accordingly.");
    let Ok(player_position) = player_position_q.get_single() else { return };

    let min_x = cmp::max(player_position.v.x - VISIBILITY_PLAYER_RANGE, 0);     // player.x = 10. Visibility range = 25. max(10 - 25), 0)  = > retourne 0 au lieu de -15 qui serait hors map.
    let min_y = cmp::max(player_position.v.y - VISIBILITY_PLAYER_RANGE, 0);
    let max_x = cmp::min(player_position.v.x + VISIBILITY_PLAYER_RANGE, board.width - 1);   // idem on veut pas deborder du board.
    let max_y = cmp::min(player_position.v.y + VISIBILITY_PLAYER_RANGE, board.height - 1);
    println!("Visibility: Player Position is {:?}. Minx {min_x}, MinY {min_y}, MaxX {max_x}, MaxY {max_y}", player_position.v);

    let game_map_render = game_map_render_q.single();

    for x in (cmp::max(player_position.v.x - VISIBILITY_PLAYER_RANGE, 0))..(cmp::min(player_position.v.x + VISIBILITY_PLAYER_RANGE, board.width - 1)) {
        for y in (cmp::max(player_position.v.y - VISIBILITY_PLAYER_RANGE, 0))..(cmp::min(player_position.v.y + VISIBILITY_PLAYER_RANGE, board.height - 1)) {
            // J'ai x,y : la tile logic que je peux trouver dans board.map_entities[Vector2Int {x,y}]  = Entity. A terme je peux voir si elle est occupied par exemple.
            // Avec x,y je peux aussi trouver le floor & wall dans GameMapRender.floor & GameMapRender.wall et modifier leur visibilité.
            if game_map_render.floor.contains_key(&Vector2Int {x, y}) {
                let option_entity_floor = game_map_render.floor.get(&Vector2Int {x, y});
                if let Some(entity_floor) = option_entity_floor {
                    if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                        * visibility_floor = Visibility::Visible;
                    }                    
                }
            }
            if game_map_render.wall.contains_key(&Vector2Int {x, y}) {
                let option_entity_wall = game_map_render.wall.get(&Vector2Int {x, y});
                if let Some(entity_wall) = option_entity_wall {
                    if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                        * visibility_wall = Visibility::Visible;
                    }                    
                }
            }
        }
    }
}

 
 
 