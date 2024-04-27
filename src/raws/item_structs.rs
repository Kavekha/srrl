use serde::Deserialize;


#[derive(Debug)]
pub struct Raws {
    pub kinds : Vec<RawKind>,
}
impl Raws{
    pub fn new() -> Raws {
        Raws {
            kinds: Vec::new(),
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Raw {
    pub name: String,
    pub playable: Option<bool>,
    pub can_melee: Option<bool>,
    pub can_ranged: Option<bool>, 
    pub can_walk: Option<bool>,
    pub range_view: Option<bool>,
    pub model: Option<String>,
    pub strength: Option<u32>,
    pub agility: Option<u32>,
    pub logic: Option<u32>,
}





#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct RawKind {
    pub name: String,
    pub playable: bool,
    pub can_melee: bool,
    pub can_ranged: bool, 
    pub can_walk: bool,
    pub range_view: u32,
    pub model: String,
    pub strength: u32,
    pub agility: u32,
    pub logic: u32
}



#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct RawSkill {
    pub name: String,
    pub firearms: u32,
    pub melee: u32,
}

