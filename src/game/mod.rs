// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::manager::ManagerPlugin;
use self::pieces::components::Npc;
use self::player::PlayerPlugin;
use self::tileboard::TileBoardPlugin;
use self::actions::ActionsPlugin;
use self::player::cursor::CursorPlugin;
use self::ui::UiPlugin;

pub mod player;
pub mod pieces;
pub mod actions;
pub mod tileboard;
pub mod manager;
pub mod rules;
pub mod ui;

pub use tileboard::components::{Tile, GridPosition};


use crate::despawn_screen;
use crate::ecs_elements::MapGenHistory;
use crate::game::pieces::components::Monster;
use crate::game::pieces::spawners::{spawn_player, spawn_npc};
use crate::game::tileboard::components::BoardPosition;
use crate::save_load_system::ShouldSave;
use crate::{
    globals::SHOW_MAPGEN_VISUALIZER,
    map_builders::{
        map::Map, 
    },
    map_builders::{
        random_builder,
    },    
    menus::{
        victory::VictoryPlugin,
        gameover::GameOverPlugin,
    }, 
    render::GraphicsPlugin, states::GameState
};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::new())
            .insert_resource(ShouldSave{to_save: false})

            .add_plugin(PlayerPlugin)
            .add_plugin(VictoryPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(GraphicsPlugin)
            .add_plugin(TileBoardPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(ManagerPlugin)
            .add_plugin(CursorPlugin)
            .add_plugin(UiPlugin)
            
            .add_systems(OnEnter(GameState::NewGame),init_new_game)
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Npc>) //TODO : Remove NPC ? Add a full "end game" function?
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
        .insert(BoardPosition{ v:player_starting_position })
    ;

    // Other entities. //TODO: Can't spawn different npc types: just one.
    let entities_pos = builder.spawn_entities();
    for entity_position in entities_pos {

        println!("NPC: Starting position = {:?}", entity_position);

        let npc = spawn_npc(&mut commands);

        //TODO : Le nom pour le moment est dans le spawner.
        commands
        .entity(npc)
        .insert(BoardPosition{ v:entity_position})
        .insert(Monster)
        ;
    }
    
    builder.build_data.map.populate_blocked(); 

    commands.insert_resource(builder.build_data.map.clone());

    if !SHOW_MAPGEN_VISUALIZER {
        game_state.set(GameState::Prerun); 
    } else {
        game_state.set(GameState::MapGeneration);  
    }
}


