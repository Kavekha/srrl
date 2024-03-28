use std::collections::VecDeque;

use bevy::prelude::*;

use crate::engine::render::pieces_render::path_animator_update;
use crate::engine::render::components::PathAnimator;
use crate::engine::render::get_world_position;

use self::events::AnimateEvent;

pub mod events;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AnimateEvent>() 

            .add_systems(Update, walk_animation)
            .add_systems(Update, path_animator_update)
            ;
    }
}


pub fn walk_animation(    
    mut commands: Commands,
    mut ev_animate: EventReader<AnimateEvent>,
) {
    for ev in ev_animate.read() {
        let mut path = ev.path.clone();

        let mut path_animation: VecDeque<Vec3> = VecDeque::new();
        while !ev.path.is_empty() {
            let step = path.pop_front();
            let Some(current_step) = step else { break };
            let world_position = get_world_position(&current_step);        //TODO Est ce qu'un calcul de position Render doit etre là? Bof.
            let target = Vec3::new(world_position.0, world_position.1, 2.0);
            path_animation.push_back(target);
        }
        println!("PathAnimator created");
        commands.entity(ev.entity).insert(PathAnimator{path:VecDeque::from(path_animation), wait_anim: true});        
    }
}