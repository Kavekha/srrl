use std::collections::VecDeque;

use bevy::prelude::*;


#[derive(Default, Resource)]
pub struct CurrentEntityTurnQueue(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct CombatInfos {
    pub turn: u32,
    pub current_entity: Option<Entity>
}
