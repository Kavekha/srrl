use bevy::prelude::*;
use rand::Rng;

use crate::{

    globals::{MAP_WALL, MAP_DEFAULT, MAP_EXIT, MAP_FLOOR, MAP_WALL_HIGH, MAP_WALL_LOW, TILE_HEIGHT_MEDIUM_HIGH, TILE_HEIGHT_HIGH, TILE_HEIGHT_VERY_HIGH, MAP_WALL_VERY_HIGH, TILE_HEIGHT_EXTREMELY_HIGH},
    map_builders::TileType, game::{spawners::spawn_sprite_render, GridPosition, Tile}, 
    render::{get_world_position, components::{TileCollider, TileExit, GameMapRender}, get_world_z}
};

use super::get_iso_y_modifier_from_elevation;


pub fn spawn_map_render(
    mut commands: Commands,
    all_tiles_query: Query<(Entity, &GridPosition, &Tile)>,
    asset_server: Res<AssetServer>,
) {
    println!("Rendering map begins...");
    //All tiles entities created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    for (_entity, grid_position, logic_tile) in all_tiles_query.iter() {
          let (world_x, world_y) = get_world_position(grid_position.x, grid_position.y);
          //TODO : Les coordonnées sont appliqués au Top Left d'un Sprite (0,0). La taille du Sprite donne donc son point Bottom, au sol
            // => Si pas la même taille, un gros perso sera "plus bas" dans le sol à cause de cela. Si le sol est à 32, un personnage 32 sera bien placé. Un perso 48 aura 16 unités dans le sol.
            // => Pour cette raison, il faut augmenter vers le haut la coordonnée d'un perso s'il est plus grand que TILE_HEIGHT. Perso 48 pour un TILE_HEIGHT=32 doit être affiché 16 unités plus haut.

        //get info for typetile
        let (texture, modified_y, world_z) =  get_tile_infos_render(grid_position.x, grid_position.y, logic_tile.tiletype);

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
    x: i32,
    y: i32,
    tile_type: TileType
) -> (&'static str, f32, f32) {

    let mut texture = MAP_DEFAULT;
    let mut y_modifier = 0.0;
    let mut world_z = 0.0;

    match tile_type {
        TileType::Wall => {
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
                _ => {
                    texture = MAP_WALL;
                    y_modifier = get_iso_y_modifier_from_elevation(TILE_HEIGHT_HIGH);
                }
            }            
            world_z = get_world_z(x, y);
        }
        TileType::Exit => {texture = MAP_EXIT}
        TileType::Floor => {texture = MAP_FLOOR}
    }

    (texture, y_modifier, world_z)  //Return
}