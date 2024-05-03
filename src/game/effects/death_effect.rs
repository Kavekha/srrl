use bevy::prelude::*;

use crate::{engine::{asset_loaders::GraphicsAssets, audios::SoundEvent}, game::{combat::{combat_system::components::{ActionPoints, Die, IsDead}, events::RefreshActionCostEvent}, game_generation::character_creation::components::Occupier, gamelog::LogEvent}, globals::ORDER_CORPSE};

use super::EffectSpawner;

pub fn inflict_death(
    world: &mut World,
    effect: &EffectSpawner,
    target: Entity
){
    world.entity_mut(target).insert(IsDead);
    world.entity_mut(target).insert(Die);   // Necessaire pour gerer la transformation du corps... contournement du borrow world.

    // transformation en corps.

    // TODO : son de mort = Effect.
    world.send_event(SoundEvent{id:"death_scream".to_string()});

    world.send_event(RefreshActionCostEvent);

    let entity_ref = world.entity(target);
    //log
    if let Some(killer) = effect.creator {
        let attacker_ref = world.entity(killer);
        if let Some(name) = entity_ref.get::<Name>() {
            if let Some(attacker_entity_name) = attacker_ref.get::<Name>() {
                world.send_event(LogEvent {entry: format!("{:?} has been killed by {:?}!", name, attacker_entity_name)});   // Log v0
            } else {
                world.send_event(LogEvent {entry: format!("{:?} died!", name)});   // Log v0
            }

        }
    }  

    // Trop galere de faire ça avec world a cause de borrowing.
    let transform_body = world.register_system(transform_dead_body);
        let _result = world.run_system(transform_body);
    
}


pub fn transform_dead_body(
    mut commands: Commands,    
    mut die_q: Query<(Entity, &mut Transform), With<Die>>,   
    mut body_q: Query<&mut Handle<Image>>,
    graph_assets: Res<GraphicsAssets>,    
) {
    let mut to_remove=Vec::new();
    for (entity, mut transform) in die_q.iter_mut() {
        to_remove.push(entity);        

        // Transformation en Corps.        
        if let Ok(mut body) = body_q.get_mut(entity) {
            *body = graph_assets.textures["blood"].clone();     //TODO : A ajouter aux data.
        };
        transform.translation.z = ORDER_CORPSE;
        
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Die>();
        commands.entity(entity).remove::<ActionPoints>();
        commands.entity(entity).remove::<Occupier>();
    }
}