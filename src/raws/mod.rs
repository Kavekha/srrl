use std::error::Error;
use csv::{Reader, StringRecord};

use bevy::prelude::*;

mod item_structs;
mod raw_master;

pub use raw_master::*;
use std::sync::Mutex;

use crate::{game::{pieces::{components::{Health, Melee, NewPiece, Npc, Occupier, Piece, Ranged, Stats, Walk}, spawners::Kind}, tileboard::components::BoardPosition}, raws::item_structs::Raws, vectors::Vector2Int};

use self::item_structs::{RawKind, RawModel, RawSkill, RawStat};

lazy_static! {
    pub static ref RAWS : Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}


pub fn load_raws(){
    println!("Loading raws...");

    match read_convert_all_raws() {
        Err(err) => { 
            for error in err {
                println!("{}", error); 
            }
        },
        Ok(success) => {
            RAWS.lock().unwrap().load(success);
        }
    }        
}


fn read_convert_all_raws() -> Result<Raws, Vec<Box<dyn Error>>> {
    let mut raws = Raws::new();
    let mut errors = Vec::new();

    match read_convert_model_raw() {
        Err(err) => { errors.push(err); },
        Ok(success) => {raws.models = success;}
    }

    match read_convert_kind_raw() {
        Err(err) => { errors.push(err); },
        Ok(success) => {raws.kinds = success;}
    }   
    match read_convert_stat_raw() {
        Err(err) => { errors.push(err); },
        Ok(success) => {raws.stats = success;}
    }   

    if errors.is_empty() {
        Ok(raws)        
    } else {
        Err(errors)
    }
}


fn read_convert_model_raw() -> Result<Vec<RawModel>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("./raws/models.csv")?;
    let mut raws = Vec::new();
    for result in rdr.deserialize() {
        let record: RawModel = result?;
        println!("record for model : {:?}", record);
        raws.push(record);
    }
    Ok(raws)
}


fn read_convert_kind_raw() -> Result<Vec<RawKind>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("./raws/kind.csv")?;
    let mut raws = Vec::new();
    for result in rdr.deserialize() {
        let record: RawKind = result?;
        println!("record for kind : {:?}", record);
        raws.push(record);
    }
    Ok(raws)
}

fn read_convert_stat_raw() -> Result<Vec<RawStat>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("./raws/stats.csv")?;
    let mut raws = Vec::new();
    for result in rdr.deserialize() {
        let record: RawStat = result?;
        println!("record for stat : {:?}", record);
        raws.push(record);        
    }
    Ok(raws)
}



pub fn spawn_named_kind(
    raws: &RawMaster,
    world: &mut World, 
    key: &str,
    position: Vector2Int,
){
    println!(">>> SPAWNING key {:?} ", key);
    if raws.kind_index.contains_key(key) {
        let kind_template = &raws.raws.kinds[raws.kind_index[key]];
        println!("spawn_named_kind: Kind template : {:?}", kind_template);

        let entity = world.spawn_empty().id();
        world.entity_mut(entity).insert(Npc);
        world.entity_mut(entity).insert(BoardPosition { v: position });
        world.entity_mut(entity).insert(Occupier);

        if kind_template.can_melee {
            world.entity_mut(entity).insert(Melee);
        }
        if kind_template.can_ranged {
            world.entity_mut(entity).insert(Ranged);
        }
        if kind_template.can_walk {
            world.entity_mut(entity).insert(Walk);
        }
 
        if raws.model_index.contains_key(&kind_template.model) {
            let new_piece = NewPiece { model: raws.raws.models[raws.model_index[&kind_template.model]].name.clone()};
            println!("Model is {:?}", new_piece.model.clone());
            world.entity_mut(entity).insert(new_piece);           
        }        
        world.entity_mut(entity).insert(Piece { kind: Kind::Human});        // TOCHANGE : le temps de la transition au nouveau fonctionnement.
        
        if raws.stat_index.contains_key(&kind_template.stats) {
            let stats_template = &raws.raws.stats[raws.stat_index[&kind_template.stats]];
            println!("spawn_named_kind: Stat template : {:?}", stats_template);
            let stats = Stats {
                strength: stats_template.strength,
                agility: stats_template.agility,
                logic: stats_template.logic,
                melee: 0,
                firearms: 0,
            };
            let health_points = get_health(stats);
            let health = Health {
                max: health_points,
                current: health_points,
            };
            world.entity_mut(entity).insert( stats );
            world.entity_mut(entity).insert(health);
        }  else {
            println!("Rawmaster: No Stats template.");
        }  
    } else {
        info!("rawmaster: Kind template for {:?} non présent.", key);
    }
}

// A deplacer dans Rules.
fn get_health(stats: Stats) -> u32 {
    return (stats.strength / 2) + 8
}