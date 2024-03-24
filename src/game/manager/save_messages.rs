use bevy::{ecs::world::World, tasks::IoTaskPool};
use std::{fs::{self, File}, io::Write};

use crate::{engine::save_load_system::SaveState, game::{pieces::components::{Monster, Npc, Occupier, Walk}, player::Player}, globals::SCENE_FILE_PATH};

use super::{change_state_messages::{ChangeGameStateInitialiseMessage, ChangeGameStateProcessingMessage, ChangeGameStateRunningMessage}, Message, MessageEvent};




// On demande à sauvegarder. On mets le State en processing puis on revient à la normale.
pub struct SaveGameRequestMessage;
impl Message for SaveGameRequestMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(MessageEvent(Box::new(ChangeGameStateProcessingMessage)));  
        world.send_event(MessageEvent(Box::new(SavingGameMessage))); 
        world.send_event(MessageEvent(Box::new(ChangeGameStateRunningMessage))); 
    }
}

pub struct LoadGameRequestMessage;
impl Message for LoadGameRequestMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(MessageEvent(Box::new(ChangeGameStateProcessingMessage)));  
        world.send_event(MessageEvent(Box::new(LoadGameMessage))); 
        world.send_event(MessageEvent(Box::new(ChangeGameStateInitialiseMessage))); 
    }
}

// The ACTUAL loadin.
pub struct LoadGameMessage;
impl Message for LoadGameMessage {
    fn execute(&self, world: &mut World) {
        println!("Load game!");

        let data = fs::read_to_string(SCENE_FILE_PATH)
        .expect("Unable to read file");

        let _json: serde_json::Value = serde_json::from_str(&data)
            .expect("JSON does not have correct format.");

        let state: SaveState = serde_json::from_str(&data).unwrap();

        world.insert_resource(state.map);

        for entity in state.entities {
            let mut e = world.spawn_empty();         

            if entity.player {
                e.insert(Player);
            }        
            if entity.npc {
                e.insert(Npc);
            }
            if entity.monster {
                e.insert(Monster);
            }
            if let Some(stats) = entity.stats {
                e.insert(stats);
            }
            if let Some(piece) = entity.piece {
                e.insert(piece);
                //e.insert(Actor::default()); // Actor component can't be save, so we have to add it there if NPC or Player.    // No Actor?
            }
            if let Some(position) = entity.position {
                println!("Load: Position of {:?} is now : {:?}", entity, position);
                e.insert(position);
            }
            if entity.walk {
                e.insert(Walk);
            }
            if let Some(health) = entity.health {
                e.insert(health);
            }
            if let Some(melee) = entity.melee {
                e.insert(melee);
            }
            if entity.occupier {
                e.insert(Occupier);
            }
        }
        println!("Loading complete.");
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