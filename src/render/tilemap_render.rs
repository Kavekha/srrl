use bevy::prelude::*;
use rand::Rng;

use crate::{

    globals::{
        MAP_DEFAULT, MAP_EXIT, MAP_FLOOR, MAP_WALL_HIGH, MAP_WALL_LOW, TILE_HEIGHT_MEDIUM_HIGH, TILE_HEIGHT_HIGH,
        TILE_HEIGHT_VERY_HIGH, MAP_WALL_VERY_HIGH, TILE_HEIGHT_EXTREMELY_HIGH, MAP_WALL},
    map_builders::TileType, game::{Tile, tileboard::components::BoardPosition}, 
    render::{get_world_position, components::{TileCollider, TileExit, GameMapRender}, get_world_z, pieces_render::spawn_sprite_render}
};

use super::get_iso_y_modifier_from_elevation;


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