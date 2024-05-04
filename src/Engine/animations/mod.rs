// => DOCUMENTATION
/*
0.21g : Les effets graphiques dit "particles" commencent à être générées dans Effects.
On fait un add_effect particle avec id. La position est determiné selon le type de target.

Reste en dur pour le moment:
- la taille 32x32
- Le nombre d'images d'animation (3)
- La boucle de 0.1 sec
- Se termine à la fin de l'animation.

On va devoir rendre ca configurable.
 */

use bevy::prelude::*;
use crate::commons::despawn_component;


use crate::{
    engine::animations::events::{AnimateEvent,GraphicsWaitEvent, AnimationIndices, RemoveEntity, AnimationTimer},
    game::combat::CombatSet,
};

use self::animation_systems::{path_animator_update, update_game_cursor, walk_animation};


pub mod events;
pub mod animation_systems;


pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AnimateEvent>() 
            .add_event::<GraphicsWaitEvent>()

            .add_systems(Update, walk_animation)
            .add_systems(Update, path_animator_update.in_set(CombatSet::Animation))   // 3 fois le system => 3 fois plus vite. lol.
            .add_systems(Update, update_game_cursor.in_set(CombatSet::Animation))  
            .add_systems(Update, update_game_cursor.in_set(CombatSet::Animation))     
            .add_systems(Update, animate_sprite.in_set(CombatSet::Animation))
            //.add_systems(Update, spawn_hit_effect.run_if(on_event::<EffectEvent>()))
            .add_systems(Update, clean_animations)
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
