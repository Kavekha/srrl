use bevy::prelude::*;

// Le NPC doit considerer son objectif.
#[derive(Component)]
pub struct CheckGoal;

#[derive(Component)]
pub struct Goal;

// Pas vraiment dans l'IA.
// Sert à ignorer un NPC qui n'a pas de raison d'être actif & voler du cpu pour prendre des decisions ou etre joué.
#[derive(Component)]
pub struct Frozen;