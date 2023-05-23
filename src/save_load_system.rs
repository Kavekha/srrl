use bevy::ecs::system::SystemState;
use bevy::transform::commands;
use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};
use std::path::Path;

pub struct SaveLoadPlugin;

use crate::game::{Player, npc, Monster, Stats, Npc};
use crate::map_builders::map::Map;
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
    mut world: &mut World,
    //commands: &mut Commands
    //mut app_state: ResMut<NextState<AppState>>,
    //mut game_state: ResMut<NextState<GameState>>,    
){
    println!("Save game!");
    // New world we will save.
    let mut scene_world = World::new();

    // Components to save
    let mut player = Player::from_world(world);
    let mut stats = Stats::from_world(world);
    let mut npc = Npc::from_world(world);
    let mut monster = Monster::from_world(world);

    // Components spawn in the world we'll save.
    // TODO: Là c'est la merde: Faut tout recréer...???!
    scene_world.spawn((
        player,
        stats,
        npc,
        monster
    ));

    // Insert and save resources: Je n'ai besoin que de la map, elle contient les infos dont j'ai besoin.
    // TODO: Dans l'ideal, la Seed de la map me suffirait pour la reproduire.
    // REMEMBER: 1. On recupere une Option: P-e y a une map, p-e pas.  2. Je créé une variable Map avec le contenu de Some current_map (Il y en a) et je peux travailler avec car oui, il y en a.
    // Else : agir si y en a pas. current_map.is_none => y en a pas, mais je me fiche du contenu. current_map.is_some => y en a, mais je me fiche aussi du contenu.
    let current_map = world.get_resource::<Map>();
    if let Some(map) = current_map {
        scene_world.insert_resource(Map{
            tiles: map.tiles.clone(),
            width: map.width.clone(),
            height: map.height.clone(),
            blocked: map.blocked.clone()});
    }

    let type_registry = world.resource::<AppTypeRegistry>();    // Tous les trucs que j'ai registered avec .register_type<Component)()
    let scene = DynamicScene::from_world(&scene_world, type_registry);

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