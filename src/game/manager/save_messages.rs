use bevy::ecs::archetype::{Archetype, ArchetypeId};
use bevy::ecs::system::SystemState;
use serde::{Deserialize, Serialize};

use bevy::{prelude::*, tasks::IoTaskPool};
use std::fs;
use std::{fs::File, io::Write};

use crate::game::pieces::components::{Health, Melee, Monster, Npc, Occupier, Piece, Stats, Walk};
use crate::game::player::Player;
use crate::game::states::GameState;
use crate::game::tileboard::components::BoardPosition;
use crate::globals::SCENE_FILE_PATH;
use crate::map_builders::map::Map;
use crate::engine::save_load_system::SaveState;

use super::change_state_messages::{ChangeGameStateProcessingMessage, ChangeGameStateRunningMessage};
use super::{Message, MessageEvent};





// On demande à sauvegarder. On mets le State en processing puis on revient à la normale.
pub struct SaveGameRequestMessage;
impl Message for SaveGameRequestMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(MessageEvent(Box::new(ChangeGameStateProcessingMessage)));  
        world.send_event(MessageEvent(Box::new(SavingGameMessage))); 
        world.send_event(MessageEvent(Box::new(ChangeGameStateRunningMessage))); 
    }
}


// The ACTUAL saving.
pub struct SavingGameMessage;
impl Message for SavingGameMessage {
    fn execute(&self, world: &mut World) {
        println!("Save game!");    
        let state = SaveState::create(world);
        println!("Saving... SaveState created.");
        let saved_json = serde_json::to_string(&state).unwrap();
        println!("Saving... json created.");
        
        // Formule magique pour enregistrer dans un fichier.
        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(SCENE_FILE_PATH)       //format!("assets/{NEW_SCENE_FILE_PATH}"))
                    //.and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    //.and_then(|mut file| file.write(serde_json))
                    .and_then(|mut file| file.write(saved_json.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
        println!("Saving... file written.");
    }
}