// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::combat::CombatPlugin;
use self::pieces::components::Npc;
use self::player::{PlayerPlugin, Player};
use self::tileboard::TileBoardPlugin;
use self::player::cursor::CursorPlugin;
use self::tileboard::components::GameMap;
use self::ui::UiPlugin;

pub mod player;
pub mod pieces;
pub mod tileboard;
pub mod rules;
pub mod ui;
pub mod combat;

pub use tileboard::components::Tile;

use crate::game::pieces::components::Monster;
use crate::game::pieces::spawners::{spawn_player, spawn_npc, spawn_exit};
use crate::game::tileboard::components::{BoardPosition, ExitMapTile};
use crate::map_builders::components::MapGenHistory;
use crate::render::components::{GameMapRender, GameCursorRender};
use crate::save_load_system::ShouldSave;
use crate::{
    globals::SHOW_MAPGEN_VISUALIZER,
    map_builders::map::Map,
    map_builders::random_builder,
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

            .add_plugins(PlayerPlugin)
            .add_plugins(VictoryPlugin)
            .add_plugins(GameOverPlugin)
            .add_plugins(GraphicsPlugin)
            .add_plugins(TileBoardPlugin)
             .add_plugins(CursorPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(CombatPlugin)
            
            .add_systems(OnEnter(GameState::NewGame),init_new_game)
            .add_systems(OnExit(GameState::GameMap), clean_game_screen)
            ;
    }
}



pub fn despawn_component<T: Component>(
    to_despawn: Query<Entity, With<T>>, 
    mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>, 
    commands: &mut Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clean_game_screen(
    mut commands: Commands,
    despawn_npc: Query<Entity, With<Npc>>,    
    despawn_gamemap: Query<Entity, With<GameMap>>,
    despawn_gamemap_render: Query<Entity, With<GameMapRender>>,
    despawn_player: Query<Entity, With<Player>>,
    despawn_gamecursor: Query<Entity, With<GameCursorRender>>,
    despawn_exit: Query<Entity, With<ExitMapTile>>,
    
) {
    despawn_screen(despawn_npc, &mut commands);
    despawn_screen(despawn_gamemap, &mut commands);
    despawn_screen(despawn_gamemap_render, &mut commands);    
    despawn_screen(despawn_player, &mut commands);
    despawn_screen(despawn_gamecursor, &mut commands);
    despawn_screen(despawn_exit, &mut commands);
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

    // EXIT 
    let exit_position = builder.get_exit_position();
    let exit = spawn_exit(&mut commands);
    commands.entity(exit).insert(BoardPosition{ v:exit_position});
    
    
    builder.build_data.map.populate_blocked(); 

    commands.insert_resource(builder.build_data.map.clone());

    if !SHOW_MAPGEN_VISUALIZER {
        game_state.set(GameState::Prerun);  
    } else {
        game_state.set(GameState::MapGeneration);  
    }
}


