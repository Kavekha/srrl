use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawJobs {
    pub reference : String,
    pub is_playable: bool,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
    pub is_ranged: Option<bool>,
    pub strength: Option<i32>,
    pub agility: Option<i32>,
    pub logic: Option<i32>,
    pub skills: Option<HashMap<String, i32>>,  
}
