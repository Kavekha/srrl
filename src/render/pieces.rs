use bevy::prelude::*;
use rand::Rng;

use crate::{
    globals::{SPRITE_GHOUL, SPRITE_PLAYER, POSITION_TOLERANCE, SPEED_MULTIPLIER, TILE_HEIGHT_HIGH, BASE_SPEED, SPRITE_PLAYER_HUMAN, SPRITE_PLAYER_ORC, SPRITE_PLAYER_TROLL, SPRITE_PLAYER_DWARF, TILE_HEIGHT_MEDIUM, TILE_HEIGHT_MEDIUM_HIGH, TILE_HEIGHT_VERY_HIGH, SPRITE_PLAYER_ELF, SIZE_DWARF, SIZE_ORC, SIZE_TROLL, SIZE_ELF, SIZE_HUMAN, SIZE_GHOUL,},
    game::{GridPosition, player::{Player, Stats}, pieces::components::Piece}, GraphicsWaitEvent};

use super::{get_world_position, get_world_z, get_iso_y_modifier_from_elevation};


pub fn update_piece_position(
    mut query: Query<(&GridPosition, &mut Transform, &Piece)>,   //, With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<GraphicsWaitEvent>
){
    let mut animating = false;

    for (grid_position, mut transform, piece) in query.iter_mut(){
        let (position_x, mut position_y) = get_world_position(grid_position.x, grid_position.y);

        // On fait -(TAILLE_DE_LA_TILE -STANDARD_TILE) /2  //TODO : Mieux generifier ca, car les Persos doivent l'utiliser aussi.
        // REMEMBER : +Y dans Bevy = descendre. Ici on veut "monter" pour sortir les pieds du sol : On doit aller dans le negatif... :/
        position_y += get_iso_y_modifier_from_elevation(piece.size); //(TILE_HEIGHT_HIGHT - TILE_HEIGHT_MEDIUM) / 2) as f32;

        let target = Vec3::new(position_x, position_y, get_world_z(grid_position.x, grid_position.y));
        let destination = (target - transform.translation).length();
  
        
        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                BASE_SPEED * SPEED_MULTIPLIER * time.delta_seconds()
            );
            animating = true;
        }
        if animating {
            //TODO: Currently: One wait by Actor, so a lot of wait.
            //ev_wait.send(GraphicsWaitEvent);
        }
    }
}


pub fn spawn_piece_renderer(
    mut commands: Commands,
    mut query: Query<(Entity, &GridPosition, &mut Piece, Option<&Player>)>,
    asset_server: Res<AssetServer>
) {
    println!("Rendering Pieces begins..."); 
    // On ajoute aux entités de nouveaux components.
    for (entity, grid_position, mut piece, player) in query.iter_mut() {

        let (x, mut y) = get_world_position(grid_position.x, grid_position.y);
        let z = get_world_z(grid_position.x, grid_position.y);

        let mut texture = SPRITE_PLAYER;    //DEFAULT //TODO

        if let Some(_player) = player {
            let (player_texture, y_modified) = get_player_render(grid_position.x, grid_position.y);
            texture = player_texture;
            //y += y_modified;            
        } else {
            // NPC Apparence
            let texture  = SPRITE_GHOUL;    //TODO : Plus de flexibilité pour changer les mobs.
            //y += get_iso_y_modifier_from_elevation(SIZE_GHOUL);   //((TILE_HEIGHT_HIGHT - TILE_HEIGHT_MEDIUM) / 2) as f32;
        }

        //TODO REFACTO : doit être fait au niveau du personnage créé, pas dans le rendu.
        match texture {
            SPRITE_PLAYER_DWARF => {piece.size = SIZE_DWARF}
            SPRITE_PLAYER_ORC => {piece.size = SIZE_ORC}
            SPRITE_PLAYER_TROLL => {piece.size = SIZE_TROLL}
            SPRITE_PLAYER_ELF => {piece.size = SIZE_ELF}
            SPRITE_PLAYER_HUMAN => {piece.size = SIZE_HUMAN}
            _ => {piece.size = TILE_HEIGHT_HIGH}
        }
        y += get_iso_y_modifier_from_elevation(piece.size);

        commands.entity(entity)
            .insert(SpriteBundle {
                texture: asset_server.load(texture),    
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            });
        
            if let Some(_player) = player {
                println!("INFO: player rendered.");
            }
        
    }
    println!("Pieces rendered.");
}


/// TEMP : //TODO : Necessaire pour renvoyer la bonne info dans update position. La taille doit être dans un composant p-e?

/// TEMP : Renvoie infos rendus pour les differentes races jouables par le PJ.
pub fn get_player_render(
    x: i32,
    y: i32
) -> (&'static str, f32) {
    let mut texture = SPRITE_PLAYER;
    let mut y_modified = get_iso_y_modifier_from_elevation(TILE_HEIGHT_MEDIUM);

    let mut rng = rand::thread_rng();
    let rand = 2;   //rng.gen_range(0..5);
    match rand {
        0 => {
            //Nain
            texture = SPRITE_PLAYER_DWARF;
            let y_modified = get_iso_y_modifier_from_elevation(SIZE_DWARF); 
            println!("Player is : a Dwarf.");
        }
        1 => {
            // Orc
            texture = SPRITE_PLAYER_ORC;
            let y_modified = 24;//get_iso_y_modifier_from_elevation(SIZE_ORC);  
            println!("Player is : an Orc.");  
        }
        2 => {
            // Troll
            texture = SPRITE_PLAYER_TROLL;
            let y_modified = get_iso_y_modifier_from_elevation(SIZE_TROLL);    
            println!("Player is : a Troll.");
        }
        3 => {
            // Elf
            texture = SPRITE_PLAYER_ELF;
            let y_modified = get_iso_y_modifier_from_elevation(SIZE_ELF);  
            println!("Player is : an Orc.");  
        }
        _ => {
            // Humain
            texture = SPRITE_PLAYER_HUMAN;
            let y_modified = get_iso_y_modifier_from_elevation(SIZE_HUMAN);   
            println!("Player is : a Human.");             
        }
    }
    (texture, y_modified)   //RETURN

}