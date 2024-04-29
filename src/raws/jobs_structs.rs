use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawJobs {
    pub reference : String,
    pub strength: Option<i32>,
    pub agility: Option<i32>,
    pub logic: Option<i32>,
    pub skills: Option<HashMap<String, i32>>,  
}
