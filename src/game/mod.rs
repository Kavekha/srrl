// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::tilemap::TileMapPlugin;
use self::player::PlayerPlugin;
use self::victory::VictoryPlugin;
use self::gameover::GameOverPlugin;
use self::npc::NpcPlugin;

use crate::{
    TILE_SIZE,
    map_builders::{
        map::Map,
        build_random_map,
    },
    ascii::AsciiSheet,
    game::spawners::{spawn_npc, spawn_player},
    map_builders::pathfinding::{Position, grid_to_world_position},
};

pub mod player;
pub mod tilemap;
pub mod victory;
pub mod npc;
pub mod gameover;
pub mod spawners;


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
    println!("Create new map");
    //We create the map.
    let (new_map, starting_position) = build_random_map();
 
    //spawn player
    let (x, y) = grid_to_world_position(starting_position.0, starting_position.1);  
 
    spawn_player(&mut commands, &ascii, x, y);

    for room in new_map.rooms.iter().skip(1) {
        let (x, y) = grid_to_world_position(room.center().0, room.center().1);
        let ghoul = spawn_npc(&mut commands, &ascii, x, y, format!("Ghoul"), 2);
        commands.entity(ghoul).insert(Monster);
    }

    // We don't need the map, let's make it a resource for the others.
    commands.insert_resource(new_map);   
    println!("Map creee et inseree comme ressource");
    game_state.set(GameState::GameMap);  
}


// Enum
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Disabled,
    NewGame,    // Nouvelle partie, setup Map & player creation
    GameMap,    // La map et le perso qui s'y balade.
    GameOverScreen,
    VictoryScreen,
}  

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