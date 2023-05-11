// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::tilemap::TileMapPlugin;
use self::player::PlayerPlugin;
use self::victory::VictoryPlugin;

use crate::{
    TILE_SIZE,
    map_builders::map::Map,
    game::player::spawn_player,
    ascii::AsciiSheet,
};

pub mod player;
pub mod tilemap;
pub mod victory;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_plugin(VictoryPlugin)
            .add_plugin(TileMapPlugin)
            //Character creation.
            .add_systems(OnEnter(GameState::NewGame), create_new_game_player)
            //Map creation.
            .add_systems(OnEnter(GameState::Setup),create_new_game_map) 
            //Char placement.
            .add_systems(OnEnter(GameState::Prerun), create_new_game_positioning_player  
            );
    }
}

fn create_new_game_player (
    mut commands: Commands, 
    ascii: Res<AsciiSheet>,
    mut game_state: ResMut<NextState<GameState>>
){
    //We create the player.
    spawn_player(commands, ascii);    
    println!("Personnage cree");
    game_state.set(GameState::Setup);  
}

fn create_new_game_map(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>
){
    println!("Create new game debut");
    //We create the map.
    let map = Map::new_map_rooms_and_corridors();
    commands.insert_resource(Game {map: map });   
    println!("Map creee et inseree comme ressource");
    game_state.set(GameState::Prerun);  
}

fn create_new_game_positioning_player(    
    mut player_query: Query<&mut Transform, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
    game: Res<Game>
) {
    //We give the player a position from the first room center.
    let mut player_transform = player_query.single_mut();       //TODO check si Player existe. Querry faites avant la creation?
    let (x, y) = game.map.rooms[0].center();                        // TODO check si Map existe. On l'a placé dans la resource avant.
    player_transform.translation.x = x as f32 * TILE_SIZE;
    player_transform.translation.y = -(y as f32) * TILE_SIZE;
    println!("player new position : {},{}", player_transform.translation.x, player_transform.translation.y); 

    game_state.set(GameState::GameMap); 
}


// Enum
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Disabled,
    NewGame,    // Nouvelle partie, creation perso
    Setup,      // Nouvelle map generée
    Prerun,     // Placement du personnage. TODO : Dans Tilemap avec la creation "physique"?
    GameMap,
    VictoryScreen
}  


// Resource
#[derive(Resource)]
pub struct Game {
    map: Map
}


// Components
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