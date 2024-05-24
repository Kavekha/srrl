use bevy::prelude::*;

use crate::game::game_generation::character_creation::components::Skill;

// TODO : L'utiliser pour un Attribute (dans char gen)
pub enum AttributeType {
    Strength,
    Agility,
    Logic
}
pub enum WeaponRange {
    Melee
}
#[derive(Component)]
pub struct Weapon {
    pub range: WeaponRange,
    pub skill: Skill,
    pub attack_attribute: AttributeType,
    pub damage_attribute: AttributeType,
    pub damage_attribute_modifier: i32,
    pub offensive_score: i32
}

#[derive(Component)]
pub struct Armor {
    pub defensive_score: i32
}