use bevy::prelude::*;

use crate::states::GameState;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<TurnState>()
            
            .add_event::<TickEvent>()

            .add_systems(OnEnter(GameState::GameMap), game_start)
            .add_systems(OnExit(GameState::GameMap), game_end)
            .add_systems(Update, turn_update_start.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, tick.run_if(in_state(TurnState::TurnUpdate)))
            .add_systems(Update, process.run_if(on_event::<TickEvent>()))
        ;
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TurnState{
    #[default]
    None,
    PlayerInput,
    TurnUpdate
}

#[derive(Event)]
pub struct TickEvent;


fn game_start(
    mut turn_state: ResMut<NextState<TurnState>>
){
    println!("--THE GAME START--");
    turn_state.set(TurnState::PlayerInput);
}

fn game_end(
    mut turn_state: ResMut<NextState<TurnState>>
){
    println!("-- THE GAME END --");
    turn_state.set(TurnState::None);
}

fn turn_update_start(
    mut turn_state: ResMut<NextState<TurnState>>,
    mut ev_tick: EventWriter<TickEvent>
){
    println!("-- NEW ITERATION START --");
    println!("Send tick...");
    ev_tick.send(TickEvent);
    turn_state.set(TurnState::TurnUpdate);
}

fn tick(
    mut ev_tick: EventWriter<TickEvent>
){
    println!("Tick!");
    ev_tick.send(TickEvent);
}

fn process(){
    println!("Processing....");
}