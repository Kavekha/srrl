
// => 0.21a : DOCUMENTATION
/*
Dans item_structs RawS, ajouter l'entrée de la nouvelle liste des données.
Faire une struct Raw (sans s).



 */

use std::fs;

mod spawn_table_structs;
mod raw_master;
mod kind_structs;
mod base_attributes_structs;
mod jobs_structs;
mod job_table_structs;
mod item_structs;

pub use raw_master::*;
use std::sync::Mutex;

use crate::globals::RAWS_FILE_PATH;




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
    //let paths = fs::read_dir("./raws/").unwrap();
    let paths = fs::read_dir(RAWS_FILE_PATH).unwrap();
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
                        //println!("final_file : {:?}", final_file);
                        // On le fait à la suite sinon on retire le tout premier { du fichier.
                        if file.len() > 0 {
                            file.remove(0);  // remove first { ==> Cela permets d'avoir un json valide.
                            //println!("file is : {:?}", file);
                        }
                    }
                    final_file.push_str(&file);
                    //println!("final file in this iteration is {:?}", final_file);
                }
            }
        }
    }
    //println!("final file is :");
    //println!("-----");
    //println!("{:?}", final_file);
    //println!("---------");
    let raws:Raws = serde_json::from_str(&final_file).expect("Unable to parse JSON");
    RAWS.lock().unwrap().load(raws);

    println!("Loaded.");
    //println!("Rawmaster:");
    //println!("{:?}", RAWS.lock().unwrap().kind_index);
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