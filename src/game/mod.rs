// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::tilemap::TileMapPlugin;
use self::player::PlayerPlugin;
use self::victory::VictoryPlugin;
use self::gameover::GameOverPlugin;
use self::npc::NpcPlugin;

use crate::{
    SHOW_MAPGEN_VISUALIZER,
    map_builders::{
        map::Map,
        MapGenHistory,
        pathfinding::{Position, grid_to_world_position},
    },
    ascii::AsciiSheet,
    game::spawners::{spawn_npc, spawn_player},
    map_builders,
    
    
};

pub mod player;
pub mod tilemap;
pub mod victory;
pub mod npc;
pub mod gameover;
pub mod spawners;


// Enum
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Disabled,
    NewGame,    // Nouvelle partie, setup Map & player creation
    MapGeneration,
    GameMap,    // La map et le perso qui s'y balade.
    GameOverScreen,
    VictoryScreen,
}  

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::new())
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
    ascii: Res<AsciiSheet>,
    mut game_state: ResMut<NextState<GameState>>
){
    let mut builder = map_builders::random_builder();
    builder.build_map();

    let mapgen_history = builder.build_data.history.clone();

    let starting_position = builder.get_starting_position();    //TODO
    let (x, y) = (0.0,0.0);   //TODO: Placeholder
    spawn_player(&mut commands, &ascii, x, y);

    builder.spawn_entities()    //TODO

}

/*
fn init_new_game(
    mut commands: Commands, 
    ascii: Res<AsciiSheet>,
    mut game_state: ResMut<NextState<GameState>>
){
    println!("Asking a boxed Map Builder object from the factory"); //https://bfnightly.bracketproductions.com/chapter_23.html
    let mut builder =  map_builders::random_builder();
    builder.build_map();

    //spawn player
    let starting_position = builder.get_starting_position();
    let (x, y) = grid_to_world_position(starting_position.0, starting_position.1);   //TODO: Refacto: Where should the grid_to_world_position done? In the Spawning function no?
    spawn_player(&mut commands, &ascii, x, y);

    //spawn entities
    let entities_pos = builder.spawn_entities().clone();
    for position in entities_pos {
        let (x, y) = grid_to_world_position(position.0, position.1);    //TODO: Refacto: Where should the grid_to_world_position done? In the Spawning function no?
        let ghoul = spawn_npc(&mut commands, &ascii, x, y, format!("Ghoul"), 2);
        commands.entity(ghoul).insert(Monster);
    }

    //Get history if mapgen active.
    let mapgen_history = MapGenHistory{history: builder.get_snapshot_history(), index: 0};
    commands.insert_resource((mapgen_history));

    // We don't need the map, let's make it a resource for the others.
    commands.insert_resource(builder.get_map());   
    println!("Map creee et inseree comme ressource");

    if !SHOW_MAPGEN_VISUALIZER{
        game_state.set(GameState::GameMap);  
    } else {
        game_state.set(GameState::MapGeneration);  
    }
}
 */



#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stats {
    speed: f32
}

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct TileExit;

#[derive(Component)]
pub struct GameMap;

#[derive(Component)]
pub struct Npc{
    pub home: Position,
}

#[derive(Component)]
pub struct Monster;