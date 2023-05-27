use bevy::prelude::*;

use crate::{globals::{SPRITE_GHOUL, PIECE_Z, SPRITE_PLAYER, PLAYER_Z, POSITION_TOLERANCE}, game::{GridPosition, player::{Piece, Player, Stats}}};

use super::get_world_position;


pub fn update_piece_position(
    mut query: Query<(&GridPosition, &mut Transform, &Stats), With<Piece>>,
    time: Res<Time>,
){
    for (grid_position, mut transform, stats) in query.iter_mut(){
        let (position_x, position_y) = get_world_position(grid_position.x, grid_position.y);
        let target = Vec3::new(position_x, position_y, PIECE_Z);
        let destination = (target - transform.translation).length();
        
        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                stats.speed * time.delta_seconds()
            );
        } else {
            transform.translation = target;
        }

    }
}


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