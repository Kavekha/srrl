use serde::Deserialize;

use super::spawn_table_structs::SpawnTableEntry;

#[derive(Deserialize, Debug)]

pub struct JobTable {
    pub reference: String, 
    pub job: Vec<SpawnTableEntry>,
}
