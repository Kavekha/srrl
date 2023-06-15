use std::{any::Any};

use bevy::{prelude::*, ecs::system::SystemState};

use crate::{
    map_builders::{map::Map}, game::{pieces::components::{Occupier, Health, Stats, Monster, Melee},
    tileboard::components::BoardPosition, actions::{PlayerActions}, player::Player, rules::roll_dices_against}, states::GameState, vectors::{Vector2Int, find_path}};



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
           //println!("Path for {:?} is OK : {:?}", self.0, pathing); 
           
           pathing.reverse();  // REMEMBER : We use Pop to read the path, and pop is the last of Vec.
           if let Some(first_step) = pathing.pop() {
               // First walk action.
               let mut result = Vec::new();
               result.push(Box::new(WalkAction(self.0, first_step)) as Box<dyn Action>);
               //println!("Pathing is now : {:?}", pathing);
               pathing.reverse(); // REMEMBER : We reverse back so iter() goes from first to last.

               let mut player_actions = world.get_resource_mut::<PlayerActions>(); 
               if let Some(mut player_actions_queue) = player_actions {
                    let actions = pathing.iter()
                        .map(|some_position_around | {
                                //(Box::new(WalkAction(self.0, *some_position_around)) as Box<dyn Action>, self.0)
                                (Box::new(WalkOrHitAction(self.0, *some_position_around)) as Box<dyn Action>, self.0)
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


/// TODO: Avec cette Action on check si enemy a la target (self.0) et si oui, on attaque. Sinon, on marche.
/// On fait ca sur Monster only. 
/// TODO : Changer pour Hostile ?
pub struct WalkOrHitAction(pub Entity, pub Vector2Int);
impl Action for WalkOrHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("WalkOrHit: Execute!");
        let tileboard = world.get_resource::<Map>().ok_or(())?; 
        if !tileboard.entity_tiles.contains_key(&self.1) { return Err(()) };    // Hors map 

        let mut has_target = false;
        if world.query_filtered::<&BoardPosition, With<Monster>>().iter(world).any(|p| p.v == self.1) {
            has_target = true;
        }

        let mut result = Vec::new();
        if has_target {
            let Some(melee) = world.get::<Melee>(self.0) else { return Err(()) };  
            result.push(Box::new(MeleeHitAction{
                attacker: self.0,
                target: self.1,
                damage: melee.damage
            })  as Box<dyn Action>);
            println!("WalkOrHit: Hit !");
            Ok(result)
        } else {
            result.push(Box::new(WalkAction(self.0, self.1)) as Box<dyn Action>);
            println!("WalkOrHit: Walk");
            Ok(result)
        }        
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
        // We get attacker position.
        let attacker_position = world.get::<BoardPosition>(self.attacker).ok_or(())?;
        // Si trop loin de sa cible, on ignore.
        if attacker_position.v.manhattan(self.target) > 1 { return Err(()) }; 
        // On récupère les cibles de cette case: Position Target vers Position(Gridx, gridy).
        let target_entities = world.query_filtered::<(Entity, &BoardPosition, &Stats), With<Health>>()
            .iter(world)
            .filter(|(_, position,_)| position.v == self.target)
            .collect::<Vec<_>>();        
        //Pas de cible ?
        if target_entities.len() == 0 { return Err(()) }; 

        //TODO : SR stats
        let attacker_stat = world.get::<Stats>(self.attacker).ok_or(())?;
        let mut result = Vec::new();
        for (target_entity, _target_position, target_stats) in target_entities.iter() {            
            //TODO: Add to Stats component.
            let dice_roll = roll_dices_against(attacker_stat.attack, target_stats.dodge);   
            let dmg = dice_roll.success.saturating_add(attacker_stat.power as u32);
            if dice_roll.success > 0 {
                println!("HIT target with {:?} success!", dice_roll.success);
                result.push(Box::new(DamageAction(*target_entity, self.damage.saturating_add(dmg))) as Box<dyn Action>)
            }
        }
        /*
        // Old version.
        let result = target_entities.iter()
        .map(|e| Box::new(DamageAction(e.0, self.damage)) as Box<dyn Action>)
        .collect::<Vec<_>>();
        println!("HIT !");
         */
        Ok(result)

    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}


pub struct DamageAction(pub Entity, pub u32);   //target, dmg
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("DamageAction: Execute!");

        //TODO : Stats vs Stats Shadowrun. Erzatz pour le moment.
        let Some(stats) = world.get::<Stats>(self.0) else { return Err(())};    //TODO: A surveiller. On pourra p-e se faire tabasser un jour sans Stat (Door and co)
        let dice_roll = roll_dices_against(stats.resilience, 0);       // Pas d'opposant ni difficulté : On encaisse X dmg.
        let dmg = stats.power.saturating_sub(dice_roll.success);    //REMEMBER: saturating_sub ne modifie pas l'objet sur lequel il est utilisé

        let Some(mut health) = world.get_mut::<Health>(self.0) else { return Err(()) };
        health.current = health.current.saturating_sub(dmg);
        println!("Dmg on health for {:?} is now {:?}/{:?}", dmg, health.current, health.max);
        if health.current == 0 {
            if let Some(_player) = world.get::<Player>(self.0) {
                let mut result = Vec::new();
                result.push(Box::new(GameOverAction) as Box<dyn Action>);
                return Ok(result);
            } else {
                //TODO : Some Death Event for player & NPC?
                world.despawn(self.0);
            }
            
        }
        Ok(Vec::new())
        
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}