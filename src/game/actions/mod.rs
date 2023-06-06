use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::EngineState;

use self::{action_queue_system::{process_action_queue, populate_actor_queue}, plan_systems::{plan_walk, plan_melee}, models::PendingActions};

pub mod models;
pub mod action_queue_system;
pub mod plan_systems;

pub use models::{WalkAction, Action};


pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()

            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()

            //Planning
            .configure_set(Update, ActionSet::Planning.run_if(on_event::<NextActorEvent>()))
            .configure_set(Update, ActionSet::Planning.before(ActionSet::Late))  
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



