use bevy::prelude::*;

use crate::{
    globals::CHAR_SIZE, menus::NineSlice, render::components::AsciiText, asset_loaders::GraphicsAssets,
};


#[derive(Resource, Copy, Clone)]
pub struct NineSliceIndices {
    pub center: usize,
    pub upper_left_index: usize,
    pub upper_right_index: usize,
    pub lower_left_index: usize,
    pub lower_right_index: usize,
    pub horizontal_index: usize,
    pub vertical_index: usize
}



pub fn spawn_nine_slice(
    commands: &mut Commands,
    ascii: &GraphicsAssets,
    indices: &NineSliceIndices,
    width: f32,
    height: f32
) -> Entity {
    assert!(width >= 2.0);
    assert!(height >= 2.0);

    let color = Color::rgb(0.3, 0.3, 0.9);
    let mut sprites = Vec::new();

    let left = (-width / 2.0 + 0.5) * CHAR_SIZE;
    let right = (width / 2.0 - 0.5) * CHAR_SIZE;
    let up = (height/2.0 - 0.5) * CHAR_SIZE;
    let down = (-height/2.0 + 0.5) * CHAR_SIZE;


    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.center,
        color,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(width -2.0, height -2.0, 0.0)
    ));

    //upper left
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.upper_left_index,
        color,
        Vec3::new(left, up, 0.0),
        Vec3::splat(1.0)
    ));
    // Vertical left 
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.vertical_index,
        color,
        Vec3::new(left, 0.0, 0.0),
        Vec3::new(1.0, height -2.0, 1.0)
    ));
    // Lower left
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.lower_left_index,
        color,
        Vec3::new(left, down, 0.0),
        Vec3::splat(1.0)
    ));
    //horizontal down
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.horizontal_index,
        color,
        Vec3::new(0.0, down, 0.0),
        Vec3::new(width - 2.0, 1.0, 0.0)
    ));
    //horizontal up
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.horizontal_index,
        color,
        Vec3::new(0.0, up, 0.0),
        Vec3::new(width - 2.0, 1.0, 0.0)
    ));
    //upper right 
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.upper_right_index,
        color,
        Vec3::new(right, up, 0.0),
        Vec3::splat(1.0)
    ));
    // Lower right 
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.lower_right_index,
        color,
        Vec3::new(right, down, 0.0),
        Vec3::splat(1.0)
    ));
    // Vertical right 
    sprites.push(spawn_ascii_sprite(
        commands,
        ascii,
        indices.vertical_index,
        color,
        Vec3::new(right, 0.0, 0.0),
        Vec3::new(1.0, height -2.0, 1.0)
    ));

    commands
        .spawn(NineSlice)
        .insert(Name::new("Nine Slice Box"))
        .insert(SpatialBundle{
            ..default()
        })
        .push_children(&sprites)
        .id()

}

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: &GraphicsAssets,
    to_print: &str,
    left_center: Vec3
) -> Entity {
    let color = Color::rgb(0.8, 0.8, 0.8);

    let mut character_sprites = Vec::new();
    for (i, char) in to_print.chars().enumerate(){
        assert!(char as usize <= 256);
        character_sprites.push(spawn_ascii_sprite(
            commands,
            ascii, 
            char as usize, 
            color,
            Vec3::new(i as f32 * CHAR_SIZE, 0.0, 0.0),
            Vec3::splat(1.0)));
    }
    commands
    .spawn(Name::new(format!("text: {}", to_print)))
    .insert(AsciiText)
    .insert(SpatialBundle{
        transform: Transform::from_translation(left_center),
        ..default()
    })
    .push_children(&character_sprites)
    .id()
}


// https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#texture-atlas-rework
pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    assets: &GraphicsAssets,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3
) -> Entity {
    assert!(index < 256, "Index out of Ascii range");

     // DESACTIVATE 0.13. TO CHECK / FIX
     /* 
    let mut sprite = TextureAtlas::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(CHAR_SIZE));   
    */

    commands 
        .spawn(SpriteSheetBundle {
            sprite: default(), //sprite
            atlas: TextureAtlas {
                layout: assets.ascii_sheet_layout.clone(),
                index: 0
            },            
            texture: assets.ascii_sheet_img.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..default()
            },
            ..default()
        })
        .id()
}
