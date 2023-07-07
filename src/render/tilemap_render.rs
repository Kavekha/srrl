use bevy::prelude::*;

use crate::{

    globals::{
        MAP_DEFAULT, MAP_EXIT, MAP_FLOOR, MAP_WALL_VERY_HIGH, TILE_HEIGHT_EXTREMELY_HIGH, TILE_WIDTH_HALF, TILE_HEIGHT_HALF},
    map_builders::{TileType, map::Map}, game::{Tile, tileboard::components::BoardPosition}, 
    render::{get_world_position, components::{TileCollider, TileExit, GameMapRender}, get_world_z, pieces_render::spawn_sprite_render}, vectors::Vector2Int
};

use super::get_iso_y_modifier_from_elevation;


/// Return a mask with wall corners at the Point(x,y).
/// Si x= 10,y=20 on regarde donc NorthEast: 11,19 - SouthEast: 11,21 - SouthWest: 9,21 - NorthWest: 9,19
/// 0 = Aucun.  // 1 = NorthEast    // 2 = SouthEast    // 4 = SouthWest    // 8 = NorthWest    ==> 15 = Total Mur
/// We check from the graphical tile perspective: We look at the crossroad of 4 logic tiles.
pub fn wall_corners(
    board: &Map,
    x: i32,
    y: i32
) -> &str {
    let mut mask: u8 = 0;

    let ne = (x, y);
    let se = (x, y + 1);
    let sw = (x - 1, y + 1);
    let nw = (x - 1, y);

    if board.out_of_bounds(ne.0, ne.1) { mask += 1;} else {
        if board.is_blocked(ne.0, ne.1) { mask += 1;}
    }
    if board.out_of_bounds(se.0, se.1) { mask += 2;} else {
        if board.is_blocked(se.0, se.1) { mask += 2;}
    }
    if board.out_of_bounds(sw.0, sw.1) { mask += 4;} else {
        if board.is_blocked(sw.0, sw.1) { mask += 4;}
    }
    if board.out_of_bounds(nw.0, nw.1) { mask += 8;} else {
        if board.is_blocked(nw.0, nw.1)  { mask += 8;}
    }
 
    match mask {
        0 => { "temp_tiles/Sewers_wall96_0.png" },      //No wall at all.
        1 => { "temp_tiles/Sewers_wall96_1.png" },
        2 => { "temp_tiles/Sewers_wall96_2.png" },
        3 => { "temp_tiles/Sewers_wall96_3.png" },
        4 => { "temp_tiles/Sewers_wall96_4.png" },
        5 => { "temp_tiles/Sewers_wall96_5.png" },
        6 => { "temp_tiles/Sewers_wall96_6.png" },
        7 => { "temp_tiles/Sewers_wall96_7.png" },
        8 => { "temp_tiles/Sewers_wall96_8.png" },
        9 => { "temp_tiles/Sewers_wall96_9.png" },
        10 => { "temp_tiles/Sewers_wall96_10.png" },
        11 => { "temp_tiles/Sewers_wall96_11.png" },
        12 => { "temp_tiles/Sewers_wall96_12.png" },
        13 => { "temp_tiles/Sewers_wall96_13.png" },
        14 => { "temp_tiles/Sewers_wall96_14.png" },
        15 =>  { "temp_tiles/Sewers_wall96_15.png" },
        _ => { MAP_WALL_VERY_HIGH }
    }
    //return mask;

}


pub fn spawn_map_render_new(
    mut commands: Commands,
    all_tiles_query: Query<(Entity, &BoardPosition, &Tile)>,
    asset_server: Res<AssetServer>,
    board: Res<Map>,
) {
    println!("New rendering map begins...");

    let mut graphic_tiles:Vec<Entity> = Vec::new();

    for y in 0..board.height {
        for x in 0..board.width {
            let position = BoardPosition {v : Vector2Int { x, y } };    //TODO : Moche.
            let (mut world_x, mut world_y) = get_world_position(&position.v);

            // On est sur la Dual Grid: Il faut un offset de 1/4 car le 0,0 logic est a cheval entre 0,0 - 0,1 - 1,0 - 1,1.
            world_x -= TILE_WIDTH_HALF as f32  / 2.0 ;
            world_y += TILE_HEIGHT_HALF as f32  / 2.0 ;    // REMEMBER : En World, +Y permets de "monter" dans la map.

            let texture = wall_corners(&board, x, y);
            let (modified_y, world_z) = get_y_z_rendering(x, y);
            

            let tile = spawn_sprite_render(
                &mut commands,
                &asset_server,
                world_x,
                world_y + modified_y,
                world_z,
                texture,
            );

            // Specific components. For some reason, match doesnt work here.
            // TODO : N'a rien à faire ici : Elements logiques!
            let board_tile = board.tiles[board.xy_idx(x, y)];
            if board_tile == TileType::Wall {
                commands.entity(tile).insert(TileCollider);
            }
            if board_tile == TileType::Exit {
                commands.entity(tile).insert(TileExit);
            }
    
            graphic_tiles.push(tile); 
        }
    }
    
    println!("Tiles rendered.");
    
    commands
    .spawn(Name::new("Game Map Render"))
    .insert(GameMapRender)
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&graphic_tiles);
}



pub fn spawn_map_render(
    mut commands: Commands,
    all_tiles_query: Query<(Entity, &BoardPosition, &Tile)>,
    asset_server: Res<AssetServer>,
) {
    println!("Rendering map begins...");
    //All tiles entities created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    for (_entity, position, logic_tile) in all_tiles_query.iter() {

          let (world_x, world_y) = get_world_position(&position.v);

          
        //get info for typetile
        let (texture, modified_y, world_z) =  get_tile_infos_render(&position, logic_tile.tiletype);

        //Create entity.
        let tile = spawn_sprite_render(
                &mut commands,
                &asset_server,
                world_x,
                world_y + modified_y,
                world_z,
                texture,
            );

        // Specific components. For some reason, match doesnt work here.
        // TODO : N'a rien à faire ici : Elements logiques!
        if logic_tile.tiletype == TileType::Wall {
            commands.entity(tile).insert(TileCollider);
        }
        if logic_tile.tiletype == TileType::Exit {
            commands.entity(tile).insert(TileExit);
        }

        tiles.push(tile); 
    }

    println!("Tiles rendered.");

    commands
    .spawn(Name::new("Game Map Render"))
    .insert(GameMapRender)
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&tiles)
    ;
}


/// Return y modifier for elevation and z rendering order.
fn get_y_z_rendering(
    x: i32, 
    y: i32
) -> (f32, f32) {
    let y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_EXTREMELY_HIGH);
    //let y_modifier = 0.0;   //TODO : Refacto de tout ce bordel.
    let position = BoardPosition {v : Vector2Int { x, y } };    //TODO : Sad! Change this
    let world_z = get_world_z(&position.v);
    (y_modifier, world_z)
}


/// Return texture, y modifier for elevation and z rendering order for TileType.
fn get_tile_infos_render(
    position: &BoardPosition,
    tile_type: TileType
) -> (&'static str, f32, f32) {

    let mut texture = MAP_DEFAULT;
    let mut y_modifier = 0.0;
    let mut world_z = 0.0;

    match tile_type {
        TileType::Wall => {
            texture = MAP_WALL_VERY_HIGH;
            y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_EXTREMELY_HIGH);
            /* 
            let mut rng = rand::thread_rng();
            let rand = rng.gen_range(0..4);
            match rand {
                0 => { 
                    texture = MAP_WALL_HIGH;
                    y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_VERY_HIGH);
                }
                1 => { 
                    texture = MAP_WALL_LOW;
                    y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_MEDIUM_HIGH);
                }
                2 => { 
                    texture = MAP_WALL_VERY_HIGH;
                    y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_EXTREMELY_HIGH);
                }
                3 => {
                    texture = MAP_WALL;
                    y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_HIGH);
                }
                _ => {
                    y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_HIGH);
                }
            } 
            */           
            world_z = get_world_z(&position.v);
        }
        TileType::Exit => {texture = MAP_EXIT}
        TileType::Floor => {texture = MAP_FLOOR}
    }

    (texture, y_modifier, world_z)  //Return
}