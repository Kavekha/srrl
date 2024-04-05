use std::collections::HashMap;

// => DOCUMENTATION 0.19
/*
L'objectif ici est d'avoir la possibilité pour le joueur de choisir entre plusieurs actions.
A voir comment on va gérer ça, on avance en mode agile
L'idée serait d'avoir un portefeuille d'abilités et de pouvoir les utiliser.
Cela est réservé au joueur pour le moment.

*/
use bevy::prelude::*;


// 0.19 - A tester: solution aux deux modes de combat?
// TODO : Doit être agnostique vis a vis des Abilities.
#[derive(Component)]
pub struct Abilities {
    slots: HashMap<u8, AbilityAttack>,
}
impl Abilities {
    pub fn new() -> Abilities {
        Abilities {
            slots: HashMap::new(),
        }
    }
    pub fn add(&mut self, ability:AbilityAttack) -> bool {
        let mut ability_added = false;
        for slot in 0..3 {    // MAX_ABILITY_SLOTS
            match self.slots.get(&slot) {
                None => {
                    self.slots.insert(slot, ability);
                    ability_added = true;
                    break;
                },
                Some(_) => continue
            };
        };
        return ability_added
    }
}


#[derive(Debug)]
pub struct AbilityAttack {
    pub name: String, 
    pub reach: u32,
    pub dmg: u32,
    pub ap_cost: u32, 
    pub description: String,
    pub ability_type: AbilityType
}

#[derive(Debug)]
pub enum AbilityType {
    Melee,
    Ranged
}

pub fn create_ability_attack (
    request: AbilityType    
) -> AbilityAttack {
    match request {
        AbilityType::Melee => {
            return AbilityAttack {
                name: "Melee attack".to_string(),
                reach: 1,
                dmg: 1,
                ap_cost: 3,
                description: "A good old punch in the face.".to_string(),
                ability_type: AbilityType::Melee
            }
        },
        AbilityType::Ranged => {
            return AbilityAttack {
                name: "Ranged attack".to_string(),
                reach: 10,
                dmg: 3,
                ap_cost: 5,
                description: "Headshot!".to_string(),
                ability_type: AbilityType::Ranged
            }
        }
    };
}