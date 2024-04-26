use std::error::Error;
use csv::{Reader, StringRecord};

use bevy::prelude::*;

mod item_structs;
mod raw_master;

pub use raw_master::*;
use std::sync::Mutex;

use crate::raws::item_structs::{KindRaw, Raws};

lazy_static! {
    pub static ref RAWS : Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}


pub fn load_raws(){
    println!("Loading raws...");

    match read_convert_kind_raw() {
        Err(err) => { println!("{}", err); },
        Ok(success) => {
            RAWS.lock().unwrap().load(success);
        }
    }        
}

fn read_convert_kind_raw() -> Result<Raws, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("./raws/kind.csv")?;
    let mut raws = Raws::new();
    for result in rdr.deserialize() {
        //println!("Result : {:?}", result);
        let record: KindRaw = result?;
        //println!("record: {:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
        raws.kinds.push(record);
    }
    Ok(raws)
}

pub fn spawn_named_kind(
    raws: &RawMaster,
    key: &str,
){
    println!(">>> SPAWNING ");
    if raws.kind_index.contains_key(key) {
        let kind_template = &raws.raws.kinds[raws.kind_index[key]];
        info!("rawmaster: {} - returning template {:?}", key, kind_template);
    } else {
        info!("rawmaster: Kind template for {:?} non présent.", key);
    }
}