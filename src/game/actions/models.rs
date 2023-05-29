use bevy::prelude::*;

use crate::{map_builders::{pathfinding::Position, map::Map}, game::{GridPosition, pieces::components::Occupier}};

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
            //println!("WalkingAction: tileboard est bloqué à la position {:?}, {:?}. FALSE.", self.1.0, self.1.1);
            //println!("WalkingAction: tileboard vraiment bloqué? {:?}", tileboard.is_blocked(self.1.0, self.1.1));
            //println!("WalkingAction: Tileboard tile type is : {:?}", tileboard.tiles[tileboard.xy_idx(self.1.0, self.1.1)]);
             return false };    // self.1 : Arg1  = Position
        //println!("WalkingAction: La position n'est pas bloquée.");

        // Quelqu'un est-il déjà dans cette tile?   //TODO : Deactivate for this Release: we want to die at contact with Ghouls. Reactivate ==> Component Occupied on NPC / Player.
        if world.query_filtered::<&GridPosition, With<Occupier>>().iter(world).any(|p| p.x == self.1.0 && p.y == self.1.1) { return false };

        let Some(mut grid_position) = world.get_mut::<GridPosition>(self.0) else { 
            //println!("WalkingAction: Je n'ai pas réussi à recuperer la GridPosition de l'entité {:?}. Je ne peux PAS la mettre à jour.  FALSE.", self.0);
            return false };  // On recupere la GridPosition de l'Entité qui fait l'action (self.0)
        //println!("WalkingAction: Je mets à jour l'Entité dont la GridPosition actuelle est : {:?},{:?}", grid_position.x, grid_position.y);
        (grid_position.x, grid_position.y) = (self.1.0, self.1.1);  // On mets à jour sa GridPosition.
        //println!("WalkingAction: Grid Position à jour. Est désormais : {:?},{:?}. Je retourne TRUE: Action accomplie.",grid_position.x, grid_position.y);
        true
    }
}