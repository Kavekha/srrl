// window resizing : https://github.com/bevyengine/bevy/blob/main/examples/window/window_resizing.rs
    // Not really a resize: resolution is changed, but low = less thing to see.


use bevy::{prelude::*, app::AppExit};
use crate::{
    states::{AppState, GameState, MainMenuState}, 
    asset_loaders::GraphicsAssets, 
};

use super::{
    clean_menu, components::{DisplayQuality, MenuButtonAction, OnScreenMenu, ResolutionSettings, SelectedOption} 
};


// PLUGIN
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<MainMenuState>()

            .insert_resource(DisplayQuality::Medium)
            .insert_resource(ResolutionSettings{
                low:Vec2::new(640.0, 360.0),
                medium:Vec2::new(800.0, 600.0),
                high:Vec2::new(1920.0, 1080.0)
            })

            .add_systems(OnEnter(AppState::MainMenu), load_main_menu)
            .add_systems(OnEnter(MainMenuState::MainMenu), menu_camera)  
            .add_systems(OnEnter(MainMenuState::MainMenu), spawn_main_menu)      
            .add_systems(OnEnter(MainMenuState::Settings), spawn_settings_menu)      
            .add_systems(OnEnter(MainMenuState::DisplayMenu), spawn_display_menu)      
            .add_systems(OnEnter(MainMenuState::QuitConfirm), spawn_quit_confirm_menu)
            
              
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))   
            .add_systems(Update, resolution_menu_action.run_if(in_state(MainMenuState::DisplayMenu)))    //Only in display menu there. Not really cool but hey.   
            

            .add_systems(OnExit(MainMenuState::MainMenu), clean_menu)
            .add_systems(OnExit(MainMenuState::Settings), clean_menu)
            .add_systems(OnExit(MainMenuState::DisplayMenu), clean_menu)               
            .add_systems(OnExit(MainMenuState::QuitConfirm), clean_menu)   
            .add_systems(OnExit(AppState::MainMenu), quit_main_menu);
    }
}




fn load_main_menu(
    mut mainmenu_state: ResMut<NextState<MainMenuState>>
){
    mainmenu_state.set(MainMenuState::MainMenu);
}

fn quit_main_menu(
    mut mainmenu_state: ResMut<NextState<MainMenuState>>
){
    mainmenu_state.set(MainMenuState::Disabled);
}


/// Lance une nouvelle partie depuis le menu. 
// TODO : Deplacer dans Game Mod?
fn start_new_game(
    app_state: &mut ResMut<NextState<AppState>>,
    game_state: &mut ResMut<NextState<GameState>>,
) {
    app_state.set(AppState::Game);
    game_state.set(GameState::NewGame);

}

fn load_saved_game(
    app_state: &mut ResMut<NextState<AppState>>,
    game_state: &mut ResMut<NextState<GameState>>,
){
    app_state.set(AppState::Game);
    game_state.set(GameState::LoadGame);
    //load_game(app_state, game_state);
}


/// Camera centré sur 0.0,0.0 pour ne pas avoir contenu des menus off screen.
pub fn menu_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

// Bevy example
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);          // TODO : Même couleur que le fond si on veut le cacher. Defaut background button est blanc.
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);



// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}


pub fn resolution_menu_action(
    interaction_query: Query<(&Interaction, &DisplayQuality), (Changed<Interaction>, With<Button>),>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
){
    let mut window = windows.single_mut();
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                DisplayQuality::Low => {
                    println!("Resolution changed to Low");
                    let res = resolution.low;
                    window.resolution.set(res.x, res.y);
                }
                DisplayQuality::Medium => {
                    println!("Resolution changed to Medium");
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                }
                DisplayQuality::High => {
                    println!("Resolution changed to High");
                    let res = resolution.high;
                    window.resolution.set(res.x, res.y);
                }
            }
        }
    }
}

pub fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MainMenuState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::QuitConfirm => {
                    //app_exit_events.send(AppExit);
                    println!("Do you want to quit?");
                    menu_state.set(MainMenuState::QuitConfirm);
                }
                MenuButtonAction::Quit => {
                    //app_exit_events.send(AppExit);
                    println!("Quit App");
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Cancel => {
                    //app_exit_events.send(AppExit);
                    println!("Don't want to quit.");
                    menu_state.set(MainMenuState::MainMenu);
                }
                MenuButtonAction::Play => {
                    println!("Go to game !");
                    start_new_game(&mut app_state, &mut game_state);
                    //game_state.set(GameState::Game);
                    //menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Load => {
                    //game_state.set(GameState::Game);
                    //menu_state.set(MenuState::Disabled);
                    println!("Load a saved game!");
                    load_saved_game(&mut app_state, &mut game_state);
                }
                MenuButtonAction::Settings => {
                    println!("Settings!");
                    menu_state.set(MainMenuState::Settings);
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to main menu");
                    menu_state.set(MainMenuState::MainMenu)
                }
                MenuButtonAction::SettingsDisplay => {
                    println!("Display Menu!");
                    menu_state.set(MainMenuState::DisplayMenu);
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings!");
                    menu_state.set(MainMenuState::Settings);
                }
                _ => {
                    println!("Something Else to deal with!");
                }
            }
        }
    }
}


// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
/* 
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
*/

fn spawn_quit_confirm_menu(
    mut commands: Commands 
){
    println!("Menu de confirmation");
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
                    for (action, text) in [                        
                        (MenuButtonAction::Cancel, "Cancel"),
                        (MenuButtonAction::Quit, "Confirm"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}

fn spawn_main_menu(
    mut commands: Commands, 
    //asset_server: Res<AssetServer>,
    graphics_assets: Res<GraphicsAssets>
) {
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
                            MenuButtonAction::Settings,
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


fn spawn_settings_menu(mut commands: Commands) {
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
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}

fn spawn_display_menu(
    mut commands: Commands, 
    display_quality: Res<DisplayQuality>
) {
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
            OnScreenMenu,   //OnDisplaySettingsMenuScreen,
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
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            //background_color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn(TextBundle::from_section(
                                "Display Quality",
                                button_text_style.clone(),
                            ));
                            // Display a button for each possible value
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(75.0),
                                            height: Val::Px(32.5),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    quality_setting
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        format!("{quality_setting:?}"),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}
/* 
   
    //let has_file = Path::new("assets/scenes/load_scene_example.scn.ron").exists();
    //println!("Mon Path est : {:?}", has_file);
    if has_save_file() {
        // Load game button.
        let load_game_text = "Load game";
        let load_game_width = (load_game_text.len()+ 2) as f32;  
        box_center_y -= box_height * CHAR_SIZE;

        spawn_menu_button(
            &mut commands, 
            &ascii, 
            &nine_slice_indices, 
            Vec3::new(box_center_x, box_center_y, 100.0),
            load_game_text, 
            MainMenuOptions::LoadGame,
            Vec2::new(load_game_width, box_height)
        ); 
    } else {
        println!("je n'ai pas de fichier de save")    }


*/