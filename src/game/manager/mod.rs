use bevy::prelude::*;

use crate::{game::{pieces::{components::Monster, spawners::{spawn_exit, spawn_npc, spawn_player, Kind}}, tileboard::components::BoardPosition}, map_builders::{map::Map, random_builder, BuilderMap}, vectors::Vector2Int};

use super::{pieces::{components::{Health, Melee, Npc, Occupier, Piece, Stats, Walk}, spawners::get_random_kind}, player::Player, states::GameState};


pub struct ManagerPlugin;
 
impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
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

pub struct StartGameMessage;

impl Message for StartGameMessage {
    fn execute(&self, world: &mut World) {
        let game_infos = create_map(world);
        create_player(world, game_infos.starting_position);
        spawn_npcs(world, game_infos.spawn_list);
        world.send_event(MessageEvent(Box::new(DisplayMapMessage)));
    }
}

fn create_map(world: &mut World) -> GameInfos {
    println!("CreateMapMessage: Building Map.");
        let mut builder = random_builder();
        builder.build_map();        
        builder.build_data.map.populate_blocked(); 

        //get resource Game Infos
        let player_starting_position = builder.get_starting_position();  
        let mut game_infos = GameInfos{starting_position:Vector2Int{x:0, y:0}, spawn_list:Vec::new()};
        game_infos.starting_position = player_starting_position;
        game_infos.spawn_list = builder.spawn_entities();
        println!("Generating Map: Player starting position will be {:?}", player_starting_position);
        //};
        world.insert_resource(builder.build_data.map.clone());

        world.send_event(MessageEvent(Box::new(TextMessage{source:"CreateMapMessage".to_string(), text:"Map has been builded".to_string()})));
        return game_infos
}

fn create_player(world: &mut World, player_starting_position: Vector2Int){
    //if let Some(game_infos) = world.get_resource::<GameInfos>(){
        //let player_starting_position = game_infos.starting_position;
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
    //}
}

fn spawn_npcs(world: &mut World, entities_pos: Vec<Vector2Int>){
    for entity_position in entities_pos {
        println!("NPC: Starting position = {:?}", entity_position);
        spawn_mob(world, entity_position);
    }
}

fn spawn_mob(world: &mut World, npc_spawning_position: Vector2Int
) {
        let mut npc = world.spawn_empty();
        
        npc
        .insert(Name::new(format!("Ghoul")))
        .insert(Piece{kind: Kind::Ghoul })
        .insert(Stats {
            power: 4,         
            attack: 4,
            dodge: 3,
            resilience: 4
        })
        //.insert(Actor::default(),)
        .insert(Npc)
        .insert(Monster)
        .insert(Walk)
        .insert(Melee { damage: 2 })
        .insert(Health { max: 10, current: 10 })
        .insert(BoardPosition{ v:npc_spawning_position })
        .insert(Occupier);
    println!("Npc created");
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
     