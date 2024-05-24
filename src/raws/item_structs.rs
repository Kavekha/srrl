use bevy::prelude::*;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct RawItem {
    pub reference : String,
    pub name : String,
    pub weapon: Option<RawWeapon>,
    pub armor: Option<RawArmor>
}

#[derive(Deserialize, Debug)]
pub struct RawWeapon {
    pub range: String,
    pub offensive_score: i32
}

#[derive(Deserialize, Debug)]
pub struct RawArmor {
    pub defensive_score: i32
}