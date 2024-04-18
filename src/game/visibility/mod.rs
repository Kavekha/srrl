// ==> DOCUMENTATION 0.20
/*
Analyse

    Je peux faire confiance à la view.visible_entities.
    Je peux faire confiance au component ChangeTileVisibility.

Clarifier l'attendu.

    Avec un Range de 0, je ne veux rien voir en render, je ne vois que moi.
    Avec un range de 1, je veux voir ma tile en render et la moitié des tiles autour de moi. Elles apparaissent donc partiellement dans l'ombre, et les NPC y sont visibles et semblent en sortir.
    Les murs doivent être visibles. En tout cas le premier "layer" du mur. On arrêterait donc la vue après le premier obstacle. Peut-être même un peu plus loin pour ne pas être etouffant (Determiné par l'environnement interieur / exterieur?) Sinon on ne voit jamais que le sol.

La question qui fache.

A quel point ai-je besoin d'afficher ou non le champ de vision, ou en tout cas à quel point ce doit être précis?

    Cela a un interêt tactique et suspense : Je ne sais pas ce qu'il y a devant moi, je dois donc faire attention et être prudent si je n'ai pas de visibilité tactique.
    Un "contre" serait que c'est très claustrophobique. On ne voit que le sol et un peu des murs, mais jamais d'avantage. On peut résoudre cela en ne coupant le champ de vision à la fin de l'obstacle ou après un certain nombre de "layers de murs" traversés. P-e un peu trop spécifique.
    On peut avoir des drones, des sorts, des schematiques ou autres outils qui permettent d'avoir une visibilité de haut, temporaire ou permanente, plus ou moins précise.


v1  | 0.20a | 
*/

use bevy::prelude::*;

use crate::game::visibility::components::ComputeFovEvent;
use self::{view_systems::{update_character_view_on_npc_action, update_character_view_with_blocked}, visibility_render::{update_npc_visibility_status, update_tile_visibility_render}};

use super::{combat::CombatSet, states::GameState};

pub mod components;
mod view_systems;
mod visibility_render;

 pub struct ViewPlugin;
 
 impl Plugin for ViewPlugin {
     fn build(&self, app: &mut App) {
         app
            // 0.20a
            .add_event::<ComputeFovEvent>()            

            //0.20f 
            .add_systems(OnEnter(GameState::Running), init_compute_fov)
            .add_systems(Update, update_character_view_with_blocked.run_if(on_event::<ComputeFovEvent>()).in_set(CombatSet::Logic))
            .add_systems(Update, update_character_view_on_npc_action.in_set(CombatSet::Logic))
            .add_systems(Update, update_npc_visibility_status.in_set(CombatSet::Animation))  
            .add_systems(Update, update_tile_visibility_render.in_set(CombatSet::Animation))    // PERFS : Tourne en boucle.
        ;   
     }
 }

 // 0.20d On lance ici au lieu de combat start car le combat peut etre lancé pendant l'initialisation et provoquer un crash.
 fn init_compute_fov(
    mut ev_fov: EventWriter<ComputeFovEvent>
 ){
    ev_fov.send(ComputeFovEvent);
 }


