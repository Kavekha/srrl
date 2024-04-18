// Game Plugin + Component & enum go there + new game setup.
use bevy::prelude::*;

use self::combat::CombatPlugin;
use self::movements::MovementPlugin;
use self::pieces::components::{GameElement, Npc};
use self::player::{PlayerPlugin, Player, cursor::CursorPlugin};
use self::tileboard::components::GameMap;
use self::ui::UiPlugin;
use self::menus::MenuPlugin;
use self::manager::ManagerPlugin;
use self::gamelog::GameLogsPlugin;
use self::visibility::ViewPlugin;

pub mod combat;
pub mod pieces;
pub mod player;
pub mod tileboard;
pub mod ui;
pub mod menus;
pub mod states;
pub mod gamelog;
pub mod visibility;

mod commons;
mod manager;
mod movements;
 

use crate::commons::despawn_component;
use crate::game::tileboard::components::ExitMapTile;
use crate::game::states::GameState;
use crate::engine::render::components::{GameMapRender, GameCursorRender};
use crate::map_builders::map::Map;
use crate::map_builders::{MAPHEIGHT, MAPWIDTH};


//--
// Movement
pub const POSITION_TOLERANCE: f32 = 0.01;

//pub const BASE_SPEED: f32 = 5.0;                        // La vitesse de base d'un deplacement (legacy)
pub const SPEED_MULTIPLIER: f32 = 5.0;                  // FAST debug / balance on speed movement.
pub const CURSOR_SPEED: f32 = 20.0;                     // Cursor.
pub const BASE_SPEED_PATH_ANIMATOR_UPDATE: f32 = 10.0;   // Vitesse de base d'une animation de deplacement.
pub const BASE_SPEED_CAMERA_SMOOTH_FOLLOW: f32 = 8.0;

// Anim
pub const BASE_TIME_FRAME_EFFECT: f32 = 0.1;        // Vitesse de defilement des etapes de l'animation d'un sprite / FX.


//---------

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::new(MAPWIDTH, MAPHEIGHT, "Underground"))

            .add_plugins(PlayerPlugin)            
            .add_plugins(CursorPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(UiPlugin)     
            .add_plugins(CombatPlugin)
            .add_plugins(ManagerPlugin)
            .add_plugins(GameLogsPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(ViewPlugin)

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
    despawn_game_thingies: Query<Entity, With<GameElement>>,
    
) {
    println!("Cleaning Game Screen now.");
    despawn_component(despawn_npc, &mut commands);
    despawn_component(despawn_gamemap, &mut commands);
    println!("Despawn gamemap render");
    despawn_component(despawn_gamemap_render, &mut commands);  
    println!("Gamemap rendered despawned.");
    despawn_component(despawn_player, &mut commands);
    despawn_component(despawn_gamecursor, &mut commands);
    despawn_component(despawn_exit, &mut commands);
    println!("Despawn non-specific game elements.");
    despawn_component(despawn_game_thingies, &mut commands);
}

