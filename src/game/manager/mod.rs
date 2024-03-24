use bevy::{app::AppExit, prelude::*};

pub mod menu_messages;
pub mod game_messages;
pub mod save_messages;
pub mod change_state_messages;

use crate::engine::audios::MusicEvent;

use super::states::GameState;


pub struct ManagerPlugin;
 
impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<MessageEvent>()   
        .add_systems(Update, handle_message_event.run_if(on_event::<MessageEvent>()));
    }
}

#[derive(Event)]
pub struct MessageEvent(pub Box<dyn Message>);

pub fn handle_message_event(
    world: &mut World
) {
    let events = if let Some(mut res) = world.get_resource_mut::<Events<MessageEvent>>() {
        res.drain().collect::<Vec<_>>()
    } else { return };
    for ev in events {
        println!("Manager: Je traite un evenement.");
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

pub struct ExitAppMessage;

impl Message for ExitAppMessage {
    fn execute(&self, world: &mut World) {
        println!("ExitApp ");
        world.send_event(AppExit);
    }
}



enum RecapType{
    GameOver,
    Victory
}

