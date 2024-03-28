use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game::{pieces::components::Occupier, tileboard::components::{BoardPosition, GameMap, Tile}}, 
    map_builders::{map::Map, random_builder}, vectors::Vector2Int
};


// Communicate informations from Builder : reponse de create_map.
#[derive(Resource, Clone, Default, Debug)]  
pub struct MapInfos{
    pub starting_position: Vector2Int,
    pub spawn_list: Vec<Vector2Int>,
    pub exit_position: Vector2Int
}    

// Créer une Map via le Builder. Retourne les elements necessaires au placement des NPC & etc.

pub fn create_map(world: &mut World) -> MapInfos {
    println!("CreateMapMessage: Building Map.");
        let mut builder = random_builder();
        builder.build_map();        
        builder.build_data.map.populate_blocked(); 

        let mut map_infos = MapInfos{starting_position:Vector2Int{x:0, y:0}, spawn_list:Vec::new(), exit_position:Vector2Int{x:0, y:0}};
        map_infos.starting_position = builder.get_starting_position();
        map_infos.spawn_list = builder.spawn_entities();
        map_infos.exit_position = builder.get_exit_position();
        println!("Generating Map: Player starting position will be {:?}", map_infos.starting_position);

        world.insert_resource(builder.build_data.map.clone());

        println!("Map has been builded");
        return map_infos
}


// Créer les entités necessaires à son affichage, à partir d'une map déja générée.
pub fn spawning_map(world:&mut World, map:&mut Map){
    println!("Let's spawn the map.");
    let mut tiles = HashMap::new();
            let mut tile_entities:Vec::<Entity> = Vec::new();

            //We create logic entities from the map.tiles
            let mut x = 0;
            let mut y = 0;
            for (_idx, tile_info) in map.tiles.iter().enumerate(){
                let v = Vector2Int::new(x, y);
                let mut tile = world.spawn_empty();
                tile.insert(Tile {tiletype: *tile_info}).insert(BoardPosition{v});

                if map.is_blocked(x, y) {
                    tile.insert(Occupier); //TODO : Something else? Occupier is used by Pieces too.
                }
                tiles.insert(v, tile.id()); 
                tile_entities.push(tile.id());
                
                x += 1;
                if x > map.width as i32 - 1 {
                    x = 0;
                    y += 1;
                }
            }    
            let mut game_map = world.spawn_empty();
            game_map.insert(Name::new("Game Map")).insert(GameMap).push_children(&tile_entities);
            map.entity_tiles = tiles; 
    
            println!("Map generated.");
            world.insert_resource(map.clone());
}
