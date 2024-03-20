use bevy::prelude::*;

use crate::{game::{pieces::{components::Monster, spawners::{spawn_exit, spawn_npc, spawn_player}}, tileboard::components::BoardPosition}, map_builders::random_builder};


pub struct ManagerPlugin;
 
impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<MessageEvent>()   
        .add_systems(Update, handle_event.run_if(on_event::<MessageEvent>()));
        ;
    }
}

#[derive(Event)]
pub struct MessageEvent(pub Box<dyn Message>);

fn handle_event(
    world: &mut World
) {
    let events = if let Some(mut res) = world.get_resource_mut::<Events<MessageEvent>>() {
        res.drain().collect::<Vec<_>>()
    } else { return };
    for ev in events {
       ev.0.execute(world);
    }
}

pub trait Message: Send + Sync {
    fn execute(&self, world: &mut World);
}

pub struct TextMessage;
impl Message for TextMessage {
    fn execute(&self, world: &mut World) {
        println!("Message sent.");
    }
}

pub struct StartGameMessage;
impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        println!("Message StartGame : Building Map.");
        let mut builder = random_builder();
        builder.build_map();
        world.insert_resource(builder.build_data.map.clone());

        world.send_event(MessageEvent(Box::new(TextMessage)));
    }
        /* TODO 
        if SHOW_MAPGEN_VISUALIZER {
            let mapgen_history = MapGenHistory{
                history: builder.build_data.history.clone(),
                index: 0,
            };
            commands.insert_resource(mapgen_history);
        }
        */

        /* 
        let player = spawn_player(&mut commands);        
        let player_starting_position = builder.get_starting_position();    
        println!("Player: Starting position = {:?}", player_starting_position);
        commands
            .entity(player)
            .insert(BoardPosition{ v:player_starting_position })
        ;

        // Other entities. //TODO: Can't spawn different npc types: just one.
        let entities_pos = builder.spawn_entities();
        for entity_position in entities_pos {
            println!("NPC: Starting position = {:?}", entity_position);
            let npc = spawn_npc(&mut commands);
            //TODO : Le nom pour le moment est dans le spawner.
            commands
            .entity(npc)
            .insert(BoardPosition{ v:entity_position})
            .insert(Monster)
            ;
        }
        */

        /* 
        // EXIT 
        let exit_position = builder.get_exit_position();
        let exit = spawn_exit(&mut commands);
        commands.entity(exit).insert(BoardPosition{ v:exit_position});
                
        builder.build_data.map.populate_blocked(); 

        commands.insert_resource(builder.build_data.map.clone());
*/
        /* OLD CODE.
        if !SHOW_MAPGEN_VISUALIZER {
            game_state.set(GameState::Prerun);  
        } else {
            game_state.set(GameState::MapGeneration);  
        }       
        */
}






/* NO USAGE AT ALL... OLD CODE... 
pub struct CombatManagerPlugin;
 
impl Plugin for CombatManagerPlugin {
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
*/