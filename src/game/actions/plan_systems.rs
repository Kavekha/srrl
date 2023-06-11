use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    game::{pieces::components::{Actor, Walk, Melee, Occupier, PathTo}, player::Player, tileboard::components::BoardPosition}, 
    map_builders::{map::Map}, 
    globals::{NPC_MOVE_SCORE_BONUS, NPC_MOVE_SCORE_DEFAULT, NPC_ATTACK_SCORE_DEFAULT}, vectors::{MULTI_DIRECTIONS, find_path}};

use super::{ActorQueue, models::{MeleeHitAction, WalkAction}};



pub fn pathfinding_walk(
    mut query: Query<(&mut Actor, &mut PathTo)>,
    queue: Res<ActorQueue>,
){
    println!("Pathfinding walk is running.");
    // On prends la premiere entity de la queue.
    let Some(entity) = queue.0.get(0) else { return };

    // Fait-il parti des Actors avec Grid position?
    let Ok((mut actor, path)) = query.get_mut(*entity) else { 
        println!("{:?} : Je n'ai pas de path", entity);
        return };

    println!("Je suis {:?} et j'ai un PathTo pour me deplacer!", entity)

}


pub fn plan_walk(
    mut query: Query<(&BoardPosition, &mut Actor), With<Walk>>,
    queue: Res<ActorQueue>,
    occupier_query: Query<&BoardPosition, With<Occupier>>, //Will return None for now, since no char have it.
    player_query: Query<&BoardPosition, With<Player>>,
    map: Res<Map>,
) {
    // On prends la premiere entity de la queue.
    let Some(entity) = queue.0.get(0) else { return };

    // Fait-il parti des Actors avec Grid position?
    let Ok((position, mut actor)) = query.get_mut(*entity) else { return };

    // On veut connaitre le joueur pour en faire notre Goal. Si y en a pas, on s'en va.
    let Ok(player_position) = player_query.get_single() else { return };


    // get all possible move targets
    let positions = MULTI_DIRECTIONS.iter().map(|direction| *direction + position.v).collect::<Vec<_>>();

    // find possible path to the player
    let path_to_player = find_path(
        position.v,
        player_position.v,
        &map.entity_tiles.keys().cloned().collect(),
        &occupier_query.iter().map(|p| p.v).collect()
    );    
    
    let mut rng = thread_rng();
    // positions.iter().map { Pour chaque element iteré qui corresponds à une position autour, on attribue une valeur -10 à 0, et si cette position est sur le Path, on lui donne NPC_MOVE_SCORE_BONUS}
    let actions = positions.iter()
        .map(|some_position_around | {
            // randomize movement choices
            let mut random_action_value = rng.gen_range(-15..0);
            if let Some(path) = &path_to_player {
                // however prioritize a movement if it leads to the player                
                if path.contains(some_position_around) { random_action_value += NPC_MOVE_SCORE_BONUS }
            }
            (Box::new(WalkAction(*entity, *some_position_around)) as Box<dyn super::Action>, NPC_MOVE_SCORE_DEFAULT + random_action_value)
        })
        .collect::<Vec<_>>();
    actor.0.extend(actions);
}


pub fn plan_melee(
    mut query: Query<(&mut Actor, &Melee)>,
    player_query: Query<&BoardPosition, With<Player>>,
    queue: Res<ActorQueue>
) {
    //println!("Plan melee!");
    let Some(entity) = queue.0.get(0) else { return };
    let Ok((mut actor, _melee)) = query.get_mut(*entity) else { return };
    let Ok(player_position) = player_query.get_single() else { return };
    //println!("Plan Melee: Player is at : {:?}", player_position.v);
    let action = Box::new(MeleeHitAction{
        attacker: *entity,
        target: player_position.v,
    });
    actor.0.push((action, NPC_ATTACK_SCORE_DEFAULT))
}