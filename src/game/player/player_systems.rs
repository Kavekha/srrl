use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::{
    engine::{save_load_system::ShouldSave, states::GameState},
    game::{
        combat::events::RefreshActionCostEvent, menus::ingamemenu::InGameMenuState, tileboard::components::{BoardPosition, ExitMapTile}}, 
    };


use super::components::Player;


pub fn player_mouse_input(
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut mouse_move: EventReader<MouseMotion>,
){
    for _event in mouse_move.read() {
        ev_refresh_action.send(RefreshActionCostEvent);
    }
}

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut should_save: ResMut<ShouldSave>,
    mut menu_state: ResMut<NextState<InGameMenuState>>
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Call for In Game Menu.");
        menu_state.set(InGameMenuState::MainMenu);
    }

    /* QUIT GAME, BACK TO MAIN MENU, SAVE 
    if keys.just_pressed(KeyCode::Escape) {
        should_save.to_save = true;
        return;
    } 
    */   
}



pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let Ok(player_transform) = player_query.get_single() else {return};
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;

}

pub fn exit_step_check(
    player_query: Query<&BoardPosition, With<Player>>,
    exit_query: Query<&BoardPosition, With<ExitMapTile>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    let Ok(player_position) = player_query.get_single() else { return };
    for exit_position in exit_query.iter() {
        if player_position.v == exit_position.v {
            println!("Exit !");      
            game_state.set(GameState::VictoryScreen); 
        }
    }
}
