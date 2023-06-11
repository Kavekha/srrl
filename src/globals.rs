use bevy::prelude::*;

// WINDOWS
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;
pub const BASE_SCREEN_SCALE: f32 = 1.0; //DEBUG: Scaling multiplier

// RENDERING
pub const TILE_SIZE: f32 = 32.0;        // Deprecated : 2d grid based.
pub const CHAR_SIZE: f32 = 16.0; 
//pub const STANDARD_TILE_WIDTH:i32 = 64;
pub const TILE_WIDTH_HALF:i32 = 32;
//pub const STANDARD_TILE_HEIGHT:i32 = 32;
pub const TILE_HEIGHT_HALF:i32 = 16; 

// Tiles    //TEMPORARY : TODO : Loader.
pub const MAP_FLOOR: &str = "temp_tiles/Sewers_floor.png";
pub const MAP_EXIT: &str = "temp_tiles/exit.png";        //TODO : Ajouter.
pub const MAP_DEFAULT: &str = "temp_tiles/Sewers_wall.png";        //TODO : Trouver une image.
pub const MAP_WALL: &str = "temp_tiles/Sewers_wall64.png";
pub const MAP_WALL_LOW: &str = "temp_tiles/Sewers_wall48.png";
pub const MAP_WALL_HIGH: &str = "temp_tiles/Sewers_wall72.png";
pub const MAP_WALL_VERY_HIGH: &str = "temp_tiles/Sewers_wall96.png";

// size : 1 pix = 4 cm. 
pub const TILE_HEIGHT_EXTREMELY_HIGH: i32 = 96; // 1/3 more than the player.
pub const TILE_HEIGHT_VERY_HIGH: i32 = 72; // 1/3 more than the player.
pub const TILE_HEIGHT_HIGH: i32 = 64; // Same as Medium & floor : 2 Y Cell higher exactly, provoke confusion.
pub const TILE_HEIGHT_MEDIUM_HIGH: i32 = 48; // Same as player.
pub const TILE_HEIGHT_MEDIUM: i32 = 32; // Same as a floor tile but 1 Y cell higher exactly, so no real relief. May lead to confusion if different "floors" of tiles can be used.
//pub const TILE_HEIGHT_FLOOR: i32 = 0;
//pub const TILE_HEIGHT_LOW: i32 = 24;

//size: 1 pix = 4 cm. Character sprite are on tile center, so there is a TILE_HEIGHT_HALF to adds.
pub const SIZE_DWARF: i32 = 44; // 16 + 28 px (112 cm)
pub const SIZE_HUMAN: i32 = 64; // 16 + 48 px (192 cm)
pub const SIZE_ELF: i32 = 64; // 16 + 48 px (192 cm)
pub const SIZE_ORC: i32 = 64; // 16 + 48 px (192 cm)
pub const SIZE_TROLL: i32 = 80; // 16 + 64 pix (256 cm)
pub const SIZE_GHOUL: i32 = 64;
//pub const SIZE_DEFAULT: i32 = 64;

// sprite
pub const SPRITE_PLAYER: &str = "temp_tiles/Gentera.png";
pub const SPRITE_GHOUL: &str = "temp_tiles/Nosferatu.png";
pub const SPRITE_PLAYER_DWARF: &str = "temp_tiles/cube_dwarf_.png";
pub const SPRITE_PLAYER_HUMAN: &str = "temp_tiles/cube_human_.png";
pub const SPRITE_PLAYER_ORC: &str = "temp_tiles/cube_orc_.png";
pub const SPRITE_PLAYER_ELF: &str = "temp_tiles/cube_elf_.png";
pub const SPRITE_PLAYER_TROLL: &str = "temp_tiles/cube_troll_.png";

pub const CURSOR_FRONT: &str = "temp_tiles/cursor_front.png";
pub const CURSOR_BACK: &str = "temp_tiles/cursor_back.png";

// MAP GENERATOR
pub const SHOW_MAPGEN_VISUALIZER : bool = false;    //DEBUG     //BROKEN
#[warn(dead_code)]
//pub const FIXED_MAPGEN_NEW_SNAPSHOT: f32 = 10.0;    // Doesn't look like 1 update / 10 secs, but let's go with it for now.
//default
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
pub const SPEED_MULTIPLIER: f32 = 2.0;      // FAST debug / balance on speed movement.
pub const CURSOR_SPEED: f32 = 20.0;


// Npc planning
pub const NPC_MOVE_SCORE_BONUS: i32 = 5;      // Bonus si cette position autour est sur le chemin menant au Joueur. Favorise largement ce choix.
pub const NPC_MOVE_SCORE_DEFAULT: i32 = 50;     // Combien vaut le fait de se deplacer pour un NPC. Aide a faire un choix entre plusieurs actions à choisir à la fin. Permets de planifier plusieurs choses. 
pub const NPC_ATTACK_SCORE_DEFAULT: i32 = 750;