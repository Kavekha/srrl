use bevy::ecs::{schedule::NextState, world::World};

use crate::{
    game::{clean_game_screen, manager::{change_state_messages::ChangeGameStateRunningMessage, MessageEvent, PlayMusicMessage, RecapType}, 
    menus::{components::MenuButtonAction, menu_builder::{Menu, MenuItem}, MenuEvent, MenuType}, 
    pieces::spawners::{create_exit_map, create_player, spawn_npcs}, states::GameState, tileboard::system_map::{create_map, spawning_map}}, map_builders::map::Map};

use super::{menu_messages::OpenMenuMessage, Message};


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
        world.send_event(MessageEvent(Box::new(ChangeGameStateRunningMessage)));      
        let music_name = "gamemap".to_string();
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

// Create a Menu, using MenuBuilder v2.
struct EndGameRecapMessage{
    recap_type: RecapType
}
impl Message for EndGameRecapMessage {
    fn execute(&self, world: &mut World) {        
        match self.recap_type {
            RecapType::GameOver => {
                let mut menu = Menu::new("game_over", Vec::new());

                menu.add(MenuItem::header("You died."));
                menu.add(MenuItem::description("A ghoul has eaten you."));
                menu.add(MenuItem::action(MenuButtonAction::Play, "Retry"));
                menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Main Menu"));
                
        
                world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
                world.send_event(MenuEvent{menu:menu, menu_type:MenuType::RECAPMENU});
                println!("Recap GameOver generated and send for opening.");
            },
            RecapType::Victory => {
                let mut menu = Menu::new("victory", Vec::new());

                menu.add(MenuItem::header("victory!"));
                menu.add(MenuItem::description("You flee the place."));
                menu.add(MenuItem::action(MenuButtonAction::Play, "Retry"));
                menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Main Menu"));
        
                world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
                world.send_event(MenuEvent{menu:menu, menu_type:MenuType::RECAPMENU});
                println!("Recap Victory generated and send for opening.");
            },
            //_ => println!("Autres types de Recap non supportés.")
        };
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
    }
}


pub struct QuitGameMessage;
impl Message for QuitGameMessage {
    fn execute(&self, world: &mut World) {
        let mut can_quit = false;
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Disabled);
            println!("GameState is now disabled");
            can_quit = true;
        }
        if can_quit {
            world.send_event(MessageEvent(Box::new(ClearGameMessage)));
        }
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

