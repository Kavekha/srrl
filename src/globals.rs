use bevy::prelude::*;

// WINDOWS
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;

// RENDERING
pub const TILE_SIZE: f32 = 32.0;
pub const CHAR_SIZE: f32 = 16.0;  
// temp
pub const MAP_WALL: &str = "temp_tiles/Sewers_wall.png";
pub const MAP_FLOOR: &str = "temp_tiles/Sewers_floor.png";

// MAP GENERATOR
pub const SHOW_MAPGEN_VISUALIZER : bool = false;    //DEBUG 
pub const FIXED_MAPGEN_NEW_SNAPSHOT: f32 = 10.0;    // Doesn't look like 1 update / 10 secs, but let's go with it for now.
//default
pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT : usize = 50;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;

// MENUS
pub const MAIN_MENU_OPTIONS_COUNT: isize = 3;  //Necessaire pour la selection d'une option dans l'input.

// PATHFINDING
pub const FIXED_TIMESTEP: f32 = 0.1;
pub const BASE_RANGED_VIEW:i32 = 12;     // Distance à laquelle un NPC "voit" le joueur. //TODO : real visibility check
