use bevy::prelude::*;

use crate::{
    globals::{
        SPRITE_GHOUL, POSITION_TOLERANCE, SPEED_MULTIPLIER, BASE_SPEED, SPRITE_PLAYER_HUMAN, 
        SPRITE_PLAYER_ORC, SPRITE_PLAYER_TROLL, SPRITE_PLAYER_DWARF, SPRITE_PLAYER_ELF, ORDER_EXIT, ORDER_NPC, ORDER_PLAYER, MAP_EXIT},
    game::{
        player::Player, pieces::{components::Piece, spawners::Kind}, 
        tileboard::components::{BoardPosition, ExitMapTile},
    }, engine::asset_loaders::GraphicsAssets, engine::render::get_world_position
};

use super::components::PathAnimator;


pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    //mut ev_wait: EventWriter<GraphicsWaitEvent>
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        // DEBUG: println!("Anim: Entity is : {:?}", entity);
        if animator.path.len() == 0 {
            // this entity has completed it's animation
            // DEBUG: println!("PathAnimator: Anim completed.");
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }
        //DEBUG: println!("Anim update");
        let target = *animator.path.get(0).unwrap();  
        let destination = target - transform.translation;

        if destination.length() > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                BASE_SPEED * SPEED_MULTIPLIER * time.delta_seconds()
            );
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.path.pop_front();
        }
        /* 
        if animator.wait_anim {
            ev_wait.send(GraphicsWaitEvent);
            //println!("wait_anim: True");
        }
        */
    }
}


pub fn spawn_exit_render(
    mut commands: Commands,
    query: Query<(Entity, &BoardPosition), With<ExitMapTile>>,
    assets: Res<GraphicsAssets>
){
    println!("Rendering Exit begins...");
    for (entity, position) in query.iter() {
        let translation = get_world_position(&position.v);
        let texture = assets.map_items[MAP_EXIT].clone();

        commands.entity(entity)
            .insert(SpriteBundle {
                texture: texture,    
                transform: Transform {
                    translation: Vec3::new(translation.0, translation.1, ORDER_EXIT),  
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            });
        println!("An Exit has been rendered.");
    }
    println!("Rendering Exit ends.");
}


pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &BoardPosition, &mut Piece, Option<&Player>)>,
    assets: Res<GraphicsAssets>,
) {
    println!("Rendering Pieces begins..."); 
    // On ajoute aux entités de nouveaux components.
    for (entity, position, piece, player) in query.iter() {
        let translation= get_world_position(&position.v);   //TODO : get world position retourne un Vector2Int
        let texture = assets.textures[get_texture_from_kind(piece.kind)].clone();
        let mut order_z = ORDER_NPC;

        if let Some(_player) = player {
            println!("player order layer: {:?}.", entity);
            order_z = ORDER_PLAYER;
        }

        commands.entity(entity)
            .insert(SpriteBundle {
                texture: texture, //asset_server.load(texture),    
                transform: Transform {
                    translation: Vec3::new(translation.0, translation.1, order_z),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            });
        
            if let Some(_player) = player {
                println!("player rendered.");
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
        //_ => { SPRITE_PLAYER }
    }
}

pub fn spawn_sprite_render(
    commands: &mut Commands,
    texture: &Handle<Image>,
    x: f32,
    y: f32,
    z: f32,
) -> Entity {
    let sprite = commands.spawn(SpriteBundle {
        texture: texture.clone(), 
        transform: Transform {
            translation: Vec3::new(x, y, z),
            scale: Vec3::splat(1.0),  
            ..default()
        },
        ..default()
    }).id();

    sprite
}
