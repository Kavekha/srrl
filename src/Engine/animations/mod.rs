// => DOCUMENTATION
/*
Pour le moment, on envoit un event EffectEvent, avec l'id de l'effet + les positions où il doit apparaitre.
On ne fait que du 32x32 en 3 images, en 0.1 sec.
spawn_hit_effect contient les infos necessaires de base, avec le timing, le nb d'images et la taille si ce doit être rendu configurable.
 */

use std::collections::VecDeque;

use bevy::prelude::*;

pub mod events;

use crate::engine::asset_loaders::GraphicsAssets;
use crate::game::despawn_component;
use crate::globals::ORDER_EFFECT;


use crate::{
    engine::{
        render::{get_world_position,components::GameCursorRender,},
        animations::events::{AnimateEvent,GraphicsWaitEvent, PathAnimator, EffectEvent, AnimationIndices, RemoveEntity, AnimationTimer}
    },
    game::{combat::CombatSet,player::Cursor},globals::{POSITION_TOLERANCE, BASE_SPEED, SPEED_MULTIPLIER, CURSOR_SPEED, ORDER_CURSOR},
};


pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AnimateEvent>() 
            .add_event::<GraphicsWaitEvent>() 
            .add_event::<EffectEvent>()

            .add_systems(Update, walk_animation)
            .add_systems(Update, path_animator_update.in_set(CombatSet::Animation))   // 3 fois le system => 3 fois plus vite. lol.
            .add_systems(Update, update_game_cursor.in_set(CombatSet::Animation))  
            .add_systems(Update, update_game_cursor)     
            .add_systems(Update, animate_sprite)
            .add_systems(Update, spawn_hit_effect.run_if(on_event::<EffectEvent>()))
            .add_systems(Update, clean_animations)
            ;
    }
}



// Necessaire, mais est-ce la bonne manière?
fn clean_animations(
    mut commands: Commands,
    mut remove_q: Query<Entity, With<RemoveEntity>>
) {
    despawn_component(remove_q, &mut commands);
}

fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (entity, indices, mut timer, mut atlas) in &mut query {
        //println!("Effect timer: animate sprite.");
        timer.tick(time.delta());
        if timer.just_finished() {
            println!("for entity {:?}, index is now : {:?}", entity, atlas.index);
            if atlas.index == indices.last {
                println!("Please remove entity {:?}", entity);
                commands.entity(entity).insert(RemoveEntity);        
            } else {
                atlas.index += 1
            };
        }
    }
}

// Ne fonctionne que pour un cas pour le moment. Rendre configurable via l'Event à l'origine du spawn effect
pub fn spawn_hit_effect(
    mut commands: Commands,
    mut ev_spawn_effect: EventReader<EffectEvent>,
    mut graph_assets: ResMut<GraphicsAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for event in ev_spawn_effect.read() {
        println!("Creating effect");
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
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)), // Repeating car on passe par autant d'etapes que d'images.
        ));
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



pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>
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
        
        if animator.wait_anim {
            ev_wait.send(GraphicsWaitEvent);
            //println!("wait_anim: True");
        }
        
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