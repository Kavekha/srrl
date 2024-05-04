use bevy::prelude::*;

use crate::{
    commons::get_world_position, engine::{animations::events::{AnimationIndices, AnimationTimer}, asset_loaders::GraphicsAssets},
    game::{game_generation::character_creation::components::GameElement, BASE_TIME_FRAME_EFFECT}, globals::ORDER_EFFECT, vectors::Vector2Int
};

struct ParticleRequest {
    id: String,
    position: Vector2Int,
    lifetime: f32
}

#[derive(Resource)]
pub struct ParticleBuilder {
    requests : Vec<ParticleRequest>
}

impl ParticleBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ParticleBuilder {
        ParticleBuilder{ requests : Vec::new() }
    }

    pub fn request(&mut self, id: String, position: Vector2Int, lifetime: f32) {
        self.requests.push(
            ParticleRequest{
                id, position, lifetime
            }
        );
    }
}

// TODO : Prendre en compte le Lifetime.
pub fn particle_spawning(
    mut particle_builder: ResMut<ParticleBuilder>,
    mut commands: Commands,
    graph_assets: ResMut<GraphicsAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for new_particle in particle_builder.requests.iter() {
        let texture = graph_assets.effects[new_particle.id.as_str()].clone();
        let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 3, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 0, last: 2 };

        let (x, y) = get_world_position(&new_particle.position);

        commands.spawn((
            SpriteBundle {
                //transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform {
                    translation: Vec3::new(x, y, ORDER_EFFECT),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                texture,
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(BASE_TIME_FRAME_EFFECT, TimerMode::Repeating)), // Repeating car on passe par autant d'etapes que d'images.
            GameElement
        ));
    }
    particle_builder.requests.clear();
}