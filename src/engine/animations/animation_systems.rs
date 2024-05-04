
use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    commons::get_world_position, engine::render::components::GameCursorRender, game::{player::Cursor, BASE_SPEED_PATH_ANIMATOR_UPDATE, CURSOR_SPEED, POSITION_TOLERANCE, SPEED_MULTIPLIER}, globals:: ORDER_CURSOR};

use super::events::{AnimateEvent, GraphicsWaitEvent, PathAnimator};



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