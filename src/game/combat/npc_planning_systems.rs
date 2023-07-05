use bevy::prelude::*;

use crate::game::pieces::components::Npc;

use super::{components::{CombatInfos, ActionPoints}, events::{EntityEndTurnEvent, Turn}};


/// NPC : Generate / Choice to forfeit their turn.
pub fn plan_action_forfeit(
    combat_info: Res<CombatInfos>,
    query_npc: Query<(Entity, &ActionPoints, &Turn), With<Npc>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
){
    //println!("Planning forfeit...");
    let Some(_entity) = combat_info.current_entity else { return };  //TODO : Toujours necessaire avec le Component Turn?
    for (entity, _action_points, _turn) in query_npc.iter() {
        //TODO : Dans quelles circonstances un NPC decide de Forfeit.
        //println!("planning: Entity is a NPC.");
        ev_endturn.send(EntityEndTurnEvent {entity})     
    }  
}
