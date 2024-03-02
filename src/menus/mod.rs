use bevy::prelude::*;

pub mod mainmenu;
pub mod gameover;
pub mod victory;
pub mod components;

use crate::game::despawn_screen;

use self::components::OnScreenMenu;


pub fn clean_menu(
    mut commands: Commands,
    despawn_onscreenmenu: Query<Entity, With<OnScreenMenu>>,
) {
    despawn_screen(despawn_onscreenmenu, &mut commands);
}