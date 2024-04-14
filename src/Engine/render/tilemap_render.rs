use bevy::prelude::*;

use crate::{
    commons::get_world_position,
    engine::{
        asset_loaders::GraphicsAssets, 
        render::{components::{GameMapRender, TileRender, TileRendered}, pieces_render::spawn_sprite_render}
    }, 
    game::tileboard::components::{BoardPosition, Tile}, 
    globals::{
        MAP_WALL_1, MAP_WALL_10, MAP_WALL_11, MAP_WALL_12, MAP_WALL_13, MAP_WALL_14, MAP_WALL_15, MAP_WALL_2, MAP_WALL_3, MAP_WALL_4, MAP_WALL_5, MAP_WALL_6, MAP_WALL_7, MAP_WALL_8, MAP_WALL_9, ORDER_FLOOR, ORDER_WALL, STANDARD_TILE_SIZE
    }, 
    map_builders::map::Map, 
    vectors::Vector2Int
};


/// Return a mask with wall corners at the Point(x,y).
/// Si x= 10,y=20 on regarde donc NorthEast: 11,19 - SouthEast: 11,21 - SouthWest: 9,21 - NorthWest: 9,19
/// 0 = Aucun.  // 1 = NorthEast    // 2 = SouthEast    // 4 = SouthWest    // 8 = NorthWest    ==> 15 = Total Mur
/// We check from the graphical tile perspective: We look at the crossroad of 4 logic tiles.
pub fn wall_corners(
    board: &Map,
    x: i32,
    y: i32
) -> Option<&str> {
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
        0 => None,  //{ "temp_tiles/Sewers_wall96_0.png" },      //No wall at all.
        1 => { Some(MAP_WALL_1) },
        2 => { Some(MAP_WALL_2)  },
        3 => { Some(MAP_WALL_3)  },
        4 => { Some(MAP_WALL_4)  },
        5 => { Some(MAP_WALL_5)  },
        6 => { Some(MAP_WALL_6)  },
        7 => { Some(MAP_WALL_7)  },
        8 => { Some(MAP_WALL_8)  },
        9 => { Some(MAP_WALL_9)  },
        10 => { Some(MAP_WALL_10)  },
        11 => { Some(MAP_WALL_11)  },
        12 => { Some(MAP_WALL_12)  },
        13 => { Some(MAP_WALL_13)  },
        14 => { Some(MAP_WALL_14)  },
        15 =>  { Some(MAP_WALL_15)  },
        _ => { Some(MAP_WALL_15) }
    }
    //return mask;

}


// 0.20a : v2.
pub fn spawn_map_render(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    assets: Res<GraphicsAssets>,
    board: Res<Map>,
    tile_position_q: Query<(Entity, &BoardPosition), With<Tile>>,
) {
    println!("New rendering map begins...");

    let mut graphic_tiles:Vec<Entity> = Vec::new();
    let mut floor_tiles:Vec<Entity> = Vec::new();

    for (tile_entity, tile_position) in tile_position_q.iter() {
        let (mut world_x, mut world_y) = get_world_position(&tile_position.v);
        // On est sur la Dual Grid: Il faut un offset car le 0,0 logic est a cheval entre 0,0 - 0,1 - 1,0 - 1,1.
        world_x -= (STANDARD_TILE_SIZE / 2) as f32;         
        world_y -= (STANDARD_TILE_SIZE / 2) as f32; 

        // On créé le sol 
        let floor_tile = spawn_sprite_render(
            &mut commands,
            &assets.map_textures["floor"],
            world_x,
            world_y,
            ORDER_FLOOR,
            //MAP_FLOOR,
        );
        commands.entity(floor_tile).insert(TileRender { logic_entity: tile_entity });
        commands.entity(tile_entity).insert(TileRendered { render_entity: floor_tile });

        floor_tiles.push(floor_tile);   

        // TODO : l'info est dans Tile mais on a besoin des Tile autours pour gerer les Masks... On souffre tjrs du Double systeme.
        if let Some(texture) = wall_corners(&board, tile_position.v.x, tile_position.v.y) {
            // Wall
            let wall_tile = spawn_sprite_render(
                &mut commands,
                &assets.map_textures[texture], //&asset_server,  //&assets.textures["card"]
                world_x,
                world_y,
                ORDER_WALL,
                //texture,
            );
            commands.entity(floor_tile).push_children(&[wall_tile]);      
            graphic_tiles.push(wall_tile); 
        }
    
    }    
    println!("Tiles rendered.");
    
    commands
    .spawn(Name::new("Game Map Render"))
    .insert(GameMapRender)
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&graphic_tiles)
    .push_children(&floor_tiles);
}


pub fn spawn_map_render_old(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    assets: Res<GraphicsAssets>,
    board: Res<Map>,
) {
    println!("New rendering map begins...");

    let mut graphic_tiles:Vec<Entity> = Vec::new();
    let mut floor_tiles:Vec<Entity> = Vec::new();

    for y in 0..board.height -1 {
        for x in 0..board.width -1 {
            
            let position = BoardPosition {v : Vector2Int { x, y } };    //TODO : Moche.
            let (mut world_x, mut world_y) = get_world_position(&position.v);

            // On est sur la Dual Grid: Il faut un offset car le 0,0 logic est a cheval entre 0,0 - 0,1 - 1,0 - 1,1.
            world_x -= (STANDARD_TILE_SIZE / 2) as f32;         
            world_y -= (STANDARD_TILE_SIZE / 2) as f32;  

            if let Some(texture) = wall_corners(&board, x, y) {
                // Wall
                let wall_tile = spawn_sprite_render(
                    &mut commands,
                    &assets.map_textures[texture], //&asset_server,  //&assets.textures["card"]
                    world_x,
                    world_y,
                    ORDER_WALL,
                    //texture,
                );
                graphic_tiles.push(wall_tile); 
            }
            // On créé le sol 
            let floor_tile = spawn_sprite_render(
                &mut commands,
                &assets.map_textures["floor"],
                world_x,
                world_y,
                ORDER_FLOOR,
                //MAP_FLOOR,
            );
            
            floor_tiles.push(floor_tile);
        }
    }
    
    println!("Tiles rendered.");
    
    commands
    .spawn(Name::new("Game Map Render"))
    .insert(GameMapRender)
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&graphic_tiles)
    .push_children(&floor_tiles);
}

