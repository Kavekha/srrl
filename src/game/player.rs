use bevy::{
    prelude::*
};

use crate::{
    commons::{tile_collision_check},
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    TILE_SIZE, GameState, despawn_screen,
    game::{Player, Stats, TileCollider, TileExit},    
};



pub struct PlayerPlugin;


impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(GameState::NewGame), character_creation)              
            .add_systems(Update, player_input.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, camera_follow.after(player_input).run_if(in_state(GameState::GameMap)))
            .add_systems(Update, player_step_check.run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Player>);  
    }
}


pub fn spawn_player(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet,
    x: f32,
    y: f32
) -> Entity {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(x, y, 900.0), //(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(player)
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Stats {speed: 6.0});

    player
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
    if keys.any_pressed([KeyCode::Up, KeyCode::Z]) {
        y_delta += stats.speed * TILE_SIZE * time.delta_seconds(); 
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        y_delta -= stats.speed * TILE_SIZE * time.delta_seconds(); 
    }

    let mut x_delta: f32 = 0.0;
    if keys.any_pressed([KeyCode::Right, KeyCode::D]){
        x_delta += stats.speed * TILE_SIZE * time.delta_seconds(); 
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::Q]){
        x_delta -= stats.speed * TILE_SIZE * time.delta_seconds(); 
    }

    // We check if collision with TileColider for x and y (If both at the same time, we'll block a valid movement if x: True & y : False)
    // TODO: how to avoid duplicate?
    let target: Vec3 = transform.translation + Vec3::new(x_delta, 0.0, 0.0);

    if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target, transform.translation))
        {
            transform.translation = target;
        }

    let target: Vec3 = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    
    if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target, transform.translation))
        {
            transform.translation = target;
        }
}

fn player_step_check(
    player_query: Query<(&Player, &mut Transform)>,
    exit_query: Query<&Transform, (With<TileExit>, Without<Player>)>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // If player on collision with an exit...
    let (_player, player_transform) = player_query.single();
    if exit_query
        .iter()
        .any(|&transform|tile_collision_check(player_transform.translation, transform.translation))
        {
            println!("Exit !");      //TOLOG   
            game_state.set(GameState::VictoryScreen); 
        }
}
