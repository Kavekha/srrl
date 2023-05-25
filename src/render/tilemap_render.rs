use bevy::prelude::*;

use crate::{
    ecs_elements::{
        components::{GridPosition, Tile, TileExit, TileCollider, GameMap}
    }, 
    globals::{TILE_SIZE, MAP_WALL, MAP_DEFAULT, MAP_EXIT, MAP_FLOOR},
    map_builders::TileType, game::spawners::spawn_sprite
};


pub fn spawn_map_render(
    mut commands: Commands,
    all_tiles_query: Query<(Entity, &GridPosition, &Tile)>,
    asset_server: Res<AssetServer>,
) {
    println!("Rendering map begins...");
    //All tiles entities created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    for (entity, grid_position, logic_tile) in all_tiles_query.iter() {
        // TODO : use grid_to_world function
        let world_x = grid_position.x as f32 * TILE_SIZE;
        let world_y = -(grid_position.y as f32 * TILE_SIZE);

        //texture & Z according to tile, before creation.   //TODO edition post creation maybe?
        let mut texture = MAP_DEFAULT;
        let mut world_z = 0.0;
        match logic_tile.tiletype {
            TileType::Wall => {texture = MAP_WALL}
            TileType::Exit => {
                texture = MAP_EXIT;
                world_z = 100.0;    //TODO : Decider ce que represente le Z.
            }
            TileType::Floor => {texture = MAP_FLOOR}
            _ => {texture = MAP_DEFAULT}
        }

        //Create entity.
        let tile = spawn_sprite(
                &mut commands,
                &asset_server,
                world_x,
                world_y,
                world_z,
                texture,
            );

        // Specific components. For some reason, match doesnt work here.
        if logic_tile.tiletype == TileType::Wall {
            commands.entity(tile).insert(TileCollider);
        }
        if logic_tile.tiletype == TileType::Exit {
            commands.entity(tile).insert(TileExit);
        }

        tiles.push(tile); 
    }
    commands
    .spawn(Name::new("Game Map"))
    .insert(GameMap)
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&tiles)
    ;
}
