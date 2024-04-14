// ==> DOCUMENTATION 0.20
/*


v1  | 0.20a | 
*/

 use std::cmp;

use bevy::prelude::*;

use crate::{engine::render::components::{TileRender, TileRendered}, map_builders::map::Map};
use self::components::ComputeFovEvent;

use super::{player::Player, states::GameState, tileboard::components::BoardPosition, ui::events::ReloadUiEvent};

pub mod components;

const VISIBILITY_PLAYER_RANGE:i32 = 5;

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

// v1 Visibility.
 fn apply_visible_tiles(
    board: Res<Map>,
    player_position_q: Query<&BoardPosition, With<Player>>,
    logic_tile_position_q: Query<(&BoardPosition, &TileRendered)>,
    mut render_tile_q: Query<&Children, With<TileRender>>,
    mut visibility_q: Query<&mut Visibility>,
 ){
    println!("Let's check player visibility and update tiles accordingly.");
    let Ok(player_position) = player_position_q.get_single() else { return };
    let min_x = cmp::max(player_position.v.x - VISIBILITY_PLAYER_RANGE, 0);     // player.x = 10. Visibility range = 25. max(10 - 25), 0)  = > retourne 0 au lieu de -15 qui serait hors map.
    let min_y = cmp::max(player_position.v.y - VISIBILITY_PLAYER_RANGE, 0);
    let max_x = cmp::min(player_position.v.x + VISIBILITY_PLAYER_RANGE, board.width - 1);   // idem on veut pas deborder du board.
    let max_y = cmp::min(player_position.v.y + VISIBILITY_PLAYER_RANGE, board.height - 1);
    println!("Visibility: Player Position is {:?}. Minx {min_x}, MinY {min_y}, MaxX {max_x}, MaxY {max_y}", player_position.v);

    // TODO : Can't do that each step. On va devoir enregistrer les tuiles pour les mettre à jour? 0.20a
    for (tile_position, tile_rendered) in logic_tile_position_q.iter() {
        if tile_position.v.x >= min_x && tile_position.v.x <= max_x && tile_position.v.y >= min_y && tile_position.v.y <= max_y {
            if let Ok(mut parent_visibility) = visibility_q.get_mut(tile_rendered.render_entity) {
                *parent_visibility = Visibility::Visible;
            }
            if let Ok(children) = render_tile_q.get_mut(tile_rendered.render_entity) {
                for child in children {
                    let Ok(mut child_visibility) = visibility_q.get_mut(*child) else { continue };
                    *child_visibility = Visibility::Visible;
                } 
            }
        } else {
            if let Ok(mut parent_visibility) = visibility_q.get_mut(tile_rendered.render_entity) {
                *parent_visibility = Visibility::Hidden;
            }
            if let Ok(children) = render_tile_q.get_mut(tile_rendered.render_entity) {
                for child in children {
                    let Ok(mut child_visibility) = visibility_q.get_mut(*child) else { continue };
                    *child_visibility = Visibility::Hidden;
                } 
            }
        }
    }   
}
 
 
 