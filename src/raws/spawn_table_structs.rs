use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct SpawnTable {
    pub reference: String, 
    pub spawn: Vec<SpawnTableEntry>,
}
#[derive(Deserialize, Debug)]
pub struct SpawnTableEntry {
    pub reference : String,
    pub weight : i32,
}