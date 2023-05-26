use bevy::prelude::*;

use crate::{globals::{SPRITE_GHOUL, PIECE_Z, SPRITE_PLAYER, PLAYER_Z}, game::{GridPosition, player::{Piece, Player}}};

use super::get_world_position;



pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &GridPosition, &Piece, Option<&Player>)>,
    asset_server: Res<AssetServer>
) {
    println!("Rendering Pieces begins..."); 
    // On ajoute aux entités de nouveaux components.
    for (entity, grid_position, _piece, player) in query.iter() {

        // Apparence
        let mut texture  = SPRITE_GHOUL;    //TODO : Plus de flexibilité pour changer les mobs.

        let (x, y) = get_world_position(grid_position.x, grid_position.y);
        let mut z = PIECE_Z;

        if let Some(_player) = player {
            texture = SPRITE_PLAYER;
            z = PLAYER_Z;
        }

        commands.entity(entity)
            .insert(SpriteBundle {
                texture: asset_server.load(texture),    
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            });
        
            if let Some(_player) = player {
                println!("INFO: player rendered.");
            }
        
    }
    println!("Pieces rendered.");
}