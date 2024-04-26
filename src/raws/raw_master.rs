use std::collections::HashMap;

use super::item_structs::Raws;

pub struct RawMaster {
    pub raws : Raws,
    pub kind_index : HashMap<String, usize>,
    pub stat_index : HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws : Raws{ kinds: Vec::new(), stats: Vec::new() },
            kind_index : HashMap::new(),
            stat_index : HashMap::new(),
        }
    }

    pub fn load(&mut self, raws : Raws) {
        //println!("rawmaster: Raws {:?}", raws);
        self.raws = raws;
        self.kind_index = HashMap::new();
        for (i,kind) in self.raws.kinds.iter().enumerate() {
            //println!("rawmaster: {}, {:?}", i, kind);
            self.kind_index.insert(kind.name.clone(), i);
        }
        self.stat_index = HashMap::new();
        for (i,stat) in self.raws.stats.iter().enumerate() {
            println!("rawmaster: {}, {:?}", i, stat);
            self.stat_index.insert(stat.name.clone(), i);
        }
    }    
}