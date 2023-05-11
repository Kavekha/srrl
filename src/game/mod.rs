// Game Plugin + Component & enum go there.
use bevy::prelude::*;

use self::tilemap::TileMapPlugin;
use self::player::PlayerPlugin;
use self::victory::VictoryPlugin;


pub mod player;
pub mod tilemap;
pub mod victory;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_plugin(VictoryPlugin)
            .add_plugin(TileMapPlugin);
    }
}

// Enum
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Disabled,
    NewGame,
    CharacterCreation,
    GameMap,
    VictoryScreen
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