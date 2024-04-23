
use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    commons::get_world_position, engine::{animations::events::{AnimationIndices, AnimationTimer}, 
    asset_loaders::GraphicsAssets, render::components::GameCursorRender}, 
    game::{pieces::components::GameElement, player::Cursor, BASE_SPEED_PATH_ANIMATOR_UPDATE, BASE_TIME_FRAME_EFFECT, CURSOR_SPEED, POSITION_TOLERANCE, SPEED_MULTIPLIER}, globals::{ ORDER_CURSOR, ORDER_EFFECT}};

use super::events::{AnimateEvent, EffectEvent, GraphicsWaitEvent, PathAnimator};



// Ne fonctionne que pour un cas pour le moment. Rendre configurable via l'Event à l'origine du spawn effect
pub fn spawn_hit_effect(
    mut commands: Commands,
    mut ev_spawn_effect: EventReader<EffectEvent>,
    graph_assets: ResMut<GraphicsAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for event in ev_spawn_effect.read() {
        //println!("Creating effect");
        let texture = graph_assets.effects[event.id.as_str()].clone();
        let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 3, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 0, last: 2 };
        commands.spawn((
            SpriteBundle {
                //transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, ORDER_EFFECT),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                texture,
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(BASE_TIME_FRAME_EFFECT, TimerMode::Repeating)), // Repeating car on passe par autant d'etapes que d'images.
            GameElement
        ));
    }
}


// Declenché par AnimateEvent
pub fn walk_animation(    
    mut commands: Commands,
    mut ev_animate: EventReader<AnimateEvent>,
    transform_q: Query<&Transform>
) {
    let mut to_add = HashMap::new();
    for ev in ev_animate.read() {
        //println!("---- Anim with wait : {:?}", ev.wait_anim);
        let mut path = ev.path.clone();

        let Ok(transform) = transform_q.get(ev.entity) else { return };
        let mut path_animation: VecDeque<Vec3> = VecDeque::new();
        while !ev.path.is_empty() {
            let step = path.pop_front();
            let Some(current_step) = step else { break };
            let world_position = get_world_position(&current_step); 
            let target = Vec3::new(world_position.0, world_position.1, transform.translation.z);
            path_animation.push_back(target);
        }
        let path_animator = PathAnimator{path:VecDeque::from(path_animation), wait_anim: ev.wait_anim};
        //println!("PathAnimator created");
        to_add.insert(ev.entity, path_animator);
        //commands.entity(ev.entity).insert(PathAnimator{path:VecDeque::from(path_animation), wait_anim: true});        
    }
    for (entity, path_animator) in to_add {
        commands.entity(entity).insert(path_animator);
    }
}

pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>
) {
    let mut to_remove= Vec::new();
    for (entity, mut animator, mut transform) in query.iter_mut() {
        // DEBUG: println!("Anim: Entity is : {:?}", entity);
        if animator.path.len() == 0 {
            // this entity has completed it's animation
            // DEBUG: println!("PathAnimator: Anim completed.");
            to_remove.push(entity);
            continue;
        }
        //DEBUG: println!("Anim update");
        let target = *animator.path.get(0).unwrap();  
        let destination = target - transform.translation;

        if destination.length() > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                BASE_SPEED_PATH_ANIMATOR_UPDATE * SPEED_MULTIPLIER * time.delta_seconds()
            );
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.path.pop_front();
        }
        
        if animator.wait_anim {
            ev_wait.send(GraphicsWaitEvent);
            //println!("wait_anim: True");
        }        
    }
    for entity in to_remove {
        commands.entity(entity).remove::<PathAnimator>();
    }
}

pub fn update_game_cursor(
    mut query_game_cursor: Query<(&GameCursorRender, &mut Transform)>,
    cursor_position: Res<Cursor>,
    time: Res<Time>
){
    for (_game_cursor, mut transform, ) in query_game_cursor.iter_mut(){
        let grid_position = &cursor_position.grid_position;
        let position = get_world_position(grid_position);


        //let position = &cursor_position.world_position;

        let target = Vec3::new(position.0, position.1, ORDER_CURSOR);
        let destination = (target - transform.translation).length();  
        //println!("Cursor update: target is {:?}, transform is : {:?}, destination is : {:?}", target, transform.translation, destination);
        
        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                CURSOR_SPEED * SPEED_MULTIPLIER * time.delta_seconds()
            );
        } else {
            transform.translation = target;
        }
    }
}