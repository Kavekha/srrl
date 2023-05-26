use bevy::{prelude::*};

use crate::{
    globals::{CHAR_SIZE},
    ecs_elements::components::{AsciiText}, menus::{AsciiSheet, NineSlice}
};

pub struct AsciiPlugin;




impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_ascii)
            .insert_resource(NineSliceIndices{
                center: 2 * 16,
                upper_left_index: 13 * 16 + 10,
                upper_right_index: 11 * 16 + 15,
                lower_left_index: 12 * 16,
                lower_right_index: 13 * 16 + 9,
                horizontal_index: 12 * 16 + 4,
                vertical_index: 11 * 16 + 3,
            });

    }
}

#[derive(Resource, Copy, Clone)]
pub struct NineSliceIndices {
    center: usize,
    upper_left_index: usize,
    upper_right_index: usize,
    lower_left_index: usize,
    lower_right_index: usize,
    horizontal_index: usize,
    vertical_index: usize
}



pub fn spawn_nine_slice(
    commands: &mut Commands,
    ascii: &AsciiSheet,
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
    ascii: &AsciiSheet,
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


pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3
) -> Entity {
    assert!(index < 256, "Index out of Ascii range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(CHAR_SIZE));

    commands 
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..default()
            },
            ..default()
        })
        .id()
}


fn load_ascii(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>)
    {
        let image = asset_server.load("Ascii.png");        
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::splat(9.0),
            16,
            16,
            Some(Vec2::splat(2.0)),
            None);
        let atlas_handle = texture_atlases.add(atlas);

        commands.insert_resource(AsciiSheet(atlas_handle));
    }