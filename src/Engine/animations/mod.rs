// => DOCUMENTATION
/*
Pour le moment, on envoit un event EffectEvent, avec l'id de l'effet + les positions où il doit apparaitre.
On ne fait que du 32x32 en 3 images, en 0.1 sec.
spawn_hit_effect contient les infos necessaires de base, avec le timing, le nb d'images et la taille si ce doit être rendu configurable.
 */

use bevy::prelude::*;
use crate::commons::despawn_component;


use crate::{
    engine::animations::events::{AnimateEvent,GraphicsWaitEvent, EffectEvent, AnimationIndices, RemoveEntity, AnimationTimer},
    game::combat::CombatSet,
};

use self::animation_systems::{path_animator_update, spawn_hit_effect, update_game_cursor, walk_animation};
use self::display_text::{display_text_box, TextEvent};


pub mod events;
pub mod animation_systems;
pub mod display_text;


pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AnimateEvent>() 
            .add_event::<GraphicsWaitEvent>() 
            .add_event::<EffectEvent>()
            .add_event::<TextEvent>()

            .add_systems(Update, walk_animation)
            .add_systems(Update, path_animator_update.in_set(CombatSet::Animation))   // 3 fois le system => 3 fois plus vite. lol.
            .add_systems(Update, update_game_cursor.in_set(CombatSet::Animation))  
            .add_systems(Update, update_game_cursor.in_set(CombatSet::Animation))     
            .add_systems(Update, animate_sprite.in_set(CombatSet::Animation))
            .add_systems(Update, spawn_hit_effect.run_if(on_event::<EffectEvent>()))
            .add_systems(Update, clean_animations)
            .add_systems(Update, display_text_box)
            ;
    }
}



// Necessaire, mais est-ce la bonne manière?
fn clean_animations(
    mut commands: Commands,
    remove_q: Query<Entity, With<RemoveEntity>>
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
            if atlas.index == indices.last {
                commands.entity(entity).insert(RemoveEntity);        
            } else {
                atlas.index += 1;
            };
        }
    }
}
