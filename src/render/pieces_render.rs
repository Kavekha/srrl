use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    globals::{
        SPRITE_GHOUL, SPRITE_PLAYER, POSITION_TOLERANCE, SPEED_MULTIPLIER, BASE_SPEED, SPRITE_PLAYER_HUMAN, 
        SPRITE_PLAYER_ORC, SPRITE_PLAYER_TROLL, SPRITE_PLAYER_DWARF, SPRITE_PLAYER_ELF, },
    game::{player::{Player}, pieces::{components::Piece, spawners::Kind}, tileboard::components::BoardPosition, actions::{ActionExecutedEvent, WalkAction, MeleeHitAction}}, GraphicsWaitEvent};

use super::{get_world_position, get_world_z, get_iso_y_modifier_from_elevation, components::PathAnimator};


pub fn melee_animation(
    mut commands: Commands,
    query_position: Query<&BoardPosition>,
    query_piece: Query<&Piece>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>
) {
    for ev in ev_action.iter() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeHitAction>() {
            println!("MELEE ATTACK ANIM !");
            let Ok(base_position) = query_position.get(action.attacker) else { continue };
            let Ok(base_piece) = query_piece.get(action.attacker) else { continue };

            let (position_x, mut position_y) = get_world_position(&base_position.v);
            position_y += get_iso_y_modifier_from_elevation(base_piece.size);
            let base = Vec3::new(position_x, position_y, get_world_z(&base_position.v)); 

            let (position_x, position_y) = get_world_position(&action.target);
            let target = Vec3::new(position_x, position_y, get_world_z(&action.target)); 

            println!("Melee attack anim start from {:?} and goes to {:?}", base, target);

            commands.entity(action.attacker)
                //.insert(PathAnimator{path:VecDeque::from([target, base]), wait_anim: true});
                .insert(PathAnimator{path:VecDeque::from([target]), wait_anim: true});  // Just go to the target for one-shot
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

pub fn walk_animation(
    mut commands: Commands,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>,
) {
    for ev in ev_action.iter() {
        let action = ev.0.as_any();
        // WalkAction: On recupere Entity + Position + Piece
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            // Converti pour ISO.
            let (position_x, mut position_y) = get_world_position(&action.1); // Position X,Y de World.

            position_y += get_iso_y_modifier_from_elevation(action.2.size); // Affichage de l'image pour les sprites > Tile.    
            let target = Vec3::new(position_x, position_y, get_world_z(&action.1)); // Calcul du Z par la même occasion.

            commands.entity(action.0)
                .insert(PathAnimator{path:VecDeque::from([target]), wait_anim: false});
            //ev_wait.send(GraphicsWaitEvent);
        }
    }
}


pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.path.len() == 0 {
            // this entity has completed it's animation
            println!("PathAnimator: Anim completed.");
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }
        let target = *animator.path.get(0).unwrap();  
        let destination = (target - transform.translation).length();

        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                BASE_SPEED * SPEED_MULTIPLIER * time.delta_seconds()
            );
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.path.pop_front();
        }
        if animator.wait_anim {
            ev_wait.send(GraphicsWaitEvent);
            println!("wait_anim: True");
        }
    }
}

/* 
pub fn update_piece_position(
    mut query: Query<(&BoardPosition, &mut Transform, &Piece)>,  
    time: Res<Time>,
    //mut ev_wait: EventWriter<GraphicsWaitEvent>
){
    let mut animating = false;

    for (position, mut transform, piece) in query.iter_mut(){
        let (position_x, mut position_y) = get_world_position(&position.v);

        position_y += get_iso_y_modifier_from_elevation(piece.size); 

        let target = Vec3::new(position_x, position_y, get_world_z(&position.v));
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
*/

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &BoardPosition, &mut Piece, Option<&Player>)>,
    asset_server: Res<AssetServer>
) {
    println!("Rendering Pieces begins..."); 
    // On ajoute aux entités de nouveaux components.
    for (entity, position, piece, player) in query.iter() {

        let (x, mut y) = get_world_position(&position.v);
        let z = get_world_z(&position.v);

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
