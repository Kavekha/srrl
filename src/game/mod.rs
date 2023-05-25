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
        pathfinding::grid_to_world_position,
    },    
    menus::{
        victory::VictoryPlugin,
        gameover::GameOverPlugin,
    },    
    ecs_elements::{
        components::{Monster},
        resources::{ShouldSave, MapGenHistory, GameState},
    }
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
            .add_systems(OnEnter(GameState::NewGame), init_new_game)
            ;
    }
}

fn init_new_game(
    mut commands: Commands, 
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
){
    let mut builder = random_builder();
    builder.build_map();

    if SHOW_MAPGEN_VISUALIZER {
        let mapgen_history = MapGenHistory{
            history: builder.build_data.history.clone(),
            index: 0,
        };
        commands.insert_resource(mapgen_history);
    }



    let starting_position = builder.get_starting_position();    //TODO
    let (x, y) = grid_to_world_position(starting_position.0,starting_position.1);   //TODO: Placeholder
    spawn_player(&mut commands, &asset_server, x, y);
    //spawn_player(&mut commands, &ascii,x, y);

    let entities_pos = builder.spawn_entities();
    for position in entities_pos {
        let (x, y) = grid_to_world_position(position.0, position.1);    //TODO: Refacto: Where should the grid_to_world_position done? In the Spawning function no?
        //let ghoul = spawn_npc(&mut commands, &ascii, x, y, format!("Ghoul"), 2);
        let ghoul = spawn_npc(&mut commands, &asset_server, x, y, format!("Ghoul"));
        commands.entity(ghoul).insert(Monster);
    }

    builder.build_data.map.populate_blocked();  //TODO : Refacto: Où je fous ça moi?

    commands.insert_resource(builder.build_data.map.clone());

    if !SHOW_MAPGEN_VISUALIZER{
        game_state.set(GameState::GameMap);  
    } else {
        game_state.set(GameState::MapGeneration);  
    }
}


