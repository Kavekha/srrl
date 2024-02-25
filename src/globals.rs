use bevy::prelude::*;

// WINDOWS
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;
//pub const BASE_SCREEN_SCALE: f32 = 1.0; //DEBUG: Scaling multiplier


// RENDERING
pub const CHAR_SIZE: f32 = 16.0; 
pub const STANDARD_TILE_SIZE:i32 = 64;
// RENDERING ORDER
pub const ORDER_FLOOR: f32 = 0.0;
pub const ORDER_WALL: f32 = 5.0;
pub const ORDER_NPC: f32 = 8.0;
pub const ORDER_EXIT: f32 = 9.0;
pub const ORDER_PLAYER: f32 = 10.0;
pub const ORDER_CURSOR: f32 = 11.0;

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
pub const CURSOR: &str = "temp_tiles/2D_TOPDOWN_TILES/base_cursor_64.png";

// MAP GENERATOR
pub const SHOW_MAPGEN_VISUALIZER : bool = false;    //DEBUG     //BROKEN
#[allow(dead_code)]
pub const FIXED_MAPGEN_NEW_SNAPSHOT: f32 = 10.0;    // Doesn't look like 1 update / 10 secs, but let's go with it for now.
pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT : usize = 50;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;

// MENUS
pub const MAIN_MENU_OPTIONS_COUNT: isize = 3;  //Necessaire pour la selection d'une option dans l'input.

//SAVE
pub const SCENE_FILE_PATH: &str = "assets/saves/save.srrl";

// Movement
pub const POSITION_TOLERANCE: f32 = 0.01;
pub const BASE_SPEED: f32 = 3.0;
pub const SPEED_MULTIPLIER: f32 = 5.0;      // FAST debug / balance on speed movement.
pub const CURSOR_SPEED: f32 = 20.0;

// Interface
pub const INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE: f32 = 16.0;