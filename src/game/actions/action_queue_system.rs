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
    // Sur qui on agit?
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { return };
    // Pour quelle action?
    let Some(action) = actor.0.take() else { return };

    // Si execute retourne false et que ca concerne le joueur ==> on lui dit.
    if !action.execute(world) && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }
    // On passe au suivant.
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