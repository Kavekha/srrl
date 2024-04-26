use serde::Deserialize;


#[derive(Debug)]
pub struct Raws {
    pub kinds : Vec<KindRaw>,
    pub stats: Vec<StatRaw>
}
impl Raws{
    pub fn new() -> Raws {
        Raws {
            kinds: Vec::new(),
            stats: Vec::new(),
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct KindRaw {
    pub name: String,
    pub stats: String,
    pub playable: bool,
    pub can_melee: bool,
    pub can_ranged: bool, 
    pub can_walk: bool,
    pub range_view: u32,
    pub rendering: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct StatRaw {
    pub name: String,
    pub strength: u32,
    pub agility: u32,
    pub logic: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SkillsRaw {
    pub name: String,
    pub firearms: u32,
    pub melee: u32,
}


