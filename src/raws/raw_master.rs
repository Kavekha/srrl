use std::collections::HashMap;

use crate::game::game_generation::random_table::RandomTable;

use super::kind_structs::Raws;


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
        for (i,kind) in self.raws.kinds.iter().enumerate() {
            println!("rawmaster: {}, {:?}", i, kind);
            self.kind_index.insert(kind.reference.clone(), i);
        }
        for (i,spawn_table) in self.raws.spawn_tables.iter().enumerate() {
            println!("rawmaster: {}, {:?}", i, spawn_table);
            self.spawn_table_index.insert(spawn_table.reference.clone(), i);
        }
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
