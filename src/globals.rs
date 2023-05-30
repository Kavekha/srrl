use bevy::prelude::*;

use crate::map_builders::pathfinding::Position;

// WINDOWS
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;

// RENDERING
pub const TILE_SIZE: f32 = 32.0;
pub const CHAR_SIZE: f32 = 16.0;  

pub const PLAYER_Z: f32 = 20.0;
pub const PIECE_Z: f32 = 15.0;
// temp
pub const MAP_WALL: &str = "temp_tiles/Sewers_wall.png";
pub const MAP_FLOOR: &str = "temp_tiles/Sewers_floor.png";
pub const MAP_EXIT: &str = "temp_tiles/exit.png";        //TODO : Ajouter.
pub const MAP_DEFAULT: &str = "temp_tiles/Sewers_wall.png";        //TODO : Trouver une image.
// sprite
pub const SPRITE_PLAYER: &str = "temp_tiles/Gentera.png";
pub const SPRITE_GHOUL: &str = "temp_tiles/Nosferatu.png";

// MAP GENERATOR
pub const SHOW_MAPGEN_VISUALIZER : bool = false;    //DEBUG     //BROKEN
#[warn(dead_code)]
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
pub const DEFAULT_COST_PATHFINDING:i32 = 1;     // Changing this will break the plan_walk.  //TODO : Remove this pathfinding element.

//SAVE
pub const SCENE_FILE_PATH: &str = "assets/scenes/save.srrl";

// Movement
pub const POSITION_TOLERANCE: f32 = 0.1;
pub const SPEED_MULTIPLIER: f32 = 2.0;      // FAST debug / balance on speed movement.

// Positions.
pub const ORTHO_DIRECTIONS: [Position; 4] = [
    Position(0,1), Position(0,-1),
    Position(-1,0), Position(1,0)
];
pub const MULTI_DIRECTION:[Position; 8] = [
    Position(0,-1), Position(0,1),
    Position(-1,0), Position(1,0),
    Position(1,1), Position(1,-1),
    Position(-1,1), Position(-1,-1)
];

// Player input
pub const DIR_KEY_MAPPING: [(KeyCode, Position); 4] = [
    (KeyCode::Up, Position(0,1)), (KeyCode::Down, Position(0,-1)),
    (KeyCode::Left, Position(-1,0)), (KeyCode::Right, Position(1,0)),
];
pub const MULTI_DIR_KEY_MAPPING: [(KeyCode, Position); 8] = [
    (KeyCode::Numpad8, Position(0,1)), (KeyCode::Numpad2, Position(0,-1)),
    (KeyCode::Numpad4, Position(-1,0)), (KeyCode::Numpad6, Position(1,0)),
    (KeyCode::Numpad7, Position(-1,1)), (KeyCode::Numpad9, Position(1,1)),
    (KeyCode::Numpad1, Position(-1,-1)), (KeyCode::Numpad3, Position(1,-1)),  
];

pub const MULTI_DIR_KEY_MAPPING_NO_NUM: [(KeyCode, Position); 8] = [
    (KeyCode::Z, Position(0,1)), (KeyCode::S, Position(0,-1)),
    (KeyCode::Q, Position(-1,0)), (KeyCode::D, Position(1,0)),
    (KeyCode::A, Position(-1,1)), (KeyCode::E, Position(1,1)),
    (KeyCode::W, Position(-1,-1)), (KeyCode::X, Position(1,-1)),  
];

// Npc planning
pub const NPC_MOVE_SCORE_BONUS: i32 = 5;      // Bonus si cette position autour est sur le chemin menant au Joueur. Favorise largement ce choix.
pub const NPC_MOVE_SCORE_DEFAULT: i32 = 50;     // Combien vaut le fait de se deplacer pour un NPC. Aide a faire un choix entre plusieurs actions à choisir à la fin. Permets de planifier plusieurs choses. 
