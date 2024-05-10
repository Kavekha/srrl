use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct RawKind {
    pub reference : String,
    pub name : String,
    pub renderable: Option<RawRenderable>,
    pub is_playable: bool,
    pub can_melee: bool,
    pub can_ranged: bool,
    pub can_walk: bool, 
    pub is_occupier: bool,
    pub vision: RawVision,
    pub attributes: RawAttributes
}

#[derive(Deserialize, Debug)]
pub struct RawAttributes {
    pub strength_max: i32,
    pub agility_max: i32,
    pub logic_max: i32
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