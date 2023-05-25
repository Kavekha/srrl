use bevy::prelude::*;

use crate::map_builders::pathfinding::Position;


#[derive(Component)]
pub struct Pathfinding{
    pub start: Position,
    pub goal: Position,
    pub path: Vec<Position>,
    pub step: usize,
    pub dirty: bool,    //Si True, verifie la position vs Step Destination pour savoir si chemin atteint et next ordre de mouvement necessaire.
    pub debug: bool,
}

#[derive(Component)]
pub struct MoveTo{
    pub x: f32,
    pub y: f32,
    pub destination: Position
}
