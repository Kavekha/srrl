use bevy::prelude::*;
use pathfinding::prelude::astar;
use rand::{thread_rng, Rng};

use crate::{game::{GridPosition, pieces::components::{Actor, Walk}, player::Player}, map_builders::{map::Map, pathfinding::Position}, globals::{ORTHO_DIRECTIONS, NPC_MOVE_SCORE_BONUS, NPC_MOVE_SCORE_DEFAULT, DEFAULT_COST_PATHFINDING}};

use super::{ActorQueue, WalkAction};

pub fn plan_walk(
    mut query: Query<(&GridPosition, &mut Actor), With<Walk>>,
    queue: Res<ActorQueue>,
    player_query: Query<&GridPosition, With<Player>>,
    //occupier_query: Query<&GridPosition, With<Occupier>>, //Will return None for now, since no char have it.
    map: Res<Map>,
) {
    // On prends la premiere entity de la queue.
    let Some(entity) = queue.0.get(0) else { return };

    // Fait-il parti des Actors avec Grid position?
    let Ok((gridposition, mut actor)) = query.get_mut(*entity) else { return };
    let start = Position(gridposition.x, gridposition.y);

    // On veut connaitre le joueur pour en faire notre Goal. Si y en a pas, on s'en va.
    let Ok(player_position) = player_query.get_single() else { return };
    let goal = Position(player_position.x, player_position.y);

    // get all possible move targets
    let positions = ORTHO_DIRECTIONS.iter().map(|direction| Position(direction.0 + gridposition.x, direction.1 + gridposition.y)).collect::<Vec<_>>();

    // find possible path to the player
    let result = astar(
        &start,
        |some_position| {
            map.get_successors(some_position)
                .iter()
                .map(|successor| (successor.position, successor.cost))  //TODO : Pathfinding crate force me to give a cost I dont care about.
                .collect::<Vec<_>>()
        },
        |some_position| some_position.distance(&goal),
        |some_position| *some_position == goal,
    );

    let mut path_to_player = vec![];
    if let Some(result) = result {
        // Oui
        path_to_player = result.0;  
    }
    
    let mut rng = thread_rng();
    // positions.iter().map { Pour chaque element iteré qui corresponds à une position autour, on attribue une valeur -10 à 0, et si cette position est sur le Path, on lui donne NPC_MOVE_SCORE_BONUS}
    let actions = positions.iter()
        .map(|some_position_around | {
            // randomize movement choices
            let mut random_action_value = rng.gen_range(-10..0);
            //if let Some(path) = &path_to_player {
                // however prioritize a movement if it leads to the player                
                if path_to_player.contains(some_position_around) { random_action_value = NPC_MOVE_SCORE_BONUS }
            //}
            (Box::new(WalkAction(*entity, *some_position_around)) as Box<dyn super::Action>, NPC_MOVE_SCORE_DEFAULT + random_action_value)
        })
        .collect::<Vec<_>>();
    actor.0.extend(actions);
}