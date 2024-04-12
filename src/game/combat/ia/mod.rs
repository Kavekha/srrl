// ===> DOCUMENTATION 0.19h
/*
Au debut, on donne le but de tuer le joueur à tous les NPC.
Au debut de chaque tour, on demande aux NPC de planifier leurs actions selon leur goal.
Le fait d'avoir le goal KillEntity fait planifier un mouvement vers le joueur.
Le fait d'avoir Planifier un Mouvement fait demander une serie de MoveTo vers le joueur.
Par le fonctionnement actuel du Melee (TOCHANGE), le NPC va sur lui pour le taper.
/!\ Une fois qu'il a tapé le joueur, il n'a plus de WantToMove vers le joueur et ne tape plus. Mais il a encore des PA qu'il ne sait pas utilisé => Jeu bloqué.


https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://citeseerx.ist.psu.edu/document%3Frepid%3Drep1%26type%3Dpdf%26doi%3D012ef03d0f951092b8645b69aebdbce900ac03e4&ved=2ahUKEwingo_qkrKFAxWsTaQEHYTTAFIQFnoECCMQAQ&usg=AOvVaw3spa-hKcVtGhhaO5QmYsWT
On veut:
- Un Goal
- Des actions qui permettent de réaliser ce Goal.
- Ces Actions seront disponibles ou non pour les NPC selon leur type (Ranged, Melee)
- Ces Actions ont des conditions pour pouvoir être jouées: Avoir les AP nécessaires par exemple.

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
| 0.19h    | 0.3 | Goal unique + serie de plannings sequentiels jusqu'à trouver celui qui convient et s'y arrêter.
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

use crate::game::{combat::{ia::components::Frozen, rules::NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN}, pieces::components::Npc, player::Player, states::GameState, tileboard::components::BoardPosition};
use self::{goal_systems::{ npc_goal_reached, npc_initialise_goals}, plan_systems::{npc_ai_plan_forfeit, npc_ia_plan_approaching, npc_ia_plan_on_view, npc_ia_plan_when_adjacent, npc_ia_plan_when_in_range}};
use super::{combat_system::components::IsDead, events::Turn, ActionSet};

mod goal_systems;
pub mod components;
mod plan_systems;


pub struct IaPlugin;

impl Plugin for IaPlugin {
    fn build(&self, app: &mut App) {
        app
            // IA v0.3 (0.19i) - Refacto de l'IA. Reproduit la v0.2 avec la logique v0.3 à venir.
            .add_systems(OnEnter(GameState::Running), npc_initialise_goals)// Pas fou car chaque retour en Running on va refaire ça. TODO: Faire des sets sur l'initialisation pour mieux la controler.

            // L'ordre doit être respecté, car dés qu'on trouve une action faisable on ne fait pas les autres. La toute dernière doit être forfeit.
            //TODO : Choix doit être fait en amont?
            .add_systems(Update, (
                ignore_npc_out_of_game_range, 
                npc_goal_reached,           
                npc_ia_plan_when_adjacent,  
                npc_ia_plan_when_in_range,
                npc_ia_plan_on_view,
                npc_ia_plan_approaching,
                npc_ai_plan_forfeit
            ).chain().in_set(ActionSet::Planning))
            //.add_systems(Update, npc_plan_check_surroundings.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick))    
        ;
    }
}

fn ignore_npc_out_of_game_range(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, Option<&Frozen>), (With<Npc>, With<Turn>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>, 
    player_q: Query<Entity, With<Player>>,
    //mut ev_endturn: EventWriter<EntityEndTurnEvent>,    //TODO : Remplacer le EndTurn event par un Forfeit component?
){
    let Ok(player_entity) = player_q.get_single() else { return };
    let Ok(player_position) = position_q.get(player_entity) else { return };
    let mut to_remove = Vec::new();

    for (npc_entity, npc_position, is_frozen) in npc_entity_fighter_q.iter() {
        if (player_position.v.x - npc_position.v.x).abs() > NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN 
        || (player_position.v.y - npc_position.v.y).abs() > NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN {
            println!("NPC {:?} at {:?} is too far from player ({:?})", npc_entity, npc_position, player_position);
            commands.entity(npc_entity).insert(Frozen);
            //turn_to_remove.push(npc_entity)
        } else if is_frozen.is_some() {
            to_remove.push(Frozen)
        };
    };
    /* 
    for entity in turn_to_remove {
        commands.entity(entity).remove::<Turn>();
        ev_endturn.send(EntityEndTurnEvent {entity : entity});  // FIX un peu cheum où on s'assure qu'il ne reste pas bloqué en boucle dans combat_turn_entity_check 0.19j
    }
    */
}
