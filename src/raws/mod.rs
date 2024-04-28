
// => 0.21a : DOCUMENTATION
/*
Dans item_structs RawS, ajouter l'entrée de la nouvelle liste des données.
Faire une struct Raw (sans s).



 */

use std::fs;

use bevy::prelude::*;

mod spawn_table_structs;
mod raw_master;
mod kind_structs;

pub use raw_master::*;
use std::sync::Mutex;

use crate::{game::{pieces::components::{Health, Melee, Occupier, Piece, Ranged, Stats, Vision, Walk}, tileboard::components::BoardPosition}, raws::kind_structs::Raws, vectors::Vector2Int};

use self::kind_structs::RawRenderable;



lazy_static! {
    pub static ref RAWS : Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}


// v0.21b : 
/* On récupère tous les Json et on en fait une seule enorme string de facon un peu dégueu:
- en retirant à chaque nouvelle iteration le dernier } du fichier commun,
- ajoute une virgule
- retire la première { du fichier suivant, afin d'avoir un json valide.
==> Il faut que chaque json contienne un Type sinon cela provoque un duplicate. 
*/
pub fn load_raws(){
    let paths = fs::read_dir("./raws/").unwrap();
    let mut final_file:String = "".to_string();
    
    for path in paths {
        // On regarde le nom du fichier pour voir quel type on va créer dans nos raws.
        match path {
            Err(_) => { panic!("No file")},
            Ok(dir_file) => {     
                //let file_name = Path::new(&file.file_name()).file_stem().unwrap();
                //println!("path is : {:?}", &file.path());       
                if let Ok(mut file) = fs::read_to_string(&dir_file.path()) {
                    if final_file.len() > 0 {
                        final_file.pop();   // Remove last }
                        final_file.push_str(",");   
                        println!("final_file : {:?}", final_file);
                        // On le fait à la suite sinon on retire le tout premier { du fichier.
                        if file.len() > 0 {
                            file.remove(0);  // remove first { ==> Cela permets d'avoir un json valide.
                            println!("file is : {:?}", file);
                        }
                    }
                    final_file.push_str(&file);
                    println!("final file in this iteration is {:?}", final_file);
                }
            }
        }
    }
    println!("final file is :");
    println!("-----");
    println!("{:?}", final_file);
    println!("---------");
    let raws:Raws = serde_json::from_str(&final_file).expect("Unable to parse JSON");
    RAWS.lock().unwrap().load(raws);

    println!("Loaded.");
    println!("Rawmaster:");
    println!("{:?}", RAWS.lock().unwrap().kind_index);
}

/*
THIS version of load_raws only load 1 json file.
Ca veut dire que tout doit être mis dedans et c'est un peu lourd à la fin.

pub fn load_raws(){
    println!("Loading raws from json...");
    let raw_path  = "./raws/kind.json".to_string();
    if let Ok(file) = fs::read_to_string(raw_path) {
        let raws:Raws = serde_json::from_str(&file).expect("Unable to parse JSON");
        //let raw_string = std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");
        println!("Full raws are: {:?}", raws);
        RAWS.lock().unwrap().load(raws);
    }    
}
*/

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

/* 
//A gerer : CSV deserialize error: record 1 (line: 1, byte: 84): field 6: provided string was not `true` or `false`
pub fn load_raws_v2() {
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

*/