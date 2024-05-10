
use std::cmp::{max, min};

use bevy::{prelude::*, utils::{HashMap, HashSet}};
use serde::Deserialize;

use crate::{
    engine::render::components::Renderable, 
    game::{game_generation::{character_creation::components::{Attribute, Attributes, Health, Melee, Occupier, Ranged, Skill, Skills, Vision, Walk}, 
    random_table::RandomTable}, tileboard::components::BoardPosition}, vectors::Vector2Int};

use super::{base_attributes_structs::BaseAttributes, job_table_structs::JobTable, jobs_structs::{RawJob}, kind_structs::{RawKind, RawRenderable}, spawn_table_structs::SpawnTable};



#[derive(Deserialize, Debug)]
pub struct Raws {
    pub kinds : Vec<RawKind>,
    pub spawn_tables: Vec<SpawnTable>,
    pub base_attributes: Vec<BaseAttributes>,
    pub jobs: Vec<RawJob>,
    pub job_tables: Vec<JobTable>
}


pub struct RawMaster {
    pub raws : Raws,
    pub kind_index : HashMap<String, usize>,
    pub spawn_table_index: HashMap<String, usize>,
    pub base_attributes_index: HashMap<String, usize>,
    pub job_index: HashMap<String, usize>,
    pub job_table_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws : Raws{ 
                kinds: Vec::new(), 
                spawn_tables: Vec::new(),
                base_attributes: Vec::new(),
                jobs: Vec::new(),
                job_tables: Vec::new(),
            },
            kind_index : HashMap::new(),
            spawn_table_index: HashMap::new(),
            base_attributes_index: HashMap::new(),
            job_index: HashMap::new(),
            job_table_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws : Raws) {
        //println!("rawmaster: Raws {:?}", raws);
        println!("Rawmaster: load...");
        self.raws = raws;
        self.kind_index = HashMap::new();
        self.spawn_table_index = HashMap::new();
        self.base_attributes_index = HashMap::new();
        self.job_index = HashMap::new();
        self.job_table_index = HashMap::new();

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
        for (i,base_attributes) in self.raws.base_attributes.iter().enumerate() {
            // Le base_attributes enrichie un profil existant, on ne verifie pas la duplication car on ne fait pas des appels directs à la base.
            self.base_attributes_index.insert(base_attributes.reference.clone(), i);
        }
        for (i,job) in self.raws.jobs.iter().enumerate() {
            if used_references.contains(&job.reference) {
                println!("WARNING : duplicate JOB reference in raw [{}]", job.reference);
            }
            self.job_index.insert(job.reference.clone(), i);
            used_references.insert(job.reference.clone());
        }
        for (i,job_table) in self.raws.job_tables.iter().enumerate() {
            // Le base_attributes enrichie un profil existant, on ne verifie pas la duplication car on utilise le meme referent que le npc que l'on veut créer.
            self.job_table_index.insert(job_table.reference.clone(), i);
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

        // Attributes Max & Base Attributes.
        let mut strength = 1;
        let mut agility = 1;
        let mut logic = 1;
        if raws.base_attributes_index.contains_key(key) {
            let base_attributes_template = &raws.raws.base_attributes[raws.base_attributes_index[key]];
            if let Some(str) = base_attributes_template.strength {strength += str };
            if let Some(agi) = base_attributes_template.agility {agility += agi };
            if let Some(log) = base_attributes_template.logic {logic += log };
        }
        
        world.entity_mut(entity).insert( Attributes {
            strength: Attribute {
                base: strength,
                modifiers: 0,
                max: kind_template.attributes.strength_max,
            },
            agility: Attribute {
                base: agility,
                modifiers: 0,
                max: kind_template.attributes.agility_max,
            },
            logic: Attribute {
                base: logic,
                modifiers: 0,
                max: kind_template.attributes.logic_max,
            },
        });

        let health = (strength / 2) + 8;
        world.entity_mut(entity).insert( Health { current: health, max: health});

    return Some(entity)
    } else {
        info!("No reference for key {:?}", key);
        return None
    }    
}

fn get_renderable_component(
    renderable: &RawRenderable
) -> Renderable {
    Renderable {
        model: renderable.model.clone()
    }
}

pub fn get_kind<'a>(
    raws: &'a RawMaster,
    key: &'a str
) -> Option<&'a RawKind> {
    if raws.kind_index.contains_key(key) {
        return Some(&raws.raws.kinds[raws.kind_index[key]])
    }
    None
}

pub fn get_job<'a>(
    raws: &'a RawMaster,
    key: &'a str
) -> Option<&'a RawJob> {
    if raws.job_index.contains_key(key) {
        return Some(&raws.raws.jobs[raws.job_index[key]])
    }
    None
}

pub fn get_playable_jobs(
    raws: &RawMaster
) -> Vec<String> {   
    println!("Getting playable jobs.");
    let mut playable_jobs = Vec::new();
    for (key, index) in raws.job_index.iter() {
        println!("Key is {:?}", key);        
        let template = &raws.raws.jobs[*index];
        if template.is_playable {
            playable_jobs.push(key.clone());
        }        
    }
    return playable_jobs
}

pub fn get_playable_kinds(
    raws: &RawMaster
) -> Vec<String> {
    println!("Getting playable kinds.");
    let mut playable_kinds = Vec::new();
    for (key, index) in raws.kind_index.iter() {
        println!("Key is {:?}", key);        
        let template = &raws.raws.kinds[*index];
        if template.is_playable {
            playable_kinds.push(key.clone());
        }        
    }
    return playable_kinds
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

pub fn get_job_table(raws: &RawMaster, key: &str) -> RandomTable {
    let mut random_table = RandomTable::new();
    if raws.job_table_index.contains_key(key) {
        let st_template = &raws.raws.job_tables[raws.job_table_index[key]].job;  

        for entry in st_template {
            random_table = random_table.add(entry.reference.clone(), entry.weight);
        }
    }
    random_table
}

pub fn apply_referenced_job(
    raws: &RawMaster,
    world: &mut World, 
    key: &str,
    entity: Entity
){
    println!("apply referenced job : {:?} for {:?}", key, entity);
    if raws.job_index.contains_key(key) {
        let job_template =  &raws.raws.jobs[raws.job_index[key]];
        let mut entity_ref = world.entity_mut(entity);
        let mut attributes = entity_ref.get_mut::<Attributes>().unwrap();

        // Attributes modifiers
        if let Some(strength) = job_template.strength { 
            attributes.strength.base += strength;
            attributes.strength.base = min(max(0, attributes.strength.base), attributes.strength.max);
        }
        if let Some(agility) = job_template.agility { 
            attributes.agility.base += agility;
            attributes.agility.base = min(max(0, attributes.agility.base), attributes.agility.max);
        }
        if let Some(logic) = job_template.logic { 
            attributes.logic.base += logic;
            attributes.logic.base = min(max(0, attributes.logic.base), attributes.logic.max);
        }
        // Skills.
        let mut skills = Skills{ skills: HashMap::new() }; 
        if let Some(rawskills) = &job_template.skills {
            for skill in rawskills.iter() {
                match skill.0.as_str() { 
                    "UnarmedCombat" => { skills.skills.insert(Skill::UnarmedCombat, *skill.1); },
                    "FireArms" => { skills.skills.insert(Skill::FireArms, *skill.1); },
                    _ => { println!("WARNING : Unkwnown skill referenced : [{}]", skill.0)}
                }
            }
        }
        println!("Apply job. Attributes are now {:?}.", attributes);
        
        world.entity_mut(entity).insert( skills);

        
    } else {
        println!("WARNING: No job for key {}", key);
    }

}
