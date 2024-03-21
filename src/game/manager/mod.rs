use bevy::{app::AppExit, prelude::*};

use crate::vectors::Vector2Int;

use super::{menus::components::InGameMenuState, pieces::spawners::{create_exit_map, create_player, spawn_npcs}, states::{GameState, MainMenuState}, tileboard::system_map::create_map};


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

pub struct TextMessage{
    pub text: String,
    pub source: String
}

impl Message for TextMessage {
    fn execute(&self, _world: &mut World) {
        println!("{} : {}.", self.source, self.text);
    }
}

pub struct StartGameMessage;

impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        let game_infos = create_map(world);
        create_player(world, game_infos.starting_position);
        spawn_npcs(world, game_infos.spawn_list);
        create_exit_map(world, game_infos.exit_position);
        world.send_event(MessageEvent(Box::new(DisplayMapMessage)));
    }
}

pub struct ExitAppMessage;

impl Message for ExitAppMessage {
    fn execute(&self, world: &mut World) {
        println!("ExitApp ");
        world.send_event(AppExit);
    }
}


pub struct DisplayMapMessage;
impl Message for DisplayMapMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Prerun);
        }
    }
}


pub struct QuitGameMessage;
impl Message for QuitGameMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Disabled);
        }
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