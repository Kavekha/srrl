use bevy::prelude::*;

use crate::{map_builders::{pathfinding::Position, map::Map}, game::GridPosition};

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> bool;
}


pub struct WalkAction(pub Entity, pub Position);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        // Y a-t-il une Map?
        let Some(tileboard) = world.get_resource::<Map>() else { return false };
        // La position où l'on se rends est-elle bloquée?
        if !tileboard.is_blocked(self.1.0, self.1.1) { return false };    // self.1 : Arg1  = Position

        let Some(mut grid_position) = world.get_mut::<GridPosition>(self.0) else { return false };  // On recupere la GridPosition de l'Entité qui fait l'action (self.0)
        (grid_position.x, grid_position.y) = (self.1.0, self.1.1);  // On mets à jour sa GridPosition.
        true
    }
}