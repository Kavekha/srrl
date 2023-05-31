use bevy::prelude::*;

use crate::{globals::{SPRITE_GHOUL, SPRITE_PLAYER, POSITION_TOLERANCE, SPEED_MULTIPLIER, TILE_HEIGHT_HIGHT, TILE_HEIGHT_MEDIUM, TILE_HEIGHT_MEDIUM_HIGHT, TILE_HEIGHT_HALF}, game::{GridPosition, player::{Piece, Player, Stats}}, GraphicsWaitEvent};

use super::{get_world_position, get_world_z};


pub fn update_piece_position(
    mut query: Query<(&GridPosition, &mut Transform, &Stats), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>
){
    let mut animating = false;

    for (grid_position, mut transform, stats) in query.iter_mut(){
        let (position_x, mut position_y) = get_world_position(grid_position.x, grid_position.y);

        // On fait -(TAILLE_DE_LA_TILE -STANDARD_TILE) /2  //TODO : Mieux generifier ca, car les Persos doivent l'utiliser aussi.
        // REMEMBER : +Y dans Bevy = descendre. Ici on veut "monter" pour sortir les pieds du sol : On doit aller dans le negatif... :/
        position_y -= ((TILE_HEIGHT_HIGHT - TILE_HEIGHT_MEDIUM) / 2) as f32;

        let target = Vec3::new(position_x, position_y, get_world_z(grid_position.x, grid_position.y));
        let destination = (target - transform.translation).length();
  
        
        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                stats.speed * SPEED_MULTIPLIER * time.delta_seconds()
            );
            animating = true;
        }
        if animating {
            //TODO: Currently: One wait by Actor, so a lot of wait.
            //ev_wait.send(GraphicsWaitEvent);
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

        let (x, mut y) = get_world_position(grid_position.x, grid_position.y);
        let mut z = get_world_z(grid_position.x, grid_position.y);

        // On fait -(TAILLE_DE_LA_TILE -STANDARD_TILE) /2  //TODO : Mieux generifier ca, car les Persos doivent l'utiliser aussi.
        // REMEMBER : +Y dans Bevy = descendre. Ici on veut "monter" pour sortir les pieds du sol : On doit aller dans le negatif... :/
        y -= ((TILE_HEIGHT_HIGHT - TILE_HEIGHT_MEDIUM) / 2) as f32;

        if let Some(_player) = player {
            texture = SPRITE_PLAYER;
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