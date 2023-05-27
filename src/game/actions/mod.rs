use bevy::prelude::*;
use std::collections::VecDeque;

use self::action_queue_system::process_action_queue;

pub mod models;
pub mod action_queue_system;

pub use models::{WalkAction, Action};


pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>())
            );
    }
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);
pub struct TickEvent;
pub struct NextActorEvent;
pub struct ActionsCompleteEvent;
pub struct InvalidPlayerActionEvent;