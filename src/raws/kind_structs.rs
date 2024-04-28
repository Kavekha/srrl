use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Raws {
    pub kinds : Vec<Kind>
}

#[derive(Deserialize, Debug)]
pub struct Kind {
    pub reference : String,
    pub name : String,
    pub renderable: Option<RawRenderable>,
    pub is_playable: bool,
    pub can_melee: bool,
    pub can_ranged: bool,
    pub can_walk: bool, 
    pub is_occupier: bool,
    pub vision: RawVision,
    pub stats: RawStats,

}

#[derive(Deserialize, Debug)]
pub struct RawRenderable {
    pub model: String
}

#[derive(Deserialize, Debug)]
pub struct RawVision {
    pub range_view: u32
}

#[derive(Deserialize, Debug)]
pub struct RawStats {
    pub strength: u32,
    pub agility: u32,
    pub logic: u32
}