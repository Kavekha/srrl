use bevy::prelude::*;

use crate::game::{
        gamelog::LogEvent, manager::{game_messages::VictoryMessage, MessageEvent}, tileboard::components::{BoardPosition, ExitMapTile}
    };


use super::components::Player;



//TODO : Pas a être là.
pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let Ok(player_transform) = player_query.get_single() else {return};
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

//TODO : Pas a etre là.
//Le log est envoyé 3 fois car 3 check?
pub fn exit_step_check(
    player_query: Query<&BoardPosition, With<Player>>,
    exit_query: Query<&BoardPosition, With<ExitMapTile>>,
    mut ev_log: EventWriter<LogEvent>,
    mut ev_message: EventWriter<MessageEvent>   //NEW MESSAGE EVENT SYSTEM v0.15.2
){
    let Ok(player_position) = player_query.get_single() else { return };
    for exit_position in exit_query.iter() {
        if player_position.v == exit_position.v {
            println!("Exit !");                  
            ev_log.send(LogEvent {entry: format!("You exit the scene.")});             // Log v0
            ev_message.send(MessageEvent(Box::new(VictoryMessage)));            
        }
    }
}
