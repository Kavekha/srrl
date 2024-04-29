use std::collections::HashMap;

use bevy::{prelude::*, utils::HashSet};

use crate::{game::{game_generation::{character_creation::components::{Health, Melee, Occupier, Piece, Ranged, Stats, Vision, Walk}, random_table::RandomTable}, tileboard::components::BoardPosition}, vectors::Vector2Int};

use super::kind_structs::{RawRenderable, Raws};


pub struct RawMaster {
    pub raws : Raws,
    pub kind_index : HashMap<String, usize>,
    pub spawn_table_index: HashMap<String, usize>
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws : Raws{ 
                kinds: Vec::new(), 
                spawn_tables: Vec::new()
            },
            kind_index : HashMap::new(),
            spawn_table_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws : Raws) {
        //println!("rawmaster: Raws {:?}", raws);
        println!("Rawmaster: load...");
        self.raws = raws;
        self.kind_index = HashMap::new();
        self.spawn_table_index = HashMap::new();

        let mut used_references : HashSet<String> = HashSet::new();
        
        for (i,kind) in self.raws.kinds.iter().enumerate() {
            if used_references.contains(&kind.reference) {
                println!("WARNING : duplicate kind reference in raw [{}]", kind.reference);
            }
            self.kind_index.insert(kind.reference.clone(), i);
            used_references.insert(kind.reference.clone());
        }
        for (i,spawn_table) in self.raws.spawn_tables.iter().enumerate() {
            if used_references.contains(&spawn_table.reference) {
                println!("WARNING : duplicate kind reference in raw [{}]", spawn_table.reference);
            }
            self.spawn_table_index.insert(spawn_table.reference.clone(), i);
            used_references.insert(spawn_table.reference.clone());
        }
    }    
}



pub fn spawn_referenced_entity(
    raws: &RawMaster,
    world: &mut World, 
    key: &str,
    position: Vector2Int,
) -> Option<Entity> {
    if raws.kind_index.contains_key(key) {
        return spawn_referenced_kind(raws, world, key, position)
    }
    None
}

fn spawn_referenced_kind(
    raws: &RawMaster,
    world: &mut World, 
    key: &str,
    position: Vector2Int,
) -> Option<Entity> {
    if raws.kind_index.contains_key(key) {
        let kind_template = &raws.raws.kinds[raws.kind_index[key]];

        let entity = world.spawn_empty().id();
        world.entity_mut(entity).insert(BoardPosition {v: position});

        if let Some(renderable) = &kind_template.renderable {
            world.entity_mut(entity).insert(get_renderable_component(renderable));
        }

        world.entity_mut(entity).insert(Name::new(kind_template.name.clone()));

        if kind_template.is_occupier { world.entity_mut(entity).insert(Occupier); }
        if kind_template.can_melee { world.entity_mut(entity).insert(Melee); }
        if kind_template.can_ranged { world.entity_mut(entity).insert(Ranged); }
        if kind_template.can_walk { world.entity_mut(entity).insert(Walk); }

        world.entity_mut(entity).insert(Vision { range_view : kind_template.vision.range_view} );

        world.entity_mut(entity).insert( Stats {
            strength: kind_template.stats.strength,
            agility: kind_template.stats.agility,
            logic: kind_template.stats.logic,
            melee: 0,
            firearms: 0,
        });

        let health = (kind_template.stats.strength / 2) + 8;
        world.entity_mut(entity).insert( Health { current: health, max: health});


    return Some(entity)
    } else {
        info!("No reference for key {:?}", key);
        return None
    }    
}

fn get_renderable_component(
    renderable: &RawRenderable
) -> Piece {
    Piece {
        model: renderable.model.clone()
    }
}


pub fn get_spawn_table(raws: &RawMaster, key: &str) -> RandomTable {
    let mut random_table = RandomTable::new();
    if raws.spawn_table_index.contains_key(key) {
        let st_template = &raws.raws.spawn_tables[raws.spawn_table_index[key]].spawn;  

        for entry in st_template {
            random_table = random_table.add(entry.reference.clone(), entry.weight);
        }
    }
    random_table
}
