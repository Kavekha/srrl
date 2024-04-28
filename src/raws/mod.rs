
// => 0.21a : DOCUMENTATION
/*
Dans item_structs RawS, ajouter l'entrée de la nouvelle liste des données.
Faire une struct Raw (sans s).



 */

use std::{error::Error, fs, path::Path};

use bevy::prelude::*;

mod item_structs;
mod raw_master;

pub use raw_master::*;
use std::sync::Mutex;

use crate::{game::{pieces::{components::{Health, Melee, Piece, Ranged, Stats, Walk}, spawners::Kind}, tileboard::components::BoardPosition}, raws::item_structs::Raws, vectors::Vector2Int};

use self::item_structs::Raw;

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
                //let file_name = Path::new(&file.file_name()).file_stem().unwrap();
                println!("filename with no extension : {:?}", Path::new(&file.file_name()).file_stem().unwrap()); 
                match read_convert_raw( &file.path())  {
                    Err(err) => { 
                        panic!("{}", err); 
                    },
                    Ok(mut success) => {                        
                        raws.kinds.append(&mut success);
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



pub fn spawn_named_kind(
    raws: &RawMaster,
    world: &mut World, 
    key: &str,
    position: Vector2Int,
) -> Option<Entity> {
 
    println!(">>> SPAWNING key {:?} ", key);
    if raws.kind_index.contains_key(key) {
        let kind_template = &raws.raws.kinds[raws.kind_index[key]];
        println!("spawn_named_kind: Kind template : {:?}", kind_template);

        let entity = world.spawn_empty().id();
        //world.entity_mut(entity).insert(Npc);
        world.entity_mut(entity).insert(BoardPosition { v: position });
        //world.entity_mut(entity).insert(Occupier);

        if let Some(can_melee) = kind_template.can_melee {
            if can_melee { world.entity_mut(entity).insert(Melee); }            
        }
        if let Some(can_ranged) = kind_template.can_ranged {
            if can_ranged { world.entity_mut(entity).insert(Ranged); }
        }
        if let Some(can_walk) = kind_template.can_walk {
            if can_walk { world.entity_mut(entity).insert(Walk); }
        }
        
        if let Some(model) = kind_template.model.clone() {
            let piece = Piece { 
                kind: Kind::Human,
                model: model.clone(),
            };
            println!("Model is {:?}", piece.model.clone());
            world.entity_mut(entity).insert(piece);
        }

        if let Some(_strength) = kind_template.strength {
            let stats = Stats {
                strength: 1, //kind_template.strength,
                agility: 1,  //kind_template.agility,
                logic: 1,   //kind_template.logic,
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
        }           
        return Some(entity)
    } else {
        info!("rawmaster: Kind template for {:?} non présent.", key);
        return None
    }
}


// A deplacer dans Rules.
fn get_health(stats: Stats) -> u32 {
    return (stats.strength / 2) + 8
}