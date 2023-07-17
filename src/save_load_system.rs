//https://gist.github.com/chamons/37e8c6f8753e63eaef08bef36686c2e2

use bevy::ecs::archetype::{Archetype, ArchetypeId};
use bevy::ecs::system::SystemState;
use serde::{Deserialize, Serialize};

use bevy::{prelude::*, tasks::IoTaskPool};
use std::fs;
use std::{fs::File, io::Write};
use std::path::Path;

pub struct SaveLoadPlugin;

use crate::game::pieces::components::{Actor, Walk, Piece, Health, Melee, Occupier, Stats, Npc, Monster};
use crate::game::player::Player;
use crate::game::tileboard::components::BoardPosition;
use crate::globals::SCENE_FILE_PATH;
use crate::states::{GameState, AppState};
use crate::map_builders::map::Map;


// Add a new entry in SaveEntity for saving specific components.
// Add the new component in Snapshot function for has_component_to_save and SaveEntity::new()
// Add this new component in the load function too.
impl Plugin for SaveLoadPlugin{
    fn build(&self, app: &mut App) {
        app         
            //.add_systems(OnEnter(GameState::SaveGame), save_game)
            .add_systems(Update, save_game.run_if(should_save))
            .add_systems(OnEnter(GameState::LoadGame), load_game)           
            ;         
    }
}


#[derive(Resource)]
pub struct ShouldSave {
    pub to_save: bool
}


pub fn has_save_file() -> bool {
    Path::new(SCENE_FILE_PATH).exists()
}

pub fn should_save(
    must_save: Res<ShouldSave>
) -> bool {
    must_save.to_save
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveState {
    map: Map,
    entities: Vec<SaveEntity>,
}

// Bool if marker, Option if data.
#[derive(Debug, Serialize, Deserialize)]
struct SaveEntity {
    entity: Entity,
    player: bool, 
    //skills: Option<Skills>,
    stats: Option<Stats>,
    npc: bool, 
    monster: bool,
    piece: Option<Piece>,
    position: Option<BoardPosition>,
    health: Option<Health>,
    //actor: Option<Actor>, //actor can't be added there. Need to be put back on load with some logic..
    walk: bool,
    melee: Option<Melee>,
    occupier: bool,
}

impl SaveState {
    pub fn create(world: &mut World) -> Self {
        println!("Saving... savestate start.");
        let map = world.get_resource::<Map>().unwrap().clone();
        println!("Saving... map unwraped.");
        SaveState {
            map,
            entities: SaveState::snapshot_entities(world),
        }
    }

    fn snapshot_entities(world: &World) -> Vec<SaveEntity> {
        println!("Saving.... Snapshot entities.");
        let archetypes = world.archetypes();
        let all_archetypes: Vec<&Archetype> = archetypes
            .iter()
            .filter(|archetype| match archetype.id() {
                ArchetypeId::EMPTY |ArchetypeId::INVALID => false,
                _ => true,
            })
            .collect();

        let mut entities = Vec::with_capacity(all_archetypes.len());

        for archetype in all_archetypes {
             //println!("Archetype id is {:?}", archetype.id());

            for archetype_entity in archetype.entities() {

                let current_entity = &archetype_entity.entity();
    
                let mut has_component_to_save = false;
                if world.get::<Player>(world.entity(*current_entity).id()).is_some()
                || world.get::<Npc>(world.entity(*current_entity).id()).is_some()
                || world.get::<Monster>(world.entity(*current_entity).id()).is_some()
                || world.get::<Stats>(world.entity(*current_entity).id()).is_some()
                || world.get::<Piece>(world.entity(*current_entity).id()).is_some()
                || world.get::<Walk>(world.entity(*current_entity).id()).is_some()
                || world.get::<Health>(world.entity(*current_entity).id()).is_some()
                || world.get::<Melee>(world.entity(*current_entity).id()).is_some()
                //|| world.get::<Occupier>(world.entity(*current_entity).id()).is_some()    //TODO: As for Boardposition, Tile like Wall use Occupier. This has to change!
                {
                    has_component_to_save = true
                }

                if has_component_to_save {
                    entities.push(SaveEntity {
                        entity: *current_entity,
                        player: world.get::<Player>(*current_entity).is_some(),
                        npc: world.get::<Npc>(*current_entity).is_some(),
                        monster: world.get::<Monster>(*current_entity).is_some(),
                        stats: world.get::<Stats>(*current_entity).cloned(),
                        piece: world.get::<Piece>(*current_entity).cloned(),
                        position: world.get::<BoardPosition>(*current_entity).cloned(),
                        walk: world.get::<Walk>(*current_entity).is_some(),
                        health: world.get::<Health>(*current_entity).cloned(),
                        melee: world.get::<Melee>(*current_entity).cloned(),
                        occupier: world.get::<Occupier>(*current_entity).is_some(),
                    });
                    println!("Position for entity {:?} is : {:?}", *current_entity, world.get::<BoardPosition>(*current_entity));
                }
            }        
        }
        entities
    }
}


// System with World are exclusive and can only have world as argument.
fn save_game(
    mut world: &mut World
    //commands: &mut Commands
    //mut app_state: ResMut<NextState<AppState>>,
    //mut game_state: ResMut<NextState<GameState>>,    
){
    if let Some(mut must_save) = world.get_resource_mut::<ShouldSave>(){
        must_save = world.resource_mut::<ShouldSave>();
        must_save.to_save = false;
    }
    println!("Save game!");

    let state = SaveState::create(world);
    println!("Saving... SaveState created.");
    let saved_json = serde_json::to_string(&state).unwrap();
    println!("Saving... json created.");

    //println!("Save json is {:?}", state);


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

    // Back to main menu
    // Simulate a "system" to get options we need to change the app_state & game_state at the end.
    let mut system_state: SystemState<(
        ResMut<NextState<AppState>>,
        ResMut<NextState<GameState>>,
        )> = SystemState::new(&mut world);

    let (app_state, game_state) = system_state.get_mut(&mut world);
    println!("Saved end.... Back to MainMenu.");
    
    state_back_main_menu(app_state, game_state);

}

pub fn state_back_main_menu(
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    game_state.set(GameState::Disabled);
    app_state.set(AppState::MainMenu);
}


pub fn load_game(
    //mut app_state: ResMut<NextState<AppState>>,
    //mut game_state: ResMut<NextState<GameState>>,
    mut world: &mut World
) {
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
            e.insert(Actor::default()); // Actor component can't be save, so we have to add it there if NPC or Player.
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

    /* 
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
    */

    // Back to main menu
    // Simulate a "system" to get options we need to change the app_state & game_state at the end.
    let mut system_state: SystemState<(
        ResMut<NextState<AppState>>,
        ResMut<NextState<GameState>>,
        )> = SystemState::new(&mut world);

    let (app_state, game_state) = system_state.get_mut(&mut world);
    
    state_after_load_game(app_state, game_state);

}

pub fn state_after_load_game(
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    game_state.set(GameState::Prerun); //TODO : changer quand load utilisable
    app_state.set(AppState::Game);
}
