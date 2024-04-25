use bevy::prelude::*;

// Versions
pub const VERSION: &str = "0.20.s";
pub const RELEASE: &str = "R0.4";

// Min - Max VOLUME
pub const MIN_VOLUME: f32 = 0.0;
pub const MAX_VOLUME: f32 = 5.0;
pub const DEFAULT_VOLUME: f32 = 1.0;

// WINDOWS
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;
//pub const BASE_SCREEN_SCALE: f32 = 1.0; //DEBUG: Scaling multiplier


// TODO : En attente de la R0.5 et du Loader
// RENDERING
pub const CHAR_SIZE: f32 = 16.0; 
pub const STANDARD_TILE_SIZE:i32 = 32;  //64;

// RENDERING ORDER
pub const ORDER_FLOOR: f32 = 0.0;
pub const ORDER_CORPSE: f32 = 5.0;
pub const ORDER_WALL: f32 = 10.0;
pub const ORDER_NPC: f32 = 15.0;
pub const ORDER_EXIT: f32 = 20.0;
pub const ORDER_PLAYER: f32 = 25.0;
pub const ORDER_EFFECT: f32 = 30.0;
pub const ORDER_MARKER: f32 = 35.0;
pub const ORDER_CURSOR: f32 = 40.0;

// Tiles    //TEMPORARY : TODO : Loader.
pub const MAP_EXIT: &str = "exit";
pub const MAP_WALL_1: &str = "wall_1";
pub const MAP_WALL_2: &str = "wall_2";
pub const MAP_WALL_3: &str = "wall_3";
pub const MAP_WALL_4: &str = "wall_4";
pub const MAP_WALL_5: &str = "wall_5";
pub const MAP_WALL_6: &str = "wall_6";
pub const MAP_WALL_7: &str = "wall_7";
pub const MAP_WALL_8: &str = "wall_8";
pub const MAP_WALL_9: &str = "wall_9";
pub const MAP_WALL_10: &str = "wall_10";
pub const MAP_WALL_11: &str = "wall_11";
pub const MAP_WALL_12: &str = "wall_12";
pub const MAP_WALL_13: &str = "wall_13";
pub const MAP_WALL_14: &str = "wall_14";
pub const MAP_WALL_15: &str = "wall_15";

// sprite
//pub const SPRITE_PLAYER: &str = "temp_tiles/Gentera.png";
pub const SPRITE_GHOUL: &str = "ghoul";
pub const SPRITE_PLAYER_DWARF: &str = "human";
pub const SPRITE_PLAYER_HUMAN: &str = "human";
pub const SPRITE_PLAYER_ORC: &str = "human";
pub const SPRITE_PLAYER_ELF: &str = "human";
pub const SPRITE_PLAYER_TROLL: &str = "human";
pub const SPRITE_GHOUL_RANGED: &str = "ghoul_ranged";


//SAVE
pub const SCENE_FILE_PATH: &str = "assets/saves/save.srrl";
