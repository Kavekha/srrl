// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::tilemap::TileMapPlugin;
use self::player::PlayerPlugin;
use self::victory::VictoryPlugin;
use self::gameover::GameOverPlugin;
use self::npc::NpcPlugin;

use crate::{
    TILE_SIZE,
    map_builders::map::Map,
    map_builders::pathfinding::world_to_grid_position,
    game::player::spawn_player, 
    ascii::AsciiSheet,
    game::npc::{spawn_npc, Npc},
};

pub mod player;
pub mod tilemap;
pub mod victory;
pub mod npc;
pub mod gameover;


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
    let map = Map::new_map_rooms_and_corridors();
 
    //spawn player
    // How to do this with no player_x, player_y?
    let (x, y) = map.rooms[0].center();  
    let player_x = x as f32* TILE_SIZE;
    let player_y = -(y as f32) * TILE_SIZE;

    spawn_player(&mut commands, &ascii, player_x, player_y);

    //spawn enemies
    let mut rooms = map.rooms.len() - 2; 
    for _i in 0.. 10{
        if rooms <= 0 {
            break;
        } else {
            let (x, y) = map.rooms[rooms].center();
            let npc_x = x as f32 * TILE_SIZE;
            let npc_y = -(y as f32) * TILE_SIZE;
            println!("NPC from {},{} has been spawned in world units {},{}", x, y, npc_x, npc_y);
            let (world_to_grid_x, world_to_grid_y) = world_to_grid_position(npc_x, npc_y);
            println!("Conversion world_to_grid_position donne : {},{}", world_to_grid_x, world_to_grid_y);

            spawn_npc(&mut commands, &ascii, npc_x, npc_y);  

            rooms -= 1;
        }        
    }

    // We don't need the map, let's make it a resource for the others.
    commands.insert_resource(map);   
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