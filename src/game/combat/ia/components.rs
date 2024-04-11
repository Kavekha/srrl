use bevy::prelude::*;

use crate::vectors::Vector2Int;


#[derive(Debug)]
pub enum GoalType{
    KillEntity { id: Entity },
    None,
}


#[derive(Component)]
pub struct Goal {
    pub id: GoalType,
}


// Le NPC doit considerer son objectif.
#[derive(Component)]
pub struct CheckGoal;

// Necessaire pour que le NPC sache qu'il doit plannifier.
#[derive(Component)]
pub struct Planning;

#[derive(Component)]
pub struct PlanMove {
    pub destination: Vector2Int
}

// Pas vraiment dans l'IA.
// Sert à ignorer un NPC qui n'a pas de raison d'être actif & voler du cpu pour prendre des decisions ou etre joué.
#[derive(Component)]
pub struct Frozen;