use bevy::{ecs::world::World, log::info};

use crate::{
    game::{clean_game_screen, combat::combat_start, game_generation::create_game::create_new_game, gamelog::Gamelog, manager::{
        change_state_messages::{ChangeGameStateInitialiseRequestMessage, QuitGameMessage}, menu_messages::{EndGameRecapMessage, RecapType}, MessageEvent, PlayMusicMessage
    }, player::{camera_center_on_player, cursor_position}, tileboard::system_map::spawning_map, ui::events::ReloadUiEvent}, map_builders::map::Map};

use super::Message;
 
 
// Generate the Logic Map and all NPC / items.
pub struct StartGameMessage;

impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        info!("==== START GAME ===");
        let new_game_created = create_new_game(world);
        /* 
        let map_infos = create_map(world);
        create_player(world, map_infos.starting_position);
        spawn_npcs(world, map_infos.spawn_list);
        create_exit_map(world, map_infos.exit_position);
        create_nodes(world, map_infos.rooms);
        */
        if new_game_created {
            world.send_event(MessageEvent(Box::new(SpawnMapMessage)));
            world.send_event(MessageEvent(Box::new(ChangeGameStateInitialiseRequestMessage)));      
            let music_name = "gamemap".to_string();
            // L'initialisation n'a pas forcement commencé, carefull.
            world.send_event(MessageEvent(Box::new(StartCombatMessage)));   
            world.send_event(MessageEvent(Box::new(PlayMusicMessage{source:music_name})));  
            world.send_event(MessageEvent(Box::new(GamelogClearMessage)));
        } else {
            panic!("Create new game has failed.");
        }
    }
}

pub struct GamelogClearMessage;
impl Message for GamelogClearMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut game_log) = world.get_resource_mut::<Gamelog>() {
            game_log.clear();
        }
    }
}

// Create the Renderable & Interactive Map for Bevy.
pub struct SpawnMapMessage;
impl Message for SpawnMapMessage {
    fn execute(&self, world: &mut World) {
        // Créer les entités necessaires à son affichage, à partir d'une map déja générée.
        if let Some(map) = world.get_resource_mut::<Map>() {
            let mut new_map = map.clone();
            spawning_map(world, &mut new_map); 
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
        let _result = world.run_system(clean_game);
    }
}

pub struct StartCombatMessage;
impl Message for StartCombatMessage {
    fn execute(&self, world: &mut World) {
        info!("StartCombatMessage executed");
        world.send_event(MessageEvent(Box::new(CameraCenterPlayerMessage)));  
        let start_combat = world.register_system(combat_start);
        let _result = world.run_system(start_combat);
        //world.send_event(MessageEvent(Box::new(PlayMusicMessage{source:"combat".to_string()})));          
    }
}

pub struct CameraCenterPlayerMessage;
impl Message for CameraCenterPlayerMessage {
    fn execute(&self, world: &mut World) {
        info!("CameraCenterPlayerMessage executed");
        let camera_center = world.register_system(camera_center_on_player);
        let _result = world.run_system(camera_center); 
        world.send_event(MessageEvent(Box::new(ForceCursorUpdateMessage)));      
    }
}

pub struct ForceCursorUpdateMessage;
impl Message for ForceCursorUpdateMessage {
    fn execute(&self, world: &mut World) {
        info!("ForceCursorUpdateMessage executed");
        let cursor_update = world.register_system(cursor_position);
        let _result = world.run_system(cursor_update);        
    }
}



pub struct ReloadUiMessage;
impl Message for ReloadUiMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(ReloadUiEvent);
    }
}