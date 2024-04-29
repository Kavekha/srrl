use bevy::prelude::*;

use crate::{
        commons::get_world_position, engine::asset_loaders::GraphicsAssets, game::{
        game_generation::character_creation::components::Piece, player::Player, tileboard::components::{BoardPosition, ExitMapTile}
    }, globals::{
        MAP_EXIT, ORDER_EXIT, ORDER_NPC, ORDER_PLAYER}
};



pub fn spawn_exit_render(
    mut commands: Commands,
    query: Query<(Entity, &BoardPosition), With<ExitMapTile>>,
    assets: Res<GraphicsAssets>
){
    println!("Rendering Exit begins...");
    for (entity, position) in query.iter() {
        let translation = get_world_position(&position.v);
        let texture = assets.map_items[MAP_EXIT].clone();

        commands.entity(entity)
            .insert(SpriteBundle {
                texture: texture,    
                transform: Transform {
                    translation: Vec3::new(translation.0, translation.1, ORDER_EXIT),  
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            });
        println!("An Exit has been rendered.");
    }
    println!("Rendering Exit ends.");
}


pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &BoardPosition, &mut Piece, Option<&Player>)>,
    assets: Res<GraphicsAssets>,
) {
    println!("Rendering Pieces begins..."); 
    // On ajoute aux entités de nouveaux components.
    for (entity, position, piece, player) in query.iter() {
        let translation= get_world_position(&position.v);   //TODO : get world position retourne un Vector2Int
        //let texture = assets.textures[get_texture_from_kind(piece.kind)].clone();
        let texture = assets.textures[piece.model.as_str()].clone();
        let mut order_z = ORDER_NPC;

        // 0.20k
        let mut visibility = Visibility::Hidden;// Par defaut on cache tout. v0.20k
        if player.is_some() {
            visibility= Visibility::Visible;
            println!("player order layer: {:?}.", entity);
            order_z = ORDER_PLAYER;
        }

        commands.entity(entity)
            .insert(SpriteBundle {
                texture: texture, //asset_server.load(texture),    
                transform: Transform {
                    translation: Vec3::new(translation.0, translation.1, order_z),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                visibility: visibility,
                ..default()
            });
        
        if player.is_some() {
            println!("player rendered.");
        }        
    }
    println!("Pieces rendered.");
}


pub fn spawn_sprite_render(
    commands: &mut Commands,
    texture: &Handle<Image>,
    x: f32,
    y: f32,
    z: f32,
) -> Entity {

    let sprite = commands.spawn(SpriteBundle {
        texture: texture.clone(), 
        transform: Transform {
            translation: Vec3::new(x, y, z),
            scale: Vec3::splat(1.0),             
            ..default()
        },  
        sprite: Sprite { 
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        },
        visibility: Visibility::Hidden, // Par defaut on cache tout. v0.20b
        ..default()
    }).id();

    sprite
}
