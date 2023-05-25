//https://gist.github.com/chamons/37e8c6f8753e63eaef08bef36686c2e2

use bevy::ecs::archetype::{Archetype, ArchetypeId};
use bevy::ecs::system::SystemState;
use serde::{Deserialize, Serialize};

use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};
use std::path::Path;

pub struct SaveLoadPlugin;

use crate::game::{Player, Npc, Monster, Stats};
use crate::map_builders::map::Map;
use crate::{
    game::{GameState,ShouldSave},
    AppState,
};


pub const SCENE_FILE_PATH: &str = "assets/scenes/save.srrl";



impl Plugin for SaveLoadPlugin{
    fn build(&self, app: &mut App) {
        app         
            //.add_systems(OnEnter(GameState::SaveGame), save_game)
            .add_systems(Update, save_game.run_if(should_save))
            .add_systems(OnEnter(GameState::LoadGame), load_game)           
            ;         
    }
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

#[derive(Debug, Serialize, Deserialize)]
struct SaveEntity {
    entity: Entity,
    player: Option<Player>,
    //skills: Option<Skills>,
    stats: Option<Stats>,
    npc: Option<Npc>, 
    monster: Option<Monster>
}

impl SaveState {
    pub fn create(world: &mut World) -> Self {
        let map = world.get_resource::<Map>().unwrap().clone();
        SaveState {
            map,
            entities: SaveState::snapshot_entities(world),
        }
    }

    fn snapshot_entities(world: &World) -> Vec<SaveEntity> {
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
                //println!("entity : {:?}", current_entity);
                
                let entity = world.entity(*current_entity).id();

                /*
                if let Some(player) = world.entity(world.entity(*current_entity).id()).get::<Player>(){
                    println!("Player is {:?}", player);
                } else {
                    println!("No player");
                }
                */
                let mut has_component_to_save = false;
                if world.get::<Player>(world.entity(*current_entity).id()).is_some()
                || world.get::<Npc>(world.entity(*current_entity).id()).is_some()
                || world.get::<Monster>(world.entity(*current_entity).id()).is_some()
                || world.get::<Stats>(world.entity(*current_entity).id()).is_some()
                {
                    has_component_to_save = true
                }

                if has_component_to_save {
                    entities.push(SaveEntity {
                        entity: *current_entity,
                        player: world.get::<Player>(*current_entity).cloned(),
                        //skills: world.get::<Skills>(*entity).cloned(),
                        npc: world.get::<Npc>(*current_entity).cloned(),
                        monster: world.get::<Monster>(*current_entity).cloned(),
                        stats: world.get::<Stats>(*current_entity).cloned(),
                    });
                }
            }        
        }
        entities
    }
}



pub fn load_game_new(state: &str) -> World {
    let state: SaveState = serde_json::from_str(state).unwrap();
    let mut world = World::new();
    //world.insert_resource(state.stability);


    for entity in state.entities {
        let mut e = world.spawn_empty();
        if let Some(player) = entity.player {
            e.insert(player);
        }
        /*
        if let Some(skills) = entity.skills {
            e.insert(skills);
        }
        */
        if let Some(npc) = entity.npc {
            e.insert(npc);
        }
        if let Some(monster) = entity.monster {
            e.insert(monster);
        }
        if let Some(stats) = entity.stats {
            e.insert(stats);
        }
    }
    world
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
    let mut saved_json = serde_json::to_string(&state).unwrap();

    println!("Save json is {:?}", state);


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

    // Back to main menu
    // Simulate a "system" to get options we need to change the app_state & game_state at the end.
    let mut system_state: SystemState<(
        ResMut<NextState<AppState>>,
        ResMut<NextState<GameState>>,
        )> = SystemState::new(&mut world);

    let (mut app_state, mut game_state) = system_state.get_mut(&mut world);
    
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

    /*

    let data = fs::read_to_string("assets/scenes/load_scene_example.scn.ron")
    .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let state: SaveState = serde_json::from_str(&data).unwrap();

    world.insert_resource(state.map);

    for entity in state.entities {
        let mut e = world.spawn_empty();
        if let Some(player) = entity.player {
            e.insert(player);
        }
        /*
        if let Some(skills) = entity.skills {
            e.insert(skills);
        }
        */
        if let Some(npc) = entity.npc {
            e.insert(npc);
        }
        if let Some(monster) = entity.monster {
            e.insert(monster);
        }
        if let Some(stats) = entity.stats {
            e.insert(stats);
        }
    }

    /* 
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
    */

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
    game_state.set(GameState::NewGame); //TODO : changer quand load utilisable
    app_state.set(AppState::Game);
}

/*
fn get_components_ids<'a>(world: &'a World, entity: &Entity) -> Option<impl Iterator<Item=ComponentId> + 'a>
{
    // components and entities are linked through archetypes
    for archetype in world.archetypes().iter()
    {
        for archetype_entity in archetype.entities() {
            let current_entity = archetype_entity.entity();
            if current_entity == *entity { return Some(archetype.components()) }
        }
        
    }
    None
}

fn component_id_to_component_info(world: &World, component_id: ComponentId) -> Option<&ComponentInfo>
{
    let components = world.components();
    components.get_info(component_id)
}

fn extract_component_name(component_info: &ComponentInfo) -> &str
{
    component_info.name()
}
*/