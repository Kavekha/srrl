use bevy::prelude::*;

use crate::{
    game::{combat::{action_infos::is_in_sight, components::{ActionPoints, IsDead}, 
    events::Turn, rules::NPC_VISION_RANGE_MAX}, 
    pieces::components::Npc, player::Player, tileboard::components::BoardPosition},
    map_builders::map::Map
};


// IA v0.3 (0.19h) 
pub fn npc_plan_check_surroundings(
    query_npc: Query<(Entity, &ActionPoints, &BoardPosition), (With<Npc>, With<Turn>, Without<IsDead>)>,// Les NPC dont c'est le tour et qui ont des Action Points.
    query_player: Query<&BoardPosition, (With<Player>, Without<IsDead>)>,
    board: Res<Map>,
){
    let Ok(player_position) = query_player.get_single() else { 
        println!("No position found for player. NPC can't check for target.");
        return
    };
     // TODO : is_in_sight pourrait retourner la distance pour prise de decision ensuite.
    for (npc_entity, _, npc_position) in query_npc.iter() {
        let Ok(_in_los) = is_in_sight(&board, &npc_position.v, &player_position.v, NPC_VISION_RANGE_MAX) else {
            println!("NPC {:?}: Player is not in view.", npc_entity);
            continue;
        };
        println!("NPC {:?}: saw the Player!", npc_entity);

        // TODO : J'enregistre sa position. 
        // TODO : J'informe mes copains de cette position.
        // TODO : Au prochain check, je prendrais en compte cette information de position si je l'ai et que je ne vois pas le Player.
        // TODO : Comme je vois / sais où est le Joueur, je vais vers lui / sur lui pour l'attaquer en Melee.
    }
}
