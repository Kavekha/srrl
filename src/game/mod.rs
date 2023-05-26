// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::player::PlayerPlugin;
use self::npc::NpcPlugin;

use crate::{
    globals::SHOW_MAPGEN_VISUALIZER,
    map_builders::{
        map::Map, 
        TileMapPlugin    
    },
    game::spawners::{spawn_npc, spawn_player},
    map_builders::{
        random_builder,
    },    
    menus::{
        victory::VictoryPlugin,
        gameover::GameOverPlugin,
    },    
    ecs_elements::{
        components::{Monster},
        resources::{ShouldSave, MapGenHistory, GameState}, GridPosition,
    }, 
    render::GraphicsPlugin
};

pub mod player;
pub mod npc;
pub mod spawners;




pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::new())
            .insert_resource(ShouldSave{to_save: false})
            .add_plugin(PlayerPlugin)
            .add_plugin(VictoryPlugin)
            .add_plugin(TileMapPlugin)
            .add_plugin(NpcPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(GraphicsPlugin)

            .add_systems(OnEnter(GameState::NewGame),init_new_game)
            ;
    }
}


fn init_new_game(
    mut commands: Commands, 
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut builder = random_builder();
    builder.build_map();

    if SHOW_MAPGEN_VISUALIZER {
        let mapgen_history = MapGenHistory{
            history: builder.build_data.history.clone(),
            index: 0,
        };
        commands.insert_resource(mapgen_history);
    }

    // init player  // TODO : ChainSystem ? But builder can't be made a resource cause of Dyn / Life time.
    // Logic spawning only.
    let player = spawn_player(&mut commands);

    let player_starting_position = builder.get_starting_position();    
    println!("Player: Starting position = {:?}", player_starting_position);
    commands
        .entity(player)
        .insert(GridPosition{
            x:player_starting_position.0,
            y:player_starting_position.1
        });


    // Other entities. //TODO: Can't spawn different npc types: just one.
    let entities_pos = builder.spawn_entities();
    for entity_position in entities_pos {

        println!("NPC: Starting position = {:?}", entity_position);

        let npc = spawn_npc(&mut commands);

        //TODO : Le nom pour le moment est dans le spawner.
        commands
        .entity(npc)
        .insert(GridPosition{
            x:entity_position.0,
            y:entity_position.1
        })
        .insert(Monster)
        ;
    }

    builder.build_data.map.populate_blocked(); 

    commands.insert_resource(builder.build_data.map.clone());

    if !SHOW_MAPGEN_VISUALIZER {
        game_state.set(GameState::Prerun);  //TODO : Pas a ce systeme de gerer les changements de state.
    } else {
        game_state.set(GameState::MapGeneration);  
    }
}


