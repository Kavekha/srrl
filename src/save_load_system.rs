use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};
use std::path::Path;

pub struct SaveLoadPlugin;

use crate::{
    game::GameState,
    AppState,
};


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



// System with World are exclusive and can only have world as argument.
fn save_game(
    //world: &mut World,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,    
){
    println!("Save game!");
    // New world we will save.
    let mut scene_world = World::new();

    /* 
    // Components to save
    let mut component_b = ComponentB::from_world(world);

    // Components spawn in the world we'll save.
    scene_world.spawn((
        component_b
    ));

    // Insert and save resources.
    scene_world.insert_resource(ResourceA { score: 1 });


    let type_registry = world.resource::<AppTypeRegistry>();

    let scene = DynamicScene::from_world(&scene_world, type_registry);
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    info!("{}", serialized_scene);

    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
        */

    // Back to main menu
    game_state.set(GameState::Disabled);
    app_state.set(AppState::MainMenu);
}

pub fn load_game(
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