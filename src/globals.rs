use bevy::prelude::*;

// Versions
pub const VERSION: &str = "0.17.0";
pub const RELEASE: &str = "R0.4";

// Min - Max VOLUME
pub const MIN_VOLUME: f32 = 0.0;
pub const MAX_VOLUME: f32 = 5.0;
// WINDOWS
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;
//pub const BASE_SCREEN_SCALE: f32 = 1.0; //DEBUG: Scaling multiplier

// Menu colors
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);          // TODO : Même couleur que le fond si on veut le cacher. Defaut background button est blanc.
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// RENDERING
pub const CHAR_SIZE: f32 = 16.0; 
pub const STANDARD_TILE_SIZE:i32 = 32;  //64;
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
pub const CURSOR: &str = "cursors/base_cursor_32.png";      //TODO to put in Assets

// MAP GENERATOR
pub const SHOW_MAPGEN_VISUALIZER : bool = false;    //DEBUG     //BROKEN
#[allow(dead_code)]
pub const FIXED_MAPGEN_NEW_SNAPSHOT: f32 = 10.0;    // Doesn't look like 1 update / 10 secs, but let's go with it for now.
pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT : usize = 50;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;


//SAVE
pub const SCENE_FILE_PATH: &str = "assets/saves/save.srrl";

// Movement
pub const POSITION_TOLERANCE: f32 = 0.01;
pub const BASE_SPEED: f32 = 3.0;
pub const SPEED_MULTIPLIER: f32 = 5.0;      // FAST debug / balance on speed movement.
pub const CURSOR_SPEED: f32 = 20.0;

// Interface
pub const INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE: f32 = 16.0;