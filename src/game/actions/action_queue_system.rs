use bevy::prelude::*;

use crate::game::{player::Player, pieces::components::Actor};

use super::{ActorQueue, ActionsCompleteEvent, InvalidPlayerActionEvent, NextActorEvent, models::PendingActions};



pub fn process_action_queue(world: &mut World) {
    // Y a-t-il des actions en attente à faire?
    if process_pending_actions(world) { 
        println!("Il nous reste des actions en process!");
        return }

    // Y a-t-il une queue?
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    // Quelque chose à traiter?
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    // Qui va faire l'action?
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { return };

    // On récupère la liste de ses differentes actions potentielles (NPC => 1+, PJ => Uniquement une.)
    let mut possible_actions = actor.0.drain(..).collect::<Vec<_>>();

    // On trie par score d'importance, determiné dans les plan_systems.
    //REMEMBER : On a deux elements dans Actor : l'Action et sa Valeur => a, b. On trie sur une seule valeur: la B.
    // cf ici : https://en.wikipedia.org/wiki/Partially_ordered_set
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());   

    // On regarde pour chaque action si elle réussie / est possible : si oui on s'arrête à cette action la plus importante.
    let mut success = false;
    for action in possible_actions{
        //Est ce que l'action a réussie?
        if let Ok(result) = action.0.execute(world) {
            // Est-ce que cela a généré d'autres actions à faire?
            if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
                pending.0 = result
            }
            success = true;
            break;
        }
    }         

    // Si c'est un joueur, on a eu le droit qu'à une action. Si elle echoue, on "informe" le joueur via event InvalidPlayerAction et surtout on ne lui fait pas perdre son tour / ne passe par au tour des autres.
    if !success && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }

    // Au suivant!
    world.send_event(NextActorEvent);
}


pub fn process_pending_actions(world: &mut World) -> bool {
    // Retourne True si un Pending a été processé.
    // Agit sans retenir World.
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        _ => return false
    };

    let mut next = Vec::new();  // Nous mettrons ici les nouvelles actions générées.
    let mut success = false;
    println!("About to deal with Pending actions...");
    for action in pending {
        println!("ProcessPending: Action resolved.");
        if let Ok(result) = action.execute(world) {
            next.extend(result);
            success = true;
        }
    }
    println!("Done with pending action.");
    // Si d'autres actions sont apparues suite à cela, on les ajoute.
    // unwrap OK car on a confirmé que la resource existe au debut.
    let mut res = world.get_resource_mut::<PendingActions>().unwrap();
    res.0 = next;
    println!("About to return result for pending action dealt with: {:?}", success);
    success
}

pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>
) {
    queue.0.extend(
        query.iter()
    );
}