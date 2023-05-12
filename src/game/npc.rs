use bevy::{
    prelude::*,
    sprite::collide_aabb::collide
};
use::rand::prelude::*;

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    TILE_SIZE, GameState, despawn_screen,
    game::{Player, Stats, TileCollider, TileExit},
    game::player::{tile_collision_check}
};



pub struct NpcPlugin;


impl Plugin for NpcPlugin{
    fn build(&self, app: &mut App) {
        app         
            .add_systems(Update, npc_movement.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, monster_step_check.run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Npc>);     //TODO : Refacto pour rassembler tout ca dans game?
            ;         
    }
}


#[derive(Component)]
pub struct Npc;


pub fn spawn_npc(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet
) {
    let npc = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        2,
        Color::rgb(0.3, 0.9, 0.4),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(npc)
        .insert(Npc)
        .insert(Name::new("Npc"))
        .insert(Stats {speed: 6.0});
}


fn npc_movement(
    mut npc_query: Query<(&Npc, &mut Transform, &Stats)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Npc>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    println!("Update: NPC movement");

    let mut rng = rand::thread_rng();
    for (_npc, mut npc_transform, stats) in npc_query.iter_mut(){
        // Random direction
        let mut x_delta = rng.gen_range(-1.. 2) as f32;
        let mut y_delta = rng.gen_range(-1.. 2) as f32;
        println!("NPC veut se rendre à {},{}", x_delta, y_delta);

        // How much will they move
        x_delta *= stats.speed * TILE_SIZE * time.delta_seconds();
        y_delta *= stats.speed * TILE_SIZE * time.delta_seconds();

        // Can they move to it?
        let target:Vec3 = npc_transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if !wall_query
        .iter()
        .any(|&npc_transform|tile_collision_check(target, npc_transform.translation))
        {
            npc_transform.translation = target;
            println!("NPC nouvelle position X est {}", target)
        }
        println!("NPC n'a pas changé de position X.");

        let target:Vec3 = npc_transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !wall_query
        .iter()
        .any(|&npc_transform|tile_collision_check(target, npc_transform.translation))
        {
            npc_transform.translation = target;
            println!("NPC nouvelle position Y est {}", target)
        }
        println!("NPC n'a pas changé de position Y.");

    }
}

fn monster_step_check(
    player_query: Query<(&Player, &mut Transform)>,
    npc_query: Query<&Transform, (With<Npc>, Without<Player>)>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // If player on collision with a ghoul...
    let (_player, player_transform) = player_query.single();
    if npc_query
        .iter()
        .any(|&transform|tile_collision_check(player_transform.translation, transform.translation))
        {
            println!("Eaten !");      //TOLOG   
            game_state.set(GameState::GameOverScreen);
            println!("Game State is now: {:?}", game_state);
        }
}