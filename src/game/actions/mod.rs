use bevy::prelude::*;
use std::collections::VecDeque;

mod models;
mod action_queue_system;
mod plan_systems;

pub use models::{WalkAction, Action, MeleeHitAction, MoveToAction, WalkOrHitAction};

use crate::states::{EngineState, TurnSet};
use self::{models::{PendingActions}, plan_systems::{plan_melee, plan_walk}, action_queue_system::{populate_actor_queue, process_action_queue}};

use super::player::PlayerActionEvent;


pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()
            .init_resource::<PlayerActions>()

            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_event::<PlayerActionEvent>()
            .add_event::<ActionExecutedEvent>()
            .add_event::<CancelPlayerPendingActionsEvent>()
 

            //Planning
            .configure_sets(Update, (ActionSet::Planning, ActionSet::Late).in_set(TurnSet::Logic))
            .configure_set(Update, ActionSet::Planning.run_if(on_event::<NextActorEvent>()).before(ActionSet::Late))  
              
            .add_systems(Update, plan_melee.run_if(on_event::<NextActorEvent>()).in_set(ActionSet::Planning))
            .add_systems(Update, plan_walk.run_if(on_event::<NextActorEvent>()).in_set(ActionSet::Planning))
            
            // Automatic action that the player should be able to break... TODO : implement in Queue system.
            //.add_systems(OnEnter(EngineState::TurnUpdate),pathfinding_walk.in_set(ActionSet::Planning))
            
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

#[derive(Default, Resource)]
pub struct PlayerActions(pub Vec<(Box<dyn Action>, Entity)>); 

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


#[derive(Event)]
pub struct CancelPlayerPendingActionsEvent;