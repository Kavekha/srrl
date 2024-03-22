// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::combat::CombatPlugin;
use self::pieces::components::Npc;
use self::player::{PlayerPlugin, Player, cursor::CursorPlugin};
use self::tileboard::components::GameMap;
use self::ui::UiPlugin;
use self::menus::MenuPlugin;
use self::manager::ManagerPlugin;

pub mod combat;
pub mod pieces;
pub mod player;
pub mod tileboard;
pub mod rules;
pub mod ui;
pub mod menus;
pub mod states;
pub mod manager;


use crate::game::tileboard::components::ExitMapTile;
use crate::game::states::GameState;
use crate::engine::render::components::{GameMapRender, GameCursorRender};
use crate::engine::save_load_system::ShouldSave;
use crate::map_builders::map::Map;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::new())
            .insert_resource(ShouldSave{to_save: false})

            .add_plugins(PlayerPlugin)            
            .add_plugins(CursorPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(UiPlugin)     
            .add_plugins(CombatPlugin)
            .add_plugins(ManagerPlugin)

            .add_systems(OnEnter(GameState::Disabled), clean_game_screen)
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
    println!("Cleaning Game Screen now.");
    despawn_screen(despawn_npc, &mut commands);
    despawn_screen(despawn_gamemap, &mut commands);
    despawn_screen(despawn_gamemap_render, &mut commands);    
    despawn_screen(despawn_player, &mut commands);
    despawn_screen(despawn_gamecursor, &mut commands);
    despawn_screen(despawn_exit, &mut commands);
}


