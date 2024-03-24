use bevy::ecs::world::World;

use crate::{
    game::{clean_game_screen, combat::combat_start, manager::{
        change_state_messages::{ChangeGameStateInitialiseRequestMessage, QuitGameMessage}, menu_messages::{EndGameRecapMessage, RecapType}, MessageEvent, PlayMusicMessage
    }, pieces::spawners::{create_exit_map, create_player, spawn_npcs}, tileboard::system_map::{create_map, spawning_map}}, map_builders::map::Map};

use super::Message;
 

// Generate the Logic Map and all NPC / items.
pub struct StartGameMessage;

impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        println!("==== START GAME ===");
        let game_infos = create_map(world);
        create_player(world, game_infos.starting_position);
        spawn_npcs(world, game_infos.spawn_list);
        create_exit_map(world, game_infos.exit_position);
        world.send_event(MessageEvent(Box::new(SpawnMapMessage)));
        world.send_event(MessageEvent(Box::new(ChangeGameStateInitialiseRequestMessage)));      
        let music_name = "gamemap".to_string();
        world.send_event(MessageEvent(Box::new(StartCombatMessage)));   
        world.send_event(MessageEvent(Box::new(PlayMusicMessage{source:music_name})));  
    }
}


// Create the Renderable & Interactive Map for Bevy.
pub struct SpawnMapMessage;
impl Message for SpawnMapMessage {
    fn execute(&self, world: &mut World) {
        // Créer les entités necessaires à son affichage, à partir d'une map déja générée.
        println!("Spawning map?"); 
        if let Some(map) = world.get_resource_mut::<Map>() {
            println!("Yes we do.");
            let mut new_map = map.clone();
            spawning_map(world, &mut new_map);     
        } else {
            println!("No we dont.");
        }
    }
}


// Defeat : Quit the game and display the Recap as a game over.
pub struct GameOverMessage;
impl Message for GameOverMessage {
    fn execute(&self, world: &mut World) {
        println!("Game Over Message!");
        world.send_event(MessageEvent(Box::new(QuitGameMessage)));
        world.send_event(MessageEvent(Box::new(EndGameRecapMessage{recap_type:RecapType::GameOver})));
        let music_name = "gameover".to_string();
        world.send_event(MessageEvent(Box::new(PlayMusicMessage{source:music_name})));  
    }
}


pub struct VictoryMessage;
impl Message for VictoryMessage {
    fn execute(&self, world: &mut World) {
        println!("Victory Message!");
        world.send_event(MessageEvent(Box::new(QuitGameMessage)));
        world.send_event(MessageEvent(Box::new(EndGameRecapMessage{recap_type:RecapType::Victory})));
        let music_name = "victory".to_string();
        world.send_event(MessageEvent(Box::new(PlayMusicMessage{source:music_name})));  
    }
}




// Remove any existing element of the current game. (From clean_game_screen)
pub struct ClearGameMessage;
impl Message for ClearGameMessage {
    fn execute(&self, world: &mut World) {
        let clean_game = world.register_system(clean_game_screen);
        let result = world.run_system(clean_game);
        println!("Result is {:?}", result);
    }
}

pub struct StartCombatMessage;
impl Message for StartCombatMessage {
    fn execute(&self, world: &mut World) {
        let start_combat = world.register_system(combat_start);
        let result = world.run_system(start_combat);
 
    }
}