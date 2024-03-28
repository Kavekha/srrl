//https://gist.github.com/chamons/37e8c6f8753e63eaef08bef36686c2e2

// == DOCUMENTATION
// Ces elements sont nécessaires à la sauvegarde, appelée dans save_messages qui est complementaire de ce code.

use bevy::ecs::archetype::{Archetype, ArchetypeId};
use serde::{Deserialize, Serialize};
use std::path::Path;
use bevy::prelude::*;

//pub struct SaveLoadPlugin;

use crate::game::pieces::components::{Walk, Piece, Health, Melee, Occupier, Stats, Npc, Monster};   //Actor
use crate::game::player::Player;
use crate::game::tileboard::components::BoardPosition;
use crate::globals::SCENE_FILE_PATH;
use crate::map_builders::map::Map;


pub fn has_save_file() -> bool {
    Path::new(SCENE_FILE_PATH).exists()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub map: Map,
    pub entities: Vec<SaveEntity>,
}


// Bool if marker, Option if data.
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveEntity {
    pub entity: Entity,
    pub player: bool, 
    pub stats: Option<Stats>,
    pub npc: bool, 
    pub monster: bool,
    pub piece: Option<Piece>,
    pub position: Option<BoardPosition>,
    pub health: Option<Health>,
    //actor: Option<Actor>, //actor can't be added there. Need to be put back on load with some logic..
    pub walk: bool,
    pub melee: Option<Melee>,
    pub occupier: bool,
}

impl SaveState {
    pub fn create(world: &mut World) -> Self {
        println!("Saving... savestate start.");
        let map = world.get_resource::<Map>().unwrap().clone();
        println!("Saving... map unwraped.");
        SaveState {
            map,
            entities: SaveState::snapshot_entities(world),
        }
    }

    fn snapshot_entities(world: &World) -> Vec<SaveEntity> {
        println!("Saving.... Snapshot entities.");
        let archetypes = world.archetypes();
        let all_archetypes: Vec<&Archetype> = archetypes
            .iter()
            .filter(|archetype| match archetype.id() {
                ArchetypeId::EMPTY |ArchetypeId::INVALID => false,
                _ => true,
            })
            .collect();

        let mut entities = Vec::with_capacity(all_archetypes.len());

        for archetype in all_archetypes {
             //// DEBUG: println!("Archetype id is {:?}", archetype.id());

            let mut has_player = false;    //DEBUG
            for archetype_entity in archetype.entities() {

                let current_entity = &archetype_entity.id();
    
                let mut has_component_to_save = false;
                if world.get::<Player>(world.entity(*current_entity).id()).is_some()
                || world.get::<Npc>(world.entity(*current_entity).id()).is_some()
                || world.get::<Monster>(world.entity(*current_entity).id()).is_some()
                || world.get::<Stats>(world.entity(*current_entity).id()).is_some()
                || world.get::<Piece>(world.entity(*current_entity).id()).is_some()
                || world.get::<Walk>(world.entity(*current_entity).id()).is_some()
                || world.get::<Health>(world.entity(*current_entity).id()).is_some()
                || world.get::<Melee>(world.entity(*current_entity).id()).is_some()
                //|| world.get::<Occupier>(world.entity(*current_entity).id()).is_some()    //TODO: As for Boardposition, Tile like Wall use Occupier. This has to change!
                {
                    has_component_to_save = true
                }
                
                if world.get::<Player>(world.entity(*current_entity).id()).is_some() {
                    has_player = true
                }                 

                if has_component_to_save {
                    entities.push(SaveEntity {
                        entity: *current_entity,
                        player: world.get::<Player>(*current_entity).is_some(),
                        npc: world.get::<Npc>(*current_entity).is_some(),
                        monster: world.get::<Monster>(*current_entity).is_some(),
                        stats: world.get::<Stats>(*current_entity).cloned(),
                        piece: world.get::<Piece>(*current_entity).cloned(),
                        position: world.get::<BoardPosition>(*current_entity).cloned(),
                        walk: world.get::<Walk>(*current_entity).is_some(),
                        health: world.get::<Health>(*current_entity).cloned(),
                        melee: world.get::<Melee>(*current_entity).cloned(),
                        occupier: world.get::<Occupier>(*current_entity).is_some(),
                    });
                    // DEBUG: println!("Position for entity {:?} is : {:?}", *current_entity, world.get::<BoardPosition>(*current_entity));
                }
            }      
        }
        entities
    }
}
