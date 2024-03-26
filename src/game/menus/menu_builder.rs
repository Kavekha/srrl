
use bevy::prelude::*;

use crate::{
    game::menus::components::OnScreenMenu, globals::{NORMAL_BUTTON, TEXT_COLOR},
    engine::asset_loaders::GraphicsAssets
};

use super::components::MenuButtonAction;


//MenuBuilder v2
#[derive(Clone)]
pub struct Action {pub action: MenuButtonAction, pub text:String}
#[derive(Clone)]
pub struct Header {pub text: String}
#[derive(Clone)]
pub struct Description {pub text: String}
#[derive(Clone)]
pub struct Image {pub image: String}
#[derive(Clone)]
pub struct Footer {pub text: String}


#[derive(Clone)]
pub enum MenuItem{
    Action(Action),
    Header(Header),
    Description(Description),
    Image(Image),
    Footer(Footer)
}

impl MenuItem{
    pub fn action(action:MenuButtonAction, text:&str
    ) -> MenuItem {
        MenuItem::Action(Action{action: action, text:text.to_string()})
    }
    pub fn header(text:&str
    ) -> MenuItem {
        MenuItem::Header(Header{text:text.to_string()})
    }
    pub fn description(text:&str
    ) -> MenuItem {
        MenuItem::Description(Description{text:text.to_string()})
    }
    pub fn image(image:&str
    ) -> MenuItem {
        MenuItem::Image(Image{image: image.to_string()})
    }
    pub fn footer(text: &str
    ) -> MenuItem {
        MenuItem::Footer(Footer{text: text.to_string()})
    } 
}

pub struct Menu{
    pub id: String,
    pub entries: Vec<MenuItem>
}
impl Menu{
    pub fn new(id: &str, entries: Vec<MenuItem>) -> Menu {
        let menu = Menu{
            id: id.to_string(),
            entries: entries
        };
        menu
    } 
    pub fn add(&mut self, menu_item: MenuItem){
        self.entries.push(menu_item);
    }
}

// WORK IN PROGRESS
pub fn spawn_recap_menu(
    commands: &mut Commands,
    graph_assets: Res<GraphicsAssets>,
    menu: &Menu,
) {
    let mut images:Vec<Image> = Vec::new();
    let mut headers:Vec<Header>= Vec::new();
    let mut descriptions:Vec<Description> = Vec::new();
    let mut actions:Vec<Action> = Vec::new();
    let mut footers:Vec<Footer> = Vec::new();

    for item in menu.entries.iter() {
        match item {
            MenuItem::Header(header) => headers.push(header.clone()),
            MenuItem::Description(description) => descriptions.push(description.clone()),
            MenuItem::Action(action) => actions.push(action.clone()),
            MenuItem::Image(image) => images.push(image.clone()),
            MenuItem::Footer(footer) => footers.push(footer.clone()),
            _ => println!("This MenuItem is not supported.")
        };
    }

    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(125.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    /*
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
        */
    let button_text_style = TextStyle {
        font_size: 20.0,    //40
        color: TEXT_COLOR,
        ..default()
    };

    // The full screen as a UI element
    let screen_menu = commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu,
        )).id();
    
    // Illustration.
    for image in images.iter() {
        let image = image.clone();
        let logo = commands.spawn(ImageBundle {
                            image: graph_assets.logo.clone().into(),
                            ..default()
        }).id();
        commands.entity(screen_menu).push_children(&[logo]);
        println!("Image devrait être chargée.");
    }
    

    // Header
    for header in headers.iter() {
        let menu_header = commands.spawn(TextBundle::from_section(
            header.text.clone(),   //"YOU DIED.",
            TextStyle {
                font: graph_assets.font.clone(),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        )).id();
        commands.entity(screen_menu).push_children(&[menu_header]);
    }

    // Description
    for description in descriptions.iter() {
        let menu_description = commands.spawn(TextBundle::from_section(
            description.text.clone(),   //"YOU DIED.",
            TextStyle {
                    font: graph_assets.font.clone(),
                    font_size: 15.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            )).id();
        commands.entity(screen_menu).push_children(&[menu_description]);
    }

    // Si y a des options, on mets un cadre.
    if actions.len() > 0 {
        let menu_border = commands.spawn(NodeBundle {
            // Cadre du menu en lui-même.
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            //background_color: Color::CRIMSON.into(),
            ..default()
        }).id();

        commands.entity(screen_menu).push_children(&[menu_border]);

        // Options.
        for action in actions.iter() {
            let action_button = commands.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                action.action.clone(),    //action,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    action.text.clone(),  //text,
                    button_text_style.clone(),
                ));
            }).id();
            commands.entity(menu_border).push_children(&[action_button]);
        };

        // Footers.
        // Si y a des options, on mets un cadre.
        if footers.len() > 0 {
            let menu_down = commands.spawn(NodeBundle {
                // Cadre du menu en lui-même.
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                //background_color: Color::CRIMSON.into(),
                ..default()
            }).id();
            commands.entity(screen_menu).push_children(&[menu_down]);

            for footer in footers.iter() {
                let menu_footer = commands.spawn(TextBundle::from_section(
                    footer.text.clone(),  //text,
                    button_text_style.clone(),
                )).id();
                commands.entity(menu_down).push_children(&[menu_footer]);
            };
        }
    }      
}



// Theme 2 : Recap Menu :: CURRENT
pub fn spawn_recap_menu_2(
commands: &mut Commands,
graph_assets: Res<GraphicsAssets>,
menu: &Menu
) {
    println!("Spawning recap menu.");
let button_style = Style {
    width: Val::Px(100.0),
    height: Val::Px(32.5),
    margin: UiRect::all(Val::Px(10.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
};

let button_text_style = TextStyle {
    font_size: 20.0,
    color: TEXT_COLOR,
    ..default()
};

commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu
        ))
        .with_children(|parent| {
            for item in &menu.entries {
                match item {
                    MenuItem::Header(header) => {
                        parent.spawn(TextBundle::from_section(
                            header.text.clone(),   //"YOU DIED.",
                            TextStyle {
                                font: graph_assets.font.clone(),
                                font_size: 30.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                            },
                        ));
                    },
                    MenuItem::Description(description) => {
                        parent.spawn(TextBundle::from_section(
                            description.text.clone(),   //"YOU DIED.",
                            TextStyle {
                                    font: graph_assets.font.clone(),
                                    font_size: 15.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            ));
                        },
                        MenuItem::Action(action) => {
                            parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action.action.clone(),    //action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    action.text.clone(),  //text,
                                    button_text_style.clone(),
                                ));
                            });
                        }                    
                    _ => println!("MenuItem non géré")
                };
            }

                {
                    
                }
        });
}

// Theme 1 : Classic Menu

//Not compatible with MenuV2
pub fn _spawn_basic_menu(
    //commands: &mut Commands, 
    //new_menu: Menu
) {
    println!("In Game Menu");
    //let new_menu = Menu::new();

    /* 
    let button_style = Style {
        width: Val::Px(100.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 20.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu,   //OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    //background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for page in new_menu.pages 
                    /* 
                    for (action, text) in [
                                                (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] */
                    {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                page.action,    //action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    page.text,  //text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
        */
}



fn _gameover_screen(
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


pub fn _main_menu_screen(
    mut commands: Commands, 
    //asset_server: Res<AssetServer>,
    graphics_assets: Res<GraphicsAssets>
) {
    println!("Menu principal");
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(125.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    /*
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
     */
    let button_text_style = TextStyle {
        font_size: 20.0,    //40
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            // L'ensemble de la fenetre UI. Tout s'organise autour de ca.
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu,
        ))
        .with_children(|parent| {
            // Titre
            let logo = graphics_assets.logo.clone();
            parent.spawn(ImageBundle {
                                image: UiImage::new(logo),
                                ..default()
            });
            parent
                .spawn(NodeBundle {
                    // Cadre du menu en lui-même.
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    //background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display buttons for each action available from the main menu:
                        // - new game 
                        // - load game if apply
                        // - settings
                        // - quit
                    // NEW GAME
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            /* 
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            */
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                button_text_style.clone(),
                            ));
                        });
                    // LOAD GAME
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Load,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Load game",
                                button_text_style.clone(),
                            ));
                        });
                    // SETTINGS
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            //MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            /* 
                            let icon = asset_server.load("textures/Game Icons/wrench.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            */
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    // QUIT APP
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::QuitConfirm,
                        ))
                        .with_children(|parent| {
                            /* 
                            let icon = asset_server.load("textures/Game Icons/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            */
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}
