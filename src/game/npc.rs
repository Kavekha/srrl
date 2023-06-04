use bevy::{
    prelude::*
};

use crate::{
    despawn_screen,
    commons::tile_collision_check,
    states::GameState
};

use super::{player::{Player, Npc, Monster}};



pub struct NpcPlugin;


//DEPRECATED : Dans Actions & Plans maintenant.
impl Plugin for NpcPlugin{
    fn build(&self, app: &mut App) {
        app         
            //.add_systems(Update, monster_step_check.run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Npc>)     //TODO : Refacto pour rassembler tout ca dans game?     
            ;         
    }
}

fn monster_step_check(
    player_query: Query<(&Player, &mut Transform)>,
    npc_query: Query<&Transform, (With<Monster>, Without<Player>)>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // If player on collision with a ghoul...
    let (_player, player_transform) = player_query.single();
    if npc_query
        .iter()
        .any(|&transform|tile_collision_check(player_transform.translation, transform.translation))
        {
            //println!("Eaten !");      //TOLOG   
            game_state.set(GameState::GameOverScreen);
        }
}