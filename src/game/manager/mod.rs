use bevy::prelude::*;

use crate::{game::{pieces::{components::Monster, spawners::{spawn_exit, spawn_npc, spawn_player}}, tileboard::components::BoardPosition}, map_builders::{map::Map, random_builder, BuilderMap}, vectors::Vector2Int};

use super::{pieces::{components::{Health, Melee, Occupier, Piece, Stats}, spawners::get_random_kind}, player::Player, states::GameState};


pub struct ManagerPlugin;
 
impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GameInfos{
            starting_position:Vector2Int { x:0, y: 0 },
            spawn_list: Vec::new()
        })   //Position Renommer 0.15.2
        .add_event::<MessageEvent>()   
        .add_systems(Update, handle_event.run_if(on_event::<MessageEvent>()));
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

pub struct TextMessage{
    pub text: String,
    pub source: String
}

impl Message for TextMessage {
    fn execute(&self, _world: &mut World) {
        println!("{} : {}.", self.source, self.text);
    }
}

pub struct CreatePlayerMessage;
impl Message for CreatePlayerMessage {
    fn execute(&self, world: &mut World) {
        if let Some(game_infos) = world.get_resource::<GameInfos>(){
            let player_starting_position = game_infos.starting_position;
            println!("Player: Starting position = {:?}", player_starting_position);
            let kind = get_random_kind();
            let piece = Piece{kind: kind};

            let mut player = world.spawn_empty();
            
            player
                .insert(piece)
                .insert(Player)
                .insert(Name::new("The Shadowrunner"))
                //TODO : Shadowrun stats
                .insert(Stats {
                    power: 3,         
                    attack: 6,
                    dodge: 6,
                    resilience: 3
                })
                //.insert(Actor::default(),)
                .insert(Health { max: 10, current: 10 })
                .insert(Melee { damage: 1 })
                .insert(BoardPosition{ v:player_starting_position })
                .insert(Occupier);

            world.send_event(MessageEvent(Box::new(TextMessage{source:"CreatePlayerMessage".to_string(), text:"Player has been created".to_string()})));
        } else {
            world.send_event(MessageEvent(Box::new(TextMessage{source:"CreatePlayerMessage".to_string(), text:"Player was NOT created [No Game Infos]".to_string()})));
        };       


    }
}


pub struct StartGameMessage;

impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(MessageEvent(Box::new(CreateMapMessage)));
        world.send_event(MessageEvent(Box::new(CreatePlayerMessage)));
        world.send_event(MessageEvent(Box::new(DisplayMapMessage)));

        /* 
        println!("Self step is {}", self.step);
        let mut send_again = true;
        let mut step = self.step;
        match self.step {
            0 => { world.send_event(MessageEvent(Box::new(CreateMapMessage)));},
            1 => {world.send_event(MessageEvent(Box::new(CreatePlayerMessage)));},
            2 => {world.send_event(MessageEvent(Box::new(DisplayMapMessage)));},
            _ => {send_again = false;}
        } 
        if send_again {
            step += 1;
            world.send_event(MessageEvent(Box::new(StartGameMessage {step: step})));
        }
        */
    }
}

pub struct DisplayMapMessage;
impl Message for DisplayMapMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Prerun);
        }
    }
}


#[derive(Resource, Clone, Default, Debug)]  
pub struct GameInfos{
    pub starting_position: Vector2Int,
    pub spawn_list: Vec<Vector2Int>
}

pub struct CreateMapMessage;
impl Message for CreateMapMessage {
    fn execute(&self, world: &mut World) {
        println!("CreateMapMessage: Building Map.");
        let mut builder = random_builder();
        builder.build_map();        
        builder.build_data.map.populate_blocked(); 

        //get resource Game Infos
        let player_starting_position = builder.get_starting_position();  

        if let Some(game_infos) = world.get_resource::<GameInfos>(){
            let mut new_game_infos = game_infos.clone();
            new_game_infos.starting_position = player_starting_position;
            world.insert_resource(new_game_infos.clone());
            println!("Generating Map: Player starting position will be {:?}", player_starting_position);
        };
        world.insert_resource(builder.build_data.map.clone());

        world.send_event(MessageEvent(Box::new(TextMessage{source:"CreateMapMessage".to_string(), text:"Map has been builded".to_string()})));

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

        // Other entities. //TODO: Can't spawn different npc types: just one.
        /*/
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