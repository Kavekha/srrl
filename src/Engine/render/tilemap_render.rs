use bevy::{prelude::*, utils::HashMap};

use crate::{
    commons::get_world_position,
    engine::{
        asset_loaders::GraphicsAssets, 
        render::{components::GameMapRender, pieces_render::spawn_sprite_render}
    }, 
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
/// TODO : Comment se passer du board?
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



// 0.20c : v2.
pub fn spawn_map_render(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    assets: Res<GraphicsAssets>,
    board: Res<Map>,
    //tile_position_q: Query<(Entity, &BoardPosition), With<Tile>>,
) {
    println!("New rendering map begins...");

    let mut dualgrid_floor = HashMap::new();
    let mut dualgrid_wall = HashMap::new();

    let mut floor_tiles:Vec<Entity> = Vec::new();
    let mut wall_tiles:Vec<Entity> = Vec::new();

    for (position, _entity) in board.entity_tiles.iter() {
        // Comme on va avoir un décallage de grille, on doit gérer les angles pour ne pas generer plusieurs fois les tuiles graphiques.
        // On ne peut pas le faire à la fin car elle ne nous est pas connue, il faut la faire au debut.
        // Si x,y == 0,0  on genere le coté Nord Ouest de la tile 0,0. 
        // Si x == 0, y!=0 on génère le coté Nord Est. 
        // Si x != 0, y==0 on génère le coté Sud Ouest.
        // Dans tous les autres cas, on génère systématiquement le coté Sud Est c'est à dire que x,y sera logic_x +1,logic_y +1.
        // On récupère la position dans le monde pour cette position. 
        let mut tiles_to_create = Vec::new();
        if position.x == 0 && position.y == 0 {
            tiles_to_create.push(Vector2Int { x: 0, y: 0 });
            tiles_to_create.push(Vector2Int { x: 1, y: 0 });
            tiles_to_create.push(Vector2Int { x: 0, y: 1 });
        }
        if position.x == 0 && position.y != 0 {
            tiles_to_create.push(Vector2Int { x: 0, y: position.y });
        } 
        if position.y == 0 && position.x != 0 {
            tiles_to_create.push(Vector2Int { x: position.x, y: 0 });
        } 
        // dans tous les cas: on genere la tuile graphique Sud Est.
        tiles_to_create.push(Vector2Int { x: position.x + 1, y: position.y + 1 });
    
        for new_position in tiles_to_create {
            let (mut world_x, mut world_y) = get_world_position(&new_position);
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
            dualgrid_floor.insert(new_position.clone(), floor_tile);    // On insert la tuile Nord Ouest de la tuile logique x,y.
            floor_tiles.push(floor_tile);
            
            // Si il y a un mur 
            if let Some(texture) = wall_corners(&board, new_position.x, new_position.y) {
                // Wall
                let wall_tile = spawn_sprite_render(
                    &mut commands,
                    &assets.map_textures[texture], //&asset_server,  //&assets.textures["card"]
                    world_x,
                    world_y,
                    ORDER_WALL,
                    //texture,
                );    
                dualgrid_wall.insert(new_position.clone(), wall_tile); 
                wall_tiles.push(wall_tile);
            }
        }
    }
    commands
    .spawn(Name::new("Game Map Render"))
    .insert(GameMapRender { 
        floor: dualgrid_floor,
        wall: dualgrid_wall
    })
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&wall_tiles)
    .push_children(&floor_tiles)
    ;
}


// 0.20b : v2.
pub fn spawn_map_render_old(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    assets: Res<GraphicsAssets>,
    board: Res<Map>,
    //tile_position_q: Query<(Entity, &BoardPosition), With<Tile>>,
) {
    println!("New rendering map begins...");

    let mut dualgrid_floor = HashMap::new();
    let mut dualgrid_wall = HashMap::new();

    let mut floor_tiles:Vec<Entity> = Vec::new();
    let mut wall_tiles:Vec<Entity> = Vec::new();

    for (position, _entity) in board.entity_tiles.iter() {
        // Comme on va avoir un décallage de grille, on doit gérer les angles pour ne pas generer plusieurs fois les tuiles graphiques.
        // On ne peut pas le faire à la fin car elle ne nous est pas connue, il faut la faire au debut.
        // Si x,y == 0,0  on genere le coté Nord Ouest de la tile 0,0. 
        // Si x == 0, y!=0 on génère le coté Nord Est. 
        // Si x != 0, y==0 on génère le coté Sud Ouest.
        // Dans tous les autres cas, on génère systématiquement le coté Sud Est c'est à dire que x,y sera logic_x +1,logic_y +1.
        // On récupère la position dans le monde pour cette position. 
        let new_position: Vector2Int;
        if position.x == 0 && position.y == 0 {
            new_position = Vector2Int { x: position.x, y: position.y };
        } else if position.x == 0 && position.y != 0 {
            new_position = Vector2Int { x: position.x, y: position.y + 1};
        } else if position.x != 0 && position.y == 0 {
            new_position = Vector2Int { x: position.x + 1, y: position.y };
        } else {
            // Dans la majorité des cas, on genere la tuile graphique Sud Est.
            new_position = Vector2Int { x: position.x + 1, y: position.y + 1};
        }        

        let (mut world_x, mut world_y) = get_world_position(&new_position);
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
        dualgrid_floor.insert(new_position.clone(), floor_tile);    // On insert la tuile Nord Ouest de la tuile logique x,y.
        floor_tiles.push(floor_tile);
        
        // Si il y a un mur 
        if let Some(texture) = wall_corners(&board, new_position.x, new_position.y) {
            // Wall
            let wall_tile = spawn_sprite_render(
                &mut commands,
                &assets.map_textures[texture], //&asset_server,  //&assets.textures["card"]
                world_x,
                world_y,
                ORDER_WALL,
                //texture,
            );    
            dualgrid_wall.insert(new_position.clone(), wall_tile); 
            wall_tiles.push(wall_tile);
        }
    }
    commands
    .spawn(Name::new("Game Map Render"))
    .insert(GameMapRender { 
        floor: dualgrid_floor,
        wall: dualgrid_wall
    })
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&wall_tiles)
    .push_children(&floor_tiles)
    ;
}
