// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::combat::CombatPlugin;
use self::movements::MovementPlugin;
use self::pieces::components::Npc;
use self::player::{PlayerPlugin, Player, cursor::CursorPlugin};
use self::tileboard::components::GameMap;
use self::ui::UiPlugin;
use self::menus::MenuPlugin;
use self::manager::ManagerPlugin;
use self::gamelog::GameLogsPlugin;

pub mod combat;
pub mod pieces;
pub mod player;
pub mod tileboard;
pub mod ui;
pub mod menus;
pub mod states;
pub mod gamelog;

mod commons;
mod manager;
mod movements;
 

use crate::commons::despawn_component;
use crate::game::tileboard::components::ExitMapTile;
use crate::game::states::GameState;
use crate::engine::render::components::{GameMapRender, GameCursorRender};
use crate::map_builders::map::Map;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::new())

            .add_plugins(PlayerPlugin)            
            .add_plugins(CursorPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(UiPlugin)     
            .add_plugins(CombatPlugin)
            .add_plugins(ManagerPlugin)
            .add_plugins(GameLogsPlugin)
            .add_plugins(MovementPlugin)

            .add_systems(OnEnter(GameState::Disabled), clean_game_screen)
            ;
    }
}
 
 
fn clean_game_screen(
    mut commands: Commands,
    despawn_npc: Query<Entity, With<Npc>>,    
    despawn_gamemap: Query<Entity, With<GameMap>>,
    despawn_gamemap_render: Query<Entity, With<GameMapRender>>,
    despawn_player: Query<Entity, With<Player>>,
    despawn_gamecursor: Query<Entity, With<GameCursorRender>>,
    despawn_exit: Query<Entity, With<ExitMapTile>>,
    
) {
    println!("Cleaning Game Screen now.");
    despawn_component(despawn_npc, &mut commands);
    despawn_component(despawn_gamemap, &mut commands);
    despawn_component(despawn_gamemap_render, &mut commands);    
    despawn_component(despawn_player, &mut commands);
    despawn_component(despawn_gamecursor, &mut commands);
    despawn_component(despawn_exit, &mut commands);
}

