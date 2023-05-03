use bevy::{
    prelude::*,
    sprite::collide_aabb::collide
};

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    TILE_SIZE,
    tilemap::{TileCollider},
};


pub struct PlayerPlugin;



#[derive(Component)]
struct Player;


#[derive(Component)]
struct Stats {
    speed: f32
}

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_input)
            .add_systems(Update, camera_follow.after(player_input));
    }
}


fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0)
    );

    commands 
        .entity(player)
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Stats {speed: 3.0});
}


fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;

}


fn player_input(
    mut player_query: Query<(&Player, &mut Transform, &Stats)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (_player, mut transform, stats) = player_query.single_mut();

    let mut y_delta: f32 = 0.0;
    if keys.pressed(KeyCode::Up) {
        y_delta += stats.speed * TILE_SIZE * time.delta_seconds(); //* stats.speed 
    }
    if keys.pressed(KeyCode::Down) {
        y_delta -= stats.speed * TILE_SIZE * time.delta_seconds(); //* stats.speed 
    }

    let mut x_delta: f32 = 0.0;
    if keys.pressed(KeyCode::Right){
        x_delta += stats.speed * TILE_SIZE * time.delta_seconds(); //* stats.speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Left){
        x_delta -= stats.speed * TILE_SIZE * time.delta_seconds(); //* stats.speed * time.delta_seconds();
    }

    let target: Vec3 = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }
}


fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)> 
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE)
        );
        if collision.is_some(){
            return false;
        }
    }
    true
}

