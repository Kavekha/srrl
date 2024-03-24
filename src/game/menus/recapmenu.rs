use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, game::menus::menu_builder::spawn_recap_menu,
    game::menus::components::OnScreenMenu
};

use super::menu_builder::MenuV2;


// TODO: Refacto Victory & GameOver en un seul: Recap Screen?

pub struct RecapMenuPlugin;

impl Plugin for RecapMenuPlugin {
    fn build(&self, app: &mut App){
        app
            .add_event::<MenuEvent>()
            .add_systems(Update, menu_event_reader.run_if(on_event::<MenuEvent>()));        
    }
}

// TODO : Refaire, car pas souple du tout. Ca construit le Menu par procuration, car on recoit un Event depuis World. C'est très moche.
#[derive(Event)]
pub struct MenuEvent{
    pub menu: MenuV2,
    pub menu_type: MenuType     //Type pour savoir quel menu on créé? Au cas où pôur le moment.
}

pub enum MenuType {
    RECAPMENU,
    MAINMENU,
    SETTINGS,
    DISPLAY,
    QUIT
}

fn menu_event_reader(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuEvent>,
    graph_assets: Res<GraphicsAssets>,
) {
    for event in ev_menu.read() {
        //println!("Je suis dans Menu Event Reader avec pour type: {:?}.", event.menu_type);
        println!("Menu reçu et envoyé.");
        let menu = &event.menu;
        spawn_recap_menu(&mut commands, graph_assets, menu);
        break;      // Degueu, mais seul le premier m'interesse et c peu probable que j'en ai d'autres.
    }    
}


// NE PAS DELETE : template utilisable pour Menu.

fn _display_gameover_screen(
    mut commands: Commands,
    graph_assets: Res<GraphicsAssets>
) {

    commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                OnScreenMenu
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "YOU DIED.",
                    TextStyle {
                        font: graph_assets.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "A ghoul has eaten you.",
                    TextStyle {
                        font: graph_assets.font.clone(),
                        font_size: 20.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
            });

}

