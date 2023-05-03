use bevy::{prelude::*};

use crate::TILE_SIZE;

pub struct AsciiPlugin;

# [derive(Resource)]
pub struct AsciiSheet(Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_ascii);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> Entity {
    assert!(index < 256, "Index out of Ascii range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands 
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
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