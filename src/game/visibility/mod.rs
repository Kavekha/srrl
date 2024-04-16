// ==> DOCUMENTATION 0.20
/*


v1  | 0.20a | 
*/

use bevy::prelude::*;

use crate::game::visibility::components::ComputeFovEvent;
use self::view_systems::{update_character_view, update_character_view_with_blocked, update_npc_visibility_status, update_tile_visibility_render};

use super:: states::GameState;

pub mod components;
mod view_systems;

 pub struct ViewPlugin;
 
 impl Plugin for ViewPlugin {
     fn build(&self, app: &mut App) {
         app
            // 0.20a
            .add_event::<ComputeFovEvent>()

            //0.20f 
            .add_systems(OnEnter(GameState::Running), init_compute_fov)
            .add_systems(Update, update_character_view_with_blocked.run_if(on_event::<ComputeFovEvent>()))
            .add_systems(Update, update_tile_visibility_render.after(update_character_view_with_blocked).run_if(on_event::<ComputeFovEvent>()))
            .add_systems(Update, update_npc_visibility_status.after(update_character_view_with_blocked).run_if(on_event::<ComputeFovEvent>()))        
            
            /* 
            .add_systems(Update, update_character_view.run_if(on_event::<ComputeFovEvent>()))
            .add_systems(Update, update_tile_visibility_render.after(update_character_view).run_if(on_event::<ComputeFovEvent>()))
            .add_systems(Update, update_npc_visibility_status.after(update_character_view).run_if(on_event::<ComputeFovEvent>()))
            */
        ;   
     }
 }

 // 0.20d On lance ici au lieu de combat start car le combat peut etre lancé pendant l'initialisation et provoquer un crash.
 fn init_compute_fov(
    mut ev_fov: EventWriter<ComputeFovEvent>
 ){
    ev_fov.send(ComputeFovEvent);
 }
