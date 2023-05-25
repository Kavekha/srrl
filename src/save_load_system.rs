use bevy::ecs::archetype::{Archetype, ArchetypeId};
use bevy::ecs::system::SystemState;
use bevy::ecs::world;
//use moonshine_save::prelude::*;
use serde::{Deserialize, Serialize};

use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};
use std::path::Path;

pub struct SaveLoadPlugin;

use crate::game::{Player, Npc, Monster, Stats};
use crate::map_builders::map::Map;
use crate::{
    game::GameState,
    AppState,
};


const SAVE_PATH: &str = "army.ron";

pub const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";



impl Plugin for SaveLoadPlugin{
    fn build(&self, app: &mut App) {
        app         
            .add_systems(OnEnter(GameState::SaveGame), save_game)
            .add_systems(OnEnter(GameState::LoadGame), load_game)           
            ;         
    }
}

pub fn has_save_file() -> bool {
    Path::new("assets/scenes/load_scene_example.scn.ron").exists()
}


#[derive(Debug, Serialize, Deserialize)]
struct SaveState {
    //stability: Stability,
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
        //let stability = world.get_resource::<Stability>().unwrap().clone();


        SaveState {
            //stability,
            entities: SaveState::snapshot_entities(world),
        }
    }

    fn snapshot_entities(world: &World) -> Vec<SaveEntity> {
        let archetypes = world.archetypes();
        let all_archetypes: Vec<&Archetype> = archetypes
            .iter()
            .filter(|archetype| match archetype.id() {
                ArchetypeId::EMPTY | ArchetypeId::INVALID => false,
                _ => true,
            })
            .collect();

        let mut entities = Vec::with_capacity(all_archetypes.len());
        for archetype in all_archetypes {
            for archetype_entity in archetype.entities() {
                let current_entity = &archetype_entity.entity();
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
        entities
    }
}

pub fn save_game(world: &mut World) -> String {
    let state = SaveState::create(world);
    serde_json::to_string(&state).unwrap()
}

pub fn load_game(state: &str) -> World {
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
fn save_game_old(
    mut world: &mut World
    //commands: &mut Commands
    //mut app_state: ResMut<NextState<AppState>>,
    //mut game_state: ResMut<NextState<GameState>>,    
){
    println!("Save game!");
  
    let scene_world = World::new();

    // Insert and save resources: Je n'ai besoin que de la map, elle contient les infos dont j'ai besoin.
    // TODO: Dans l'ideal, la Seed de la map me suffirait pour la reproduire.
    // REMEMBER: 1. On recupere une Option: P-e y a une map, p-e pas.  2. Je créé une variable Map avec le contenu de Some current_map (Il y en a) et je peux travailler avec car oui, il y en a.
    // Else : agir si y en a pas. current_map.is_none => y en a pas, mais je me fiche du contenu. current_map.is_some => y en a, mais je me fiche aussi du contenu.
    /*
    let mapcopy = world.get_resource::<Map>();
    if let Some(copied_map) = mapcopy {
        let map = copied_map.clone();
        scene_world.insert_resource(map);
            
   
    }
    */

    let type_registry = world.resource::<AppTypeRegistry>();    // Tous les trucs que j'ai registered avec .register_type<Component)()
    //let scene = DynamicScene::from_world(&scene_world, type_registry);
    let scene = DynamicScene::from_world(&scene_world, &type_registry);

    // Je serialise la scene.
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    info!("{}", serialized_scene);

    // Formule magique pour enregistrer dans un fichier.
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create("assets/scenes/load_scene_example.scn.ron")       //format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
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
    
    change_states_after_save(app_state, game_state);

}

pub fn change_states_after_save(
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    game_state.set(GameState::Disabled);
    app_state.set(AppState::MainMenu);
}


pub fn load_game_old(
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    println!("Load game!");

    /* 
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
    */
    app_state.set(AppState::Game);
    game_state.set(GameState::NewGame);
        //game_state.set(GameState::GameMap);
}