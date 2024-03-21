use bevy::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};


#[derive(Component, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Dwarf,
    Elf,
    Human,
    Orc,
    Troll,
    Ghoul   
}

/// TEMP : Renvoie infos rendus pour les differentes races jouables par le PJ.
pub fn get_random_kind(
) -> Kind {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..4);
    //TODO : Le size n'a plus de sens, c'etait une donnée "image" et toutes les images sont maintenant en 64x96.
    match rand {
        0 => { return Kind::Dwarf; }
        1 => { return Kind::Elf; }
        2 => { return Kind::Orc; }
        3 => { return Kind::Troll; }
        4 => { return Kind::Human; }
        _ => { return Kind::Human; }
    }
}


