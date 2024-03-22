use bevy::{app::AppExit, prelude::*};

use crate::{
    game::tileboard::system_map::spawning_map, 
    map_builders::map::Map, 
    engine::audios::MusicEvent
};

use super::{
    clean_game_screen, menus::components::InGameMenuState, pieces::spawners::{create_exit_map, create_player, spawn_npcs}, states::{GameState, MainMenuState}, tileboard::system_map::create_map
};


pub struct ManagerPlugin;
 
impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<MessageEvent>()   
        .add_systems(Update, handle_event.run_if(on_event::<MessageEvent>()));
    }
}

#[derive(Event)]
pub struct MessageEvent(pub Box<dyn Message>);

fn handle_event(
    world: &mut World
) {
    let events = if let Some(mut res) = world.get_resource_mut::<Events<MessageEvent>>() {
        res.drain().collect::<Vec<_>>()
    } else { return };
    for ev in events {
       ev.0.execute(world);
    }
}

pub trait Message: Send + Sync {
    fn execute(&self, world: &mut World);
}

pub struct PlayMusicMessage{
    pub source: String
}
impl Message for PlayMusicMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(MusicEvent{source:self.source.clone()});
    }
}


pub struct StartGameMessage;

impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        println!("==== START GAME ===");
        let game_infos = create_map(world);
        create_player(world, game_infos.starting_position);
        spawn_npcs(world, game_infos.spawn_list);
        create_exit_map(world, game_infos.exit_position);
        world.send_event(MessageEvent(Box::new(SpawnMapMessage)));
        world.send_event(MessageEvent(Box::new(RunGameMessage)));      
        let music_name = "gamemap".to_string();
        world.send_event(MessageEvent(Box::new(PlayMusicMessage{source:music_name})));  
    }
}

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


pub struct ExitAppMessage;

impl Message for ExitAppMessage {
    fn execute(&self, world: &mut World) {
        println!("ExitApp ");
        world.send_event(AppExit);
    }
}

pub struct RunGameMessage;
impl Message for RunGameMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Running);
        }
    }
}

pub struct GameOverMessage;
impl Message for GameOverMessage {
    fn execute(&self, world: &mut World) {
        println!("Game Over Message!");
        world.send_event(MessageEvent(Box::new(QuitGameMessage)));
        world.send_event(MessageEvent(Box::new(EndGameRecapMessage)));
    }
}

//TODO : Modifier pour afficher le Menu Game Over.
pub struct EndGameRecapMessage;
impl Message for EndGameRecapMessage {
    fn execute(&self, world: &mut World) {
        println!("End Game Recap?");
        if let Some(mut state) = world.get_resource_mut::<NextState<MainMenuState>>() {
            state.set(MainMenuState::RecapMenu);
            println!("yes");
        } else {
            println!("no");
        }
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

        /*
        let mut query_npc = world.query_filtered::<Entity, With<Npc>>();
        for entity in query_npc.iter(&world){
            world.commands.entity(entity).despawn_recursive();
        };
         */
    }
}

pub struct ActiveMainMenuMessage;
impl Message for ActiveMainMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<MainMenuState>>() {
            state.set(MainMenuState::MainMenu);
        }
    }
}
pub struct ActiveInGameMenuMessage;
impl Message for ActiveInGameMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<InGameMenuState>>() {
            state.set(InGameMenuState::MainMenu);
        }
    }
}

pub struct CloseInGameMenuMessage;
impl Message for CloseInGameMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<InGameMenuState>>() {
            state.set(InGameMenuState::Disabled);
        }
    }
}

pub struct CloseMainMenuMessage;
impl Message for CloseMainMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<MainMenuState>>() {
            state.set(MainMenuState::Disabled);
        }
    }
}