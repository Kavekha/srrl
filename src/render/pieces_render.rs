use bevy::prelude::*;

use crate::{
    globals::{
        SPRITE_GHOUL, SPRITE_PLAYER, POSITION_TOLERANCE, SPEED_MULTIPLIER, BASE_SPEED, SPRITE_PLAYER_HUMAN, 
        SPRITE_PLAYER_ORC, SPRITE_PLAYER_TROLL, SPRITE_PLAYER_DWARF, SPRITE_PLAYER_ELF, },
    game::{player::{Player}, pieces::{components::Piece, spawners::Kind}, tileboard::components::BoardPosition}, GraphicsWaitEvent};

use super::{get_world_position, get_world_z, get_iso_y_modifier_from_elevation};


pub fn update_piece_position(
    mut query: Query<(&BoardPosition, &mut Transform, &Piece)>,  
    time: Res<Time>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>
){
    let mut animating = false;

    for (position, mut transform, piece) in query.iter_mut(){
        let (position_x, mut position_y) = get_world_position(&position);

        position_y += get_iso_y_modifier_from_elevation(piece.size); 

        let target = Vec3::new(position_x, position_y, get_world_z(&position));
        let destination = (target - transform.translation).length();
  
        
        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                BASE_SPEED * SPEED_MULTIPLIER * time.delta_seconds()
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
    query: Query<(Entity, &BoardPosition, &mut Piece, Option<&Player>)>,
    asset_server: Res<AssetServer>
) {
    println!("Rendering Pieces begins..."); 
    // On ajoute aux entités de nouveaux components.
    for (entity, position, piece, player) in query.iter() {

        let (x, mut y) = get_world_position(&position);
        let z = get_world_z(&position);

        let texture = get_texture_from_kind(piece.kind);
        y += get_iso_y_modifier_from_elevation(piece.size);

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

pub fn get_texture_from_kind(
    kind: Kind
) -> &'static str {
    match kind {
        Kind::Dwarf => { SPRITE_PLAYER_DWARF }
        Kind::Elf => { SPRITE_PLAYER_ELF }
        Kind::Human => { SPRITE_PLAYER_HUMAN }
        Kind::Orc => { SPRITE_PLAYER_ORC }
        Kind::Troll => { SPRITE_PLAYER_TROLL }
        Kind::Ghoul => { SPRITE_GHOUL }
        _ => { SPRITE_PLAYER }
    }
}

pub fn spawn_sprite_render(
    commands: &mut Commands,
    asset_server: &AssetServer,
    x: f32,
    y: f32,
    z: f32,
    img: &str,
) -> Entity {
    let sprite = commands.spawn(SpriteBundle {
        texture: asset_server.load(img), 
        transform: Transform {
            translation: Vec3::new(x, y, z),
            scale: Vec3::splat(1.0),  
            ..default()
        },
        ..default()
    }).id();

    sprite
}
