use bevy::prelude::*;

use crate::{

    globals::{MAP_WALL, MAP_DEFAULT, MAP_EXIT, MAP_FLOOR},
    map_builders::TileType, game::{spawners::spawn_sprite_render, GridPosition, Tile}, render::{get_world_position, components::{TileCollider, TileExit, GameMapRender}}
};


pub fn spawn_map_iso_render(
    mut commands: Commands,
    all_tiles_query: Query<(Entity, &GridPosition, &Tile)>,
    asset_server: Res<AssetServer>,
){

    const TILE_ISO_WIDTH:i32 = 64;
    const TILE_ISO_HEIGHT:i32 = 32;

    println!("Rendering iso map begins...");
    let mut tiles:Vec<Entity> = Vec::new();

    for (_entity, grid_position, logic_tile) in all_tiles_query.iter() {

        //Convert x,y to Iso:
        let iso_x = grid_position.x - grid_position.y;
        let iso_y = (grid_position.x + grid_position.y) / 2;

        let (world_x, world_y) = get_world_position(iso_x, iso_y);


        //texture & Z according to tile, before creation.   //TODO edition post creation maybe?
        let mut texture = MAP_DEFAULT;
        let mut world_z = 0.0;
        match logic_tile.tiletype {
            TileType::Wall => {texture = MAP_WALL}
            TileType::Exit => {
                texture = MAP_EXIT;
                world_z = 10.0;    //TODO : Decider ce que represente le Z.
            }
            TileType::Floor => {texture = MAP_FLOOR}
            _ => {}
        }

        //Create entity.
        let tile = spawn_sprite_render(
                &mut commands,
                &asset_server,
                world_x,
                world_y,
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

        //texture & Z according to tile, before creation.   //TODO edition post creation maybe?
        let mut texture = MAP_DEFAULT;
        let mut world_z = 0.0;
        match logic_tile.tiletype {
            TileType::Wall => {texture = MAP_WALL}
            TileType::Exit => {
                texture = MAP_EXIT;
                world_z = 10.0;    //TODO : Decider ce que represente le Z.
            }
            TileType::Floor => {texture = MAP_FLOOR}
            _ => {}
        }

        //Create entity.
        let tile = spawn_sprite_render(
                &mut commands,
                &asset_server,
                world_x,
                world_y,
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
