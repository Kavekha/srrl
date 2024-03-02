use bevy::prelude::*;

pub mod mainmenu;
pub mod gameover;
pub mod victory;
mod components;

pub use components::{
    //NineSlice, 
    OnScreenMenu};

use crate::game::despawn_screen;


pub fn clean_menu(
    mut commands: Commands,
    despawn_onscreenmenu: Query<Entity, With<OnScreenMenu>>,
) {
    despawn_screen(despawn_onscreenmenu, &mut commands);
}