use std::{any::Any};

use bevy::{prelude::*, ecs::system::SystemState};

use crate::{map_builders::{map::Map}, game::{pieces::components::{Occupier, Health}, tileboard::components::BoardPosition, actions::{PlayerActions}, player::Player}, states::GameState, vectors::{Vector2Int, find_path}};



pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) ->  Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;       //https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-7-better-animation/
}


/// Following Actions after an action resolution.
#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);


pub struct ClearPendingAction(pub Entity);
impl Action for ClearPendingAction {
    fn execute(&self, world:&mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if let Some(mut player_pending_actions) = world.get_resource_mut::<PlayerActions>() {
            player_pending_actions.0.clear();
            println!("ClearPendingAction: player queue removed.");
        }
        Err(()) // Doesnt count as a turn.
    }     
   fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Generate a pathfinding & component. This component will create WalkAction each turn, with a check before each.
pub struct MoveToAction(pub Entity, pub Vector2Int);
impl Action for MoveToAction {
   fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
       println!("MoveToAction: Execute.");
       let Some(position) = world.get::<BoardPosition>(self.0) else { return Err(()) };
       if position.v == self.1 {return Err(())};
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
           println!("Path for {:?} is OK : {:?}", self.0, pathing); 
           
           pathing.reverse();  // REMEMBER : We use Pop to read the path, and pop is the last of Vec.
           if let Some(first_step) = pathing.pop() {
               // First walk action.
               let mut result = Vec::new();
               result.push(Box::new(WalkAction(self.0, first_step)) as Box<dyn Action>);
               println!("Pathing is now : {:?}", pathing);
               pathing.reverse(); // REMEMBER : We reverse back so iter() goes from first to last.

               let mut player_actions = world.get_resource_mut::<PlayerActions>(); 
               if let Some(mut player_actions_queue) = player_actions {
                    let actions = pathing.iter()
                        .map(|some_position_around | {
                                (Box::new(WalkAction(self.0, *some_position_around)) as Box<dyn super::Action>, self.0)
                        })
                        .collect::<Vec<_>>();
                    println!("MoveTo: actions len is : {:?}", actions.len());
                    player_actions_queue.0.extend(actions);
                }
                return Ok(result);                
           } else { return Err(()) };        //REMEMBER : this will consum the vec          
       } else { return Err(()) };
       //Ok(Vec::new())
       
   }
   fn as_any(&self) -> &dyn std::any::Any { self }
}


pub struct WalkAction(pub Entity, pub Vector2Int);    //REMEMBER : arg0 = self.0, arg1 = self.1
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        // Y a-t-il une Map?
        let tileboard = world.get_resource::<Map>().ok_or(())?;        
        //println!("WalkingAction: Il y a une map.");

        // La position où l'on se rends est-elle bloquée?
        if tileboard.is_blocked(self.1.x, self.1.y) { return Err(()) };

        // REMEMBER: pas de tile = pas dans la map.
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
        // We remove all actions in waiting.
        if let Some(mut player_actions) = world.get_resource_mut::<PlayerActions>() {
            player_actions.0.clear();
        }
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

pub struct MeleeHitAction{
    pub attacker: Entity,
    pub target: Vector2Int,
    pub damage: u32
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
        .map(|e| Box::new(DamageAction(e.0, self.damage)) as Box<dyn Action>)
        .collect::<Vec<_>>();
        println!("HIT !");
        Ok(result)

    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}


pub struct DamageAction(pub Entity, pub u32);   //target, dmg
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("DamageAction: Execute!");
        // On verifie si l'entity a bien des PV.
        let Some(mut health) = world.get_mut::<Health>(self.0) else { return Err(()) };
        println!("DamageAction: Health présent.");
        health.current = health.current.saturating_sub(self.1);
        println!("Health for {:?} is now {:?}/{:?}", self.0, health.current, health.max);
        if health.current == 0 {
            if let Some(_player) = world.get::<Player>(self.0) {
                let mut result = Vec::new();
                result.push(Box::new(GameOverAction) as Box<dyn Action>);
                return Ok(result);
            }
            //world.despawn(self.0);
        }
        Ok(Vec::new())
        
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}