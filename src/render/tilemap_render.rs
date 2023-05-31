use bevy::prelude::*;

use crate::{

    globals::{MAP_WALL, MAP_DEFAULT, MAP_EXIT, MAP_FLOOR, TILE_HEIGHT_HIGHT, TILE_HEIGHT_MEDIUM},
    map_builders::TileType, game::{spawners::spawn_sprite_render, GridPosition, Tile}, render::{get_world_position, components::{TileCollider, TileExit, GameMapRender}, get_world_z}
};


pub fn spawn_map_render(
    mut commands: Commands,
    all_tiles_query: Query<(Entity, &GridPosition, &Tile)>,
    asset_server: Res<AssetServer>,
) {
    println!("Rendering map begins...");
    //All tiles entities created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    for (_entity, grid_position, logic_tile) in all_tiles_query.iter() {
          let (world_x, mut world_y) = get_world_position(grid_position.x, grid_position.y);
          //TODO : Les coordonnées sont appliqués au Top Left d'un Sprite (0,0). La taille du Sprite donne donc son point Bottom, au sol
            // => Si pas la même taille, un gros perso sera "plus bas" dans le sol à cause de cela. Si le sol est à 32, un personnage 32 sera bien placé. Un perso 48 aura 16 unités dans le sol.
            // => Pour cette raison, il faut augmenter vers le haut la coordonnée d'un perso s'il est plus grand que TILE_HEIGHT. Perso 48 pour un TILE_HEIGHT=32 doit être affiché 16 unités plus haut.

        //texture & Z according to tile, before creation.   //TODO edition post creation maybe? Il nous faut l'info taille & texture par texture. 
        let mut texture = MAP_DEFAULT;
        let mut world_z = get_world_z(grid_position.x, grid_position.y);
        match logic_tile.tiletype {
            TileType::Wall => {
                texture = MAP_WALL;
                // On fait -(TAILLE_DE_LA_TILE -STANDARD_TILE) /2  //TODO : Mieux generifier ca, car les Persos doivent l'utiliser aussi.
                // REMEMBER : +Y dans Bevy = descendre. Ici on veut "monter" pour sortir les pieds du sol : On doit aller dans le negatif... :/
                world_y -= ((TILE_HEIGHT_HIGHT - TILE_HEIGHT_MEDIUM) / 2) as f32;
            }
            TileType::Exit => {
                texture = MAP_EXIT;
                world_z = 0.0;
            }
            TileType::Floor => {
                texture = MAP_FLOOR; 
                world_z = 0.0;
            }
            _ => {}
        }
        //println!("map render: world z for type {:?} is {}",texture, world_z);

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
