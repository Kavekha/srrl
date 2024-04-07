// ===> DOCUMENTATION 0.19h
/*
Conception:
* Chaque NPC regarde s'ils voient le PJ.
    * Chaque NPC qui voient le PJ communiquent avec les autres NPC pour les en informer.
    * Si NPC est mélée only, regarde s'il peut porter au moins un coup.
        * S'il ne peut pas porter au moins un coup, regarde si d'autres autour de lui peuvent frapper: si c'est le cas, il approche quand même.
        * Sinon, il essait de rejoindre d'autres NPC
        * Sinon il s'approche mais petit à petit.
        * Sinon il se rends vers la sortie.
    * Un peu de randomness dans tout ca pour ne pas être prédictif.
* Si le NPC ne voit pas le PJ, alors:
    * Il se rapproche d'un autre NPC et le suit.
    * il note l'endroit où il est et choisi une autre direction.
        * Dans l'ideal, il retient les trajets déjà faits et explore ailleurs. Pas obligé de retenir tout le chemin, au moins les 2-3 derniers points de passage.
        * Peut se deplacer plus lentement dans cette situation.

----------------------------------------------------------------------------------------------------
| 0.19h    | 0.3 |
| 0.13     | 0.2 | IA  planifie, attaque si AP, bouge si AP, abandonne si rien à faire.
| 0.6      | 0.1 | NPC poursuivent le joueur.
------------------------------------------------------------------------------------------------

!!!! 
let Ok(_in_los) = is_in_sight(&board, &position.v, &action_infos.target.unwrap(), RANGED_ATTACK_RANGE_MAX) else {
                println!("Has target, not in view");
                continue;
            };
*/

use bevy::prelude::*;

use crate::game::states::GameState;

use self::npc_planning_systems::{npc_plan_check_surroundings, npc_planning};

use super::CombatSet;
pub mod npc_planning_systems;


pub struct IaPlugin;

impl Plugin for IaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, npc_plan_check_surroundings.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, npc_planning.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))        
        ;
    }
}