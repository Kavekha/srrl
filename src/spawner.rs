use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;
use super::{Player, Renderable,Position,SerializeMe};
use specs::saveload::{MarkedBuilder, SimpleMarker};


/// Spawns the player and returns his/her entity object.
pub fn player(ecs : &mut World, player_x : i32, player_y : i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}
