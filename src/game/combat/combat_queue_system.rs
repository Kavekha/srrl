use bevy::prelude::*;

use crate::game::{
    combat::components::{CurrentEntityTurnQueue, PlayerCombatActionEvent, InvalidPlayerCombatActionEvent}, 
    pieces::components::Actor, player::{Player, PlayerActionEvent}, 
    actions::{InvalidPlayerActionEvent, ActionExecutedEvent, Action, models::PendingActions}
};


fn execute_action(action: Box<dyn Action>, world: &mut World) -> bool {
    if let Ok(result) = action.execute(world) {
        // Si l'action a généré d'autres actions, on les envoi dans Pending.
        if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
            pending.0.extend(result);
        }
        //on informe ensuite que l'action a réussie.
        world.send_event(ActionExecutedEvent(action));
        return true;
    }
    false
}

pub fn process_action_queue(world: &mut World) {
    // Y a-t-il une queue?
    let Some(mut queue) = world.get_resource_mut::<CurrentEntityTurnQueue>() else { return };
    // Quelque chose à traiter?
    let Some(entity) = queue.0.pop_front() else {
        // Aucune action à traiter.
        return;
    };

    println!("Process action queue for {:?}", entity);

    // On recupère le composant Actor qui contient l'action à faire.
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { 
        // L'actor a pu être détruit entre temps, donc on passe au suivant si on ne le trouve pas.
        return;
    };

    // On récupère la liste de ses differentes actions potentielles (NPC => 1+, PJ => Uniquement une.)
    let mut possible_actions = actor.0.drain(..).collect::<Vec<_>>();

    // On trie par score d'importance, determiné dans les plan_systems.
    //REMEMBER : On a deux elements dans Actor : l'Action et sa Valeur => a, b. On trie sur une seule valeur: la B.
    // cf ici : https://en.wikipedia.org/wiki/Partially_ordered_set
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());   

    // On regarde pour chaque action si elle réussie / est possible : si oui on s'arrête à cette action la plus importante.
    let mut success = false;
    for action in possible_actions{
        success = success || execute_action(action.0, world);
        if success { break }
    } 

    // Si c'est un joueur, on a eu le droit qu'à une action. Si elle echoue, on "informe" le joueur via event InvalidPlayerAction et surtout on ne lui fait pas perdre son tour / ne passe par au tour des autres.
    if !success && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerCombatActionEvent);
        return;
    }

    if success && world.get::<Player>(entity).is_some() {
        world.send_event(PlayerCombatActionEvent);
    }
}