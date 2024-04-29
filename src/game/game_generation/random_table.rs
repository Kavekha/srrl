use rand::Rng;


pub struct RandomEntry {
    reference : String,
    weight : i32
}

impl RandomEntry {
    pub fn new<S:ToString>(reference: S, weight: i32) -> RandomEntry {
        RandomEntry{ reference: reference.to_string(), weight }
    }
}


#[derive(Default)]
pub struct RandomTable {
    entries : Vec<RandomEntry>,
    total_weight : i32
}
impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable{ entries: Vec::new(), total_weight: 0 }
    }

    pub fn add<S:ToString>(mut self, reference : S, weight: i32) -> RandomTable {
        self.total_weight += weight;
        self.entries.push(RandomEntry::new(reference.to_string(), weight));
        self
    }

    pub fn roll(&self) -> String {
        let mut rng = rand::thread_rng();

        if self.total_weight == 0 { return "None".to_string(); }
        let mut roll = rng.gen_range(1..self.total_weight-1);
        let mut index : usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return self.entries[index].reference.clone();
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        "None".to_string()
    }
}
