use bevy::prelude::*;

use crate::vectors::Vector2Int;

// Le NPC doit considerer son objectif.
#[derive(Component)]
pub struct CheckGoal;


// Pas vraiment dans l'IA.
// Sert à ignorer un NPC qui n'a pas de raison d'être actif & voler du cpu pour prendre des decisions ou etre joué.
#[derive(Component)]
pub struct Frozen;

#[derive(Component)]
pub struct PlanMove {
    pub destination: Vector2Int
}

#[derive(Component)]
pub struct PlanFlee {
    pub away_from: Vector2Int
}

#[derive(Component)]
pub struct PlanSearch;

#[derive(Component)]
pub struct Knowledge {
    pub player_last_seen : Option<Vector2Int>,
    pub last_visited_nodes: Vec<Vector2Int>
}

#[derive(Component)]
pub struct HasTalked;

#[derive(Component)]
pub struct PlanInformAllies{
    pub target_position: Vector2Int
}

#[derive(Component)]
pub struct HasShareInfos;