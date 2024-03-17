use bevy::prelude::*;

pub mod mainmenu;
pub mod gameover;
pub mod victory;
pub mod components;
pub mod ingamemenu;

use crate::game::despawn_screen;

use self::{components::OnScreenMenu, gameover::GameOverPlugin, ingamemenu::InGameMenuPlugin, mainmenu::MainMenuPlugin, victory::VictoryPlugin};


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MainMenuPlugin)
            .add_plugins(VictoryPlugin)
            .add_plugins(GameOverPlugin)
            .add_plugins(InGameMenuPlugin);
    }
}


pub fn clean_menu(
    mut commands: Commands,
    despawn_onscreenmenu: Query<Entity, With<OnScreenMenu>>,
) {
    println!("Cleaning menu");
    despawn_screen(despawn_onscreenmenu, &mut commands);
}