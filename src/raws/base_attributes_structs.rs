use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BaseAttributes {
    pub reference : String,
    pub strength: Option<i32>,
    pub agility: Option<i32>,
    pub logic: Option<i32>
}
