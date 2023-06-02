use bevy::prelude::*;

use crate::game::{player::Player, pieces::components::Actor};

use super::{ActorQueue, ActionsCompleteEvent, InvalidPlayerActionEvent, NextActorEvent};



pub fn process_action_queue(world: &mut World) {
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
        if action.0.execute(world) {
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


pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>
) {
    queue.0.extend(
        query.iter()
    );
}