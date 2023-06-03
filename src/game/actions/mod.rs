use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::EngineState;

use self::{action_queue_system::{process_action_queue, populate_actor_queue}, plan_systems::{plan_walk, plan_melee}};

pub mod models;
pub mod action_queue_system;
pub mod plan_systems;

pub use models::{WalkAction, Action};


pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()

            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()

            .add_systems(Update, plan_melee.run_if(on_event::<NextActorEvent>()))
            .add_systems(Update, plan_walk.run_if(on_event::<NextActorEvent>()))
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()))



            .add_systems(OnExit(EngineState::PlayerInput), populate_actor_queue)
            ;
    }
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);
pub struct TickEvent;
pub struct NextActorEvent;
pub struct ActionsCompleteEvent;
pub struct InvalidPlayerActionEvent;