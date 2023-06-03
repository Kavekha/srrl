use bevy::prelude::*;

use crate::{map_builders::{pathfinding::Position, map::Map}, game::{GridPosition, pieces::components::{Occupier, Health}}};

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> bool;
}


pub struct WalkAction(pub Entity, pub Position);    //REMEMBER : arg0 = self.0, arg1 = self.1
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        //println!("WalkAction: Entity : {:?}, Position : {:?}", self.0, self.1);

        // Y a-t-il une Map?
        let Some(tileboard) = world.get_resource::<Map>() else { 
            //println!("WalkingAction: No Map. Immediate failure. FALSE.");
            return false };
        println!("WalkingAction: Il y a une map.");

        // La position où l'on se rends est-elle bloquée?
        if tileboard.is_blocked(self.1.0, self.1.1) {
             return false };
        // Quelqu'un est-il déjà dans cette tile?   //TODO : Deactivate for this Release: we want to die at contact with Ghouls. Reactivate ==> Component Occupied on NPC / Player.
        if world.query_filtered::<&GridPosition, With<Occupier>>().iter(world).any(|p| p.x == self.1.0 && p.y == self.1.1) { return false };

        let Some(mut grid_position) = world.get_mut::<GridPosition>(self.0) else { 
            return false };  // On recupere la GridPosition de l'Entité qui fait l'action (self.0)
        (grid_position.x, grid_position.y) = (self.1.0, self.1.1);  // On mets à jour sa GridPosition.
        true
    }
}


pub struct MeleeHitAction{
    pub attacker: Entity,
    pub target: Position,
    //pub damage: u32
}
impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> bool {
        // We get attacker position.
        let Some(attacker_gridposition) = world.get::<GridPosition>(self.attacker) else { return false };
        // Si trop loin de sa cible, on ignore.
        let attacker_position = Position(attacker_gridposition.x, attacker_gridposition.y);
        if attacker_position.distance(&self.target) > 1 { return false };
        // On regarde si la cible est bien là : Position Target vers Position(Gridx, gridy).
        let target_entities = world.query_filtered::<(Entity, &GridPosition), With<Health>>()
            .iter(world)
            .filter(|(_, p)| Position(p.x, p.y) == self.target)
            .collect::<Vec<_>>();
        if target_entities.len() == 0 { return false }; //Pas de cible.
        
        // TODO deal actual damage
        info!("Hit!");
        true
    }
}