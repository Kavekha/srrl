use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game::{manager::{MessageEvent, TextMessage}, pieces::components::Occupier, states::GameState, tileboard::components::{BoardPosition, GameMap, Tile}}, map_builders::{map::Map, random_builder}, vectors::Vector2Int
};


// Communicate informations from Builder : reponse de create_map.
#[derive(Resource, Clone, Default, Debug)]  
pub struct GameInfos{
    pub starting_position: Vector2Int,
    pub spawn_list: Vec<Vector2Int>,
    pub exit_position: Vector2Int
}    

// Créer une Map via le Builder. Retourne les elements necessaires au placement des NPC & etc.

pub fn create_map(world: &mut World) -> GameInfos {
    println!("CreateMapMessage: Building Map.");
        let mut builder = random_builder();
        builder.build_map();        
        builder.build_data.map.populate_blocked(); 

        let mut game_infos = GameInfos{starting_position:Vector2Int{x:0, y:0}, spawn_list:Vec::new(), exit_position:Vector2Int{x:0, y:0}};
        game_infos.starting_position = builder.get_starting_position();
        game_infos.spawn_list = builder.spawn_entities();
        game_infos.exit_position = builder.get_exit_position();
        println!("Generating Map: Player starting position will be {:?}", game_infos.starting_position);

        world.insert_resource(builder.build_data.map.clone());

        world.send_event(MessageEvent(Box::new(TextMessage{source:"CreateMapMessage".to_string(), text:"Map has been builded".to_string()})));
        return game_infos
}


// Créer les entités necessaires à son affichage, à partir d'une map déja générée.
pub fn spawn_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut game_state: ResMut<NextState<GameState>>
) {
    println!("Map generation begins...");

    let mut tiles = HashMap::new();
    let mut tile_entities:Vec::<Entity> = Vec::new();

    //We create logic entities from the map.tiles
    let mut x = 0;
    let mut y = 0;
    for (_idx, tile_info) in map.tiles.iter().enumerate(){
        let v = Vector2Int::new(x, y);
        let tile = commands.spawn((
            Tile {tiletype: *tile_info},
            BoardPosition{v}
        ))
        .id();

        if map.is_blocked(x, y) {
            commands.entity(tile).insert(Occupier); //TODO : Something else? Occupier is used by Pieces too.
        }
        tiles.insert(v, tile); 
        tile_entities.push(tile);
          
        x += 1;
        if x > map.width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }    
    
    commands
    .spawn(Name::new("Game Map"))
    .insert(GameMap)
    .push_children(&tile_entities)
    ;

    map.entity_tiles = tiles; 

    println!("Map generated.");

    game_state.set(GameState::GameMap); //TODO : Pas a ce systeme de gerer les changements de state.
}
