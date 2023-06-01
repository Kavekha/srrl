use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces_render;
pub mod components;


use self::{
    tilemap_render::{spawn_map_render},
    pieces_render::{spawn_piece_renderer, update_piece_position},
};

use crate::{
    globals::{TILE_WIDTH_HALF, TILE_HEIGHT_HALF, TILE_HEIGHT_MEDIUM, }, 
    states::GameState,
};


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GraphicsWaitEvent>()

            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)           
            .add_systems(OnEnter(GameState::GameMap), spawn_piece_renderer)
            .add_systems(Update, update_piece_position.run_if(in_state(GameState::GameMap)))
            ;
    }
}


pub struct GraphicsWaitEvent;

pub fn get_world_position(
    x: i32, 
    y: i32
) -> (f32, f32) {
        // REMEMBER : Y in bevy2d = Negative when going down!
        let iso_x = (x - y) * TILE_WIDTH_HALF;
        let iso_y = (x + y) * TILE_HEIGHT_HALF;
        
        (iso_x as f32,
        0.0 - iso_y as f32)     // REMEMBER : Y in bevy2d = Negative when going down!
}

/// z doit être calculé pour les objets à relief du genre mur. Le floor doit rester à 0 par contre.
fn get_world_z(
    x: i32,
    y: i32
) -> f32 {
    let z = (x as f32 / 10.0) + (y as f32 / 5.0);
    z
}



fn get_iso_y_modifier_from_elevation(
    tile_elevation: i32
) -> f32 {
    // On fait -(TAILLE_DE_LA_TILE -STANDARD_TILE) /2  //TODO : Mieux generifier ca, car les Persos doivent l'utiliser aussi.
    // REMEMBER : +Y dans Bevy = descendre. Ici on veut "monter" pour sortir les pieds du sol : On doit aller dans le negatif... :/
    // Original : 0.0 - ((TILE_HEIGHT_HIGHT - TILE_HEIGHT_MEDIUM) / 2) as f32;
    //let modified_y = 0.0 - (((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32);
    //let modified_y = 0.0 - ((tile_elevation - STANDARD_TILE_HEIGHT) as f32);

    /* 
    if tile_elevation > TILE_HEIGHT_MEDIUM {
        let modified_y = 0.0 - ((tile_elevation - TILE_HEIGHT_MEDIUM) as f32);       
        println!("y modifier from elevation is : elevation : {:?} - {:?} = {:?}", tile_elevation, TILE_HEIGHT_MEDIUM, modified_y);     
        return modified_y;
    }
    */    

    // Humain & tile 64 : Si résultat = 16 ou -16 OK. //0.0 - ((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32
    // Tile 48 :  Si resultat +8 OK      //  ((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32
    // Tile 72 : Si resultat +20 OK //((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32        // 72 - 32 = 40 /2 = 20
    // Tile 96 : Si resultat +32 OK //((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32   // 96 - 32 = 64 / 2 = 32
    // Troll 80 & Nain 44 => Formule NOK.
    // 
    //0.0 - ((tile_elevation / 2) - TILE_HEIGHT_MEDIUM) as f32 //
    //0.0 - ((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32  // 64 ok
    ((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32

    // 
}
