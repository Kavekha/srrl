
// => 0.21a : DOCUMENTATION
/*
Dans item_structs RawS, ajouter l'entrée de la nouvelle liste des données.
Faire une struct Raw (sans s).



 */

use std::{error::Error, fs::{self, DirEntry}, path::Path};
use csv::{Reader, StringRecord};

use bevy::prelude::*;

mod item_structs;
mod raw_master;

pub use raw_master::*;
use std::sync::Mutex;

use crate::{game::{pieces::{components::{Health, Melee, Npc, Occupier, Piece, Ranged, Stats, Walk}, spawners::Kind}, tileboard::components::BoardPosition}, raws::item_structs::Raws, vectors::Vector2Int};

use self::item_structs::{Raw, RawKind, RawSkill, RawsOld};

lazy_static! {
    pub static ref RAWS : Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}


//A gerer : CSV deserialize error: record 1 (line: 1, byte: 84): field 6: provided string was not `true` or `false`
pub fn load_raws() {
    println!("Loading raws...");

    let paths = fs::read_dir("./raws/").unwrap();
    let mut raws = Raws::new();
    
    for path in paths {
        // On regarde le nom du fichier pour voir quel type on va créer dans nos raws.
        match path {
            Err(_) => { panic!("No file")},
            Ok(file) => {                 
                match read_convert_raw( &file.path())  {
                    Err(err) => { 
                        println!("{}", err); 
                    },
                    Ok(success) => {
                        //let file_name = Path::new(&file.file_name()).file_stem().unwrap();
                        println!("filename with no extension : {:?}", Path::new(&file.file_name()).file_stem().unwrap());
                        raws.kinds = success;
                    }
                }     
            }
        }
    }
    RAWS.lock().unwrap().load(raws);
}




fn read_convert_raw(
    path: &Path,
) -> Result<Vec<Raw>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)?;
    let mut raws = Vec::new();
    for result in rdr.deserialize() {
        let record: Raw = result?;
        println!("record for raw : {:?}", record);
        raws.push(record);
    }
    Ok(raws)
}



pub fn load_raws_v1(){
    println!("Loading raws...");

    /* 

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
    */       
}




fn read_convert_all_raws() -> Result<RawsOld, Vec<Box<dyn Error>>> {
    let mut raws = RawsOld::new();
    let mut errors: Vec<Box<dyn Error>> = Vec::new();

    match read_convert_kind_raw() {
        Err(err) => { errors.push(err); },
        Ok(success) => {raws.kinds = success;}
    }    

    if errors.is_empty() {
        Ok(raws)        
    } else {
        Err(errors)
    }
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


pub fn spawn_named_kind(
    raws: &RawMaster,
    world: &mut World, 
    key: &str,
    position: Vector2Int,
){
    /*
) -> Option<Entity> {
 
    println!(">>> SPAWNING key {:?} ", key);
    if raws.kind_index.contains_key(key) {
        let kind_template = &raws.raws.kinds[raws.kind_index[key]];
        println!("spawn_named_kind: Kind template : {:?}", kind_template);

        let entity = world.spawn_empty().id();
        //world.entity_mut(entity).insert(Npc);
        world.entity_mut(entity).insert(BoardPosition { v: position });
        //world.entity_mut(entity).insert(Occupier);

        if kind_template.can_melee {
            world.entity_mut(entity).insert(Melee);
        }
        if kind_template.can_ranged {
            world.entity_mut(entity).insert(Ranged);
        }
        if kind_template.can_walk {
            world.entity_mut(entity).insert(Walk);
        }
 
               
        let piece = Piece { 
               kind: Kind::Human,
               model: kind_template.model.clone(),
        };
        println!("Model is {:?}", piece.model.clone());
        world.entity_mut(entity).insert(piece);    

         
        let stats = Stats {
            strength: kind_template.strength,
            agility: kind_template.agility,
            logic: kind_template.logic,
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

        return Some(entity)
    } else {
        info!("rawmaster: Kind template for {:?} non présent.", key);
        return None
    }
    */
}


// A deplacer dans Rules.
fn get_health(stats: Stats) -> u32 {
    return (stats.strength / 2) + 8
}