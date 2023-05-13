use bevy::{
    prelude::*
};
use::rand::prelude::*;
use pathfinding::prelude::astar;

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    TILE_SIZE, GameState, despawn_screen,
    game::{Player, Stats, TileCollider},
    game::player::{tile_collision_check},
    map_builders::{
        pathfinding::{Position, Successor},
        map::{Map},
    }
};



pub struct NpcPlugin;


impl Plugin for NpcPlugin{
    fn build(&self, app: &mut App) {
        app         
            .add_systems(Update, npc_movement.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, monster_step_check.run_if(in_state(GameState::GameMap)))
            //.add_systems(Update, hostile_ia_decision.run_if(in_state(GameState::GameMap)))        //TODO : Map doit être en resource. REFACTO init Map.
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Npc>)     //TODO : Refacto pour rassembler tout ca dans game?
            ;         
    }
}


#[derive(Component)]
pub struct Npc;


pub fn spawn_npc(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet,
    x: f32,
    y: f32
) {
    let npc = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        2,
        Color::rgb(0.3, 0.9, 0.4),
        Vec3::new(x, y, 900.0), //(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(npc)
        .insert(Npc)
        .insert(Name::new("Npc"))
        .insert(Stats {speed: 6.0});
}


/// IA chasse la cible.
fn hostile_ia_decision(
    map: Res<Map>   //TODO : Map en resources
) {
    // TODO : Pathfinding, work in progress.
    let hostile_pos = Position(10, 10);
    let target_pos = Position(5,5);

    let start = hostile_pos;
    let goal = target_pos;

    let mut path:Vec<Position> = Vec::new();    //Empty. Serie de positions pour se rendre au goal.
    let mut step = 0;   // Le nombre de pas à faire avant d'atteindre le goal.

    let result = astar(
        &start,
        |position| {
            map.get_successors(position)
                .iter()
                .map(|successor| (successor.position, successor.cost))
                .collect::<Vec<_>>()
        },
        |position| position.distance(&goal),
        |position| *position == goal,
    );
    if let Some(result) = result {
        println!("Path: {:?}", result.0);
        println!("Cost: {:?}", result.1);
        path = result.0;
        step = path.len();
    } else {
        println!("No Path Found!");
        path = Vec::new();
        step = 0;
    }

    // STEP:
    //https://github.com/frederickjjoubert/bevy-pathfinding/blob/6fa935f1a1d9fb848455c738b4e2bb41163450f5/src/game.rs#L159


}

fn npc_movement(
    mut npc_query: Query<(&Npc, &mut Transform, &Stats)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Npc>)>,
    time: Res<Time>
) {
    let mut rng = rand::thread_rng();
    for (_npc, mut npc_transform, stats) in npc_query.iter_mut(){
        // Random direction
        let mut x_delta = rng.gen_range(-1.. 2) as f32;
        let mut y_delta = rng.gen_range(-1.. 2) as f32;

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
        }

        let target:Vec3 = npc_transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !wall_query
        .iter()
        .any(|&npc_transform|tile_collision_check(target, npc_transform.translation))
        {
            npc_transform.translation = target;
        }
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
        }
}