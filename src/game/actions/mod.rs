use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::{EngineState, TurnSet};

use super::player::PlayerActionEvent;



pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()

            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_event::<PlayerActionEvent>()
            .add_event::<ActionExecutedEvent>()          

            //Planning
            .configure_sets(Update, (ActionSet::Planning, ActionSet::Late).in_set(TurnSet::Logic))

            .configure_set(Update, ActionSet::Planning.run_if(on_event::<NextActorEvent>()).before(ActionSet::Late))    
            .add_systems(Update, plan_melee.run_if(on_event::<NextActorEvent>()).in_set(ActionSet::Planning))
            .add_systems(Update, plan_walk.run_if(on_event::<NextActorEvent>()).in_set(ActionSet::Planning))
            
            //Execute
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()).in_set(ActionSet::Late))

            .add_systems(OnExit(EngineState::PlayerInput), populate_actor_queue)
            ;
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionSet {
    Planning,
    Late
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);

