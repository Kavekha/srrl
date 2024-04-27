use std::collections::HashMap;

use super::item_structs::Raws;

pub struct RawMaster {
    pub raws : Raws,
    pub kind_index : HashMap<String, usize>,
    pub model_index: HashMap<String, usize>
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws : Raws{ 
                kinds: Vec::new(),
                models: Vec::new(), 
            },
            kind_index : HashMap::new(),
            model_index: HashMap::new(),
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
        self.model_index = HashMap::new();
        for (i,model) in self.raws.models.iter().enumerate() {
            println!("rawmaster: {}, {:?}", i, model);
            self.model_index.insert(model.name.clone(), i);
        }
    }    
}