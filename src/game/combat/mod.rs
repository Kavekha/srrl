use bevy::prelude::*;

pub mod components;
use crate::{states::GameState, game::combat::components::ActionPoints};

use self::components::EndTurnEvent;

use super::{pieces::components::{Health, Stats}, player::Player};



#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CombatState {
    #[default]
    None, 
    PlayerTurn,
    EnemyTurn
}


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<CombatState>()

            .add_event::<EndTurnEvent>()
            .add_systems(OnEnter(GameState::GameMap), combat_start)
            .add_systems(OnExit(GameState::GameMap), combat_end)

            .add_systems(Update, combat_input.run_if(in_state(GameState::GameMap)))

            .add_systems(Update, entity_end_turn.run_if(on_event::<EndTurnEvent>()))
            ;
    }
}

pub fn combat_start(    
    mut commands: Commands,
    mut combat_state: ResMut<NextState<CombatState>>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>)>,
) {    
    for (fighter_id, fighter_health, fighter_stat, fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 10});
        println!("Action points added for {:?}", fighter_id);
    }
    combat_state.set(CombatState::PlayerTurn);
    println!("Combat start!");
}

pub fn combat_end(
    mut commands: Commands,
    fighters: Query<(Entity, &ActionPoints)>,
){
    for (entity, _fighter) in fighters.iter() {
        commands.entity(entity).remove::<ActionPoints>();
    }
    println!("Combat end!");
}

pub fn entity_end_turn(
    mut ev_endturn: EventReader<EndTurnEvent>,
    mut action_points_q: Query<&mut ActionPoints>
) {
    for event in ev_endturn.iter() {
        if let Ok(mut action_points) =  action_points_q.get_mut(event.entity) {
            consume_actionpoints(&mut action_points, 100);   //TODO : Better way?
            //action_points.current = 0;
            println!("Turn End for {:?}. Action points : {:?}", event.entity, action_points.current);
        }        
    }    
}

pub fn consume_actionpoints(
    actionpoints_component: &mut ActionPoints,
    lost_value: u32,
) {
    actionpoints_component.current = actionpoints_component.current.saturating_sub(lost_value);
}


pub fn combat_input(
    keys: Res<Input<KeyCode>>,
    mut ev_endturn: EventWriter<EndTurnEvent>,  
    player_query: Query<(Entity, With<Player>)>,
){
    if keys.just_pressed(KeyCode::T) {
        if let Ok(result) = player_query.get_single() {
            let entity = result.0;
            ev_endturn.send(EndTurnEvent {entity});
            println!("End of round for {:?}.", entity);
        }
        
    }
}