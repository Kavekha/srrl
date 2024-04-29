use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BaseAttributes {
    pub reference : String,
    pub strength: i32,
    pub agility: i32,
    pub logic: i32
}
