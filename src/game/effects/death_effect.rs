use bevy::prelude::*;

use crate::{engine::{asset_loaders::GraphicsAssets, audios::SoundEvent}, game::{combat::{combat_system::components::{ActionPoints, Die, IsDead}, events::RefreshActionCostEvent}, game_generation::character_creation::components::Occupier, gamelog::LogEvent}};

use super::EffectSpawner;

pub fn inflict_death(
    world: &mut World,
    effect: &EffectSpawner,
    target: Entity
){
    world.entity_mut(target).insert(IsDead);

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

    world.entity_mut(target).remove::<Die>();
    world.entity_mut(target).remove::<ActionPoints>();
    world.entity_mut(target).remove::<Occupier>();

    let mut entity_mut = world.entity_mut(target);

    /*  TODO : Comment je change ca moi?! J'ecrase?
    {
        if let Some(graph_assets) = world.get_resource::<GraphicsAssets>() {        
            if let Some(mut body) = entity_mut.get_mut::<Handle<Image>>(){
                *body = graph_assets.textures["blood"].clone();
            }        
        }
    } */
}
