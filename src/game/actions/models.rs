use std::any::Any;

use bevy::{prelude::*, ecs::system::SystemState};

use crate::{map_builders::{map::Map}, game::{pieces::components::{Occupier, Health, PathTo}, tileboard::components::BoardPosition}, states::GameState, vectors::{Vector2Int, find_path}};



pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) ->  Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;       //https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-7-better-animation/
}


/// Following Actions after an action resolution.
#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

 
 /// Generate a pathfinding & component. This component will create WalkAction each turn, with a check before each.
 pub struct MoveToAction(pub Entity, pub Vector2Int);
 impl Action for MoveToAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("MoveToAction: Execute.");
        let Some(position) = world.get::<BoardPosition>(self.0) else { return Err(()) };
        let Some(map) = world.get_resource::<Map>() else { return Err(())};
        
        //REMEMBER: Premier step: La case suivante.
        let path_to_destination = find_path(
            position.v,
            self.1,
            &map.entity_tiles.keys().cloned().collect(),
            &world.query_filtered::<&BoardPosition, With<Occupier>>().iter(world).map(|p| p.v).collect()
        );  
        if let Some(path) = path_to_destination {
            let mut pathing:Vec<Vector2Int> = path.clone().into();
            //pathing.reverse();  // REMEMBER : We use Pop to read the path, and pop is the last of Vec.

            /*
            if let Some(first_step) = pathing.pop() {
                // First walk action.
                let mut result = Vec::new();
                result.push(Box::new(WalkAction(self.0, first_step)) as Box<dyn Action>);

                println!("Path for {:?} is OK : {:?}", self.0, pathing);
                world.entity_mut(self.0).insert(PathTo{pathing: pathing});   
                return Ok(result);                
            } else { return Err(()) };        //REMEMBER : this will consum the vec  
            */

            if !pathing.is_empty() {
                println!("MoveToAction: Path for {:?} is OK : {:?}", self.0, pathing);  
                world.entity_mut(self.0).insert(PathTo{pathing: pathing}); 
            } else { return Err(()) }; 
               
        } else { return Err(()) };
        Ok(Vec::new())
        
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}


pub struct WalkAction(pub Entity, pub Vector2Int);    //REMEMBER : arg0 = self.0, arg1 = self.1
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        //println!("WalkAction: Entity : {:?}, Position : {:?}", self.0, self.1);

        // Y a-t-il une Map?
        let tileboard = world.get_resource::<Map>().ok_or(())?;        
        //println!("WalkingAction: Il y a une map.");

        // La position où l'on se rends est-elle bloquée?
        if tileboard.is_blocked(self.1.x, self.1.y) { return Err(()) };

        // Quelqu'un est-il déjà dans cette tile?   //TODO : Deactivate for this Release: we want to die at contact with Ghouls. Reactivate ==> Component Occupied on NPC / Player.
        if !tileboard.entity_tiles.contains_key(&self.1) { return Err(()) };
        if world.query_filtered::<&BoardPosition, With<Occupier>>().iter(world).any(|p| p.v == self.1) { return Err(()) };

        let Some(mut position) = world.get_mut::<BoardPosition>(self.0) else { return Err(()) };
        position.v = self.1;
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}


pub struct GameOverAction;
impl Action for GameOverAction{
    fn execute(&self, mut world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("GameOverAction: Execute!");
        let mut system_state: SystemState<ResMut<NextState<GameState>>> = SystemState::new(&mut world);
        let mut game_state = system_state.get_mut(&mut world);
        game_state.set(GameState::GameOverScreen);
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

pub struct MeleeHitAction{
    pub attacker: Entity,
    pub target: Vector2Int,
    //pub damage: u32
}
impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        //println!("Execute: MeleeHit!: attacker is : {:?}, target position is : {:?}", self.attacker, self.target);
        
        // We get attacker position.
        let attacker_position = world.get::<BoardPosition>(self.attacker).ok_or(())?;
        //println!("ActionMelee: Attacker : OK");
        
        // Si trop loin de sa cible, on ignore.
        if attacker_position.v.manhattan(self.target) > 1 { 
            //println!("Attacker position is {:?}, self.target is {:?}, manhattan is : {:?}", attacker_position.v, self.target, attacker_position.v.manhattan(self.target));
            return Err(()) }; 
        //println!("ActionMelee: Distance : OK");

        // On regarde si la cible est bien là : Position Target vers Position(Gridx, gridy).
        let target_entities = world.query_filtered::<(Entity, &BoardPosition), With<Health>>()
            .iter(world)
            .filter(|(_, position)| position.v == self.target)
            .collect::<Vec<_>>();
        
        //Pas de cible ?
        if target_entities.len() == 0 { return Err(()) }; 

        // TODO : Ajouter dmg somewhere.
        let result = target_entities.iter()
        .map(|e| Box::new(DamageAction(e.0)) as Box<dyn Action>)
        .collect::<Vec<_>>();
        println!("HIT !");
        Ok(result)

    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}


pub struct DamageAction(pub Entity);
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("DamageAction: Execute!");
        // On verifie si l'entity a bien des PV.
        let Some(_health) = world.get_mut::<Health>(self.0) else { return Err(()) };
        println!("DamageAction: Health présent.");
        /* 
        health.value = health.value.saturating_sub(self.1);
        if health.value == 0 {
            // the unit is killed
            world.despawn(self.0);
        }
        Ok(Vec::new())
        */
        let mut result = Vec::new();
        result.push(Box::new(GameOverAction) as Box<dyn Action>);
        Ok(result)
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}