use std::collections::HashMap;

use super::kind_structs::Raws;


pub struct RawMaster {
    pub raws : Raws,
    pub kind_index : HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws : Raws{ 
                kinds: Vec::new(), 
            },
            kind_index : HashMap::new(),
        }
    }

    pub fn load(&mut self, raws : Raws) {
        //println!("rawmaster: Raws {:?}", raws);
        println!("Rawmaster: load...");
        self.raws = raws;
        self.kind_index = HashMap::new();
        for (i,kind) in self.raws.kinds.iter().enumerate() {
            println!("rawmaster: {}, {:?}", i, kind);
            self.kind_index.insert(kind.name.clone(), i);
        }
    }    
}