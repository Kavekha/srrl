use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map_builders::commons::TileType;





#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct TileExit;



#[derive(Component)]
pub struct GameMapRender;