// window resizing : https://github.com/bevyengine/bevy/blob/main/examples/window/window_resizing.rs
    // Not really a resize: resolution is changed, but low = less thing to see.


use bevy::prelude::*;
use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{
        manager::{game_messages::StartGameMessage, menu_messages::{ActiveMainMenuMessage, CloseMainMenuMessage}, ExitAppMessage, MessageEvent}, 
        menus::menu_builder::{spawn_basic_menu, Menu, MenuView}, states::{GameState, MainMenuState, MenuState}
    }, 
    globals::{
        HEIGHT, 
        //HOVERED_BUTTON, HOVERED_PRESSED_BUTTON, PRESSED_BUTTON, 
        NORMAL_BUTTON, RESOLUTION, TEXT_COLOR} 
};

use super::{
    button_system, clean_menu, components::{
        DisplayQuality, MenuButtonAction, OnScreenMenu, ResolutionSettings
        //, SelectedOption
    },
    menu_camera 
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
                medium:Vec2::new(HEIGHT * RESOLUTION, HEIGHT),
                //medium:Vec2::new(800.0, 600.0),
                high:Vec2::new(1920.0, 1080.0)
            })
            //.add_systems(OnEnter(MainMenuState::MainMenu), load_main_menu)
            //.add_systems(OnEnter(MainMenuState::MainMenu), menu_camera)  //0.15.2 In Commons

            //.add_systems(OnEnter(MainMenuState::MainMenu), spawn_main_menu)      
            .add_systems(OnEnter(MainMenuState::Settings), enter_mm_settings_menu)      
            .add_systems(OnEnter(MainMenuState::DisplayMenu), enter_mm_display_menu)      
            .add_systems(OnEnter(MainMenuState::QuitConfirm), enter_mm_quit_confirm_menu)
            
            //.add_systems(Update, button_system.run_if(not(in_state(MainMenuState::Disabled))))    // 0.15.2 : Dans Commons maintenant, avec Open. 
            //.add_systems(Update, main_menu_action.run_if(not(in_state(MainMenuState::Disabled)))  )   // 0.15.2 : Dans Commons maintenant, sera en commun avec IG etc.
            
            .add_systems(OnExit(MainMenuState::QuitConfirm), clean_menu)   

            .add_systems(OnEnter(MainMenuState::Disabled), clean_menu)
            /* 0.15.2 for all states of main menu.
            .add_systems(OnExit(MainMenuState::MainMenu), clean_menu)
            .add_systems(OnExit(MainMenuState::Settings), clean_menu)
            .add_systems(OnExit(MainMenuState::DisplayMenu), clean_menu)               
            .add_systems(OnExit(MainMenuState::QuitConfirm), clean_menu)  */ 
            ;
    }
}



pub fn main_menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
     mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
    mut ev_message: EventWriter<MessageEvent>   //NEW MESSAGE EVENT SYSTEM v0.15.2
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
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //app_exit_events.send(AppExit);
                }
                MenuButtonAction::Cancel => {
                    //app_exit_events.send(AppExit);
                    println!("Don't want to quit.");
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage))); //menu_state.set(MainMenuState::MainMenu);
                }
                MenuButtonAction::Play => {
                    println!("Go to game !");                    
                    ev_message.send(MessageEvent(Box::new(CloseMainMenuMessage)));  //menu_state.set(MainMenuState::Disabled);
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //game_state.set(GameState::NewGame);                    
                }
                MenuButtonAction::Load => {
                    println!("Load a saved game!");
                    //load_saved_game(&mut app_state, &mut game_state); 
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //menu_state.set(MainMenuState::Disabled);
                    game_state.set(GameState::LoadGame);
                    //load_game(app_state, game_state);
                }
                MenuButtonAction::Settings => {
                    println!("Settings!");
                    menu_state.set(MainMenuState::Settings);
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to main menu");
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage))); //menu_state.set(MainMenuState::MainMenu)
                }
                MenuButtonAction::SettingsDisplay => {
                    println!("Display Menu!");
                    menu_state.set(MainMenuState::DisplayMenu);
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings!");
                    menu_state.set(MainMenuState::Settings);
                }
                MenuButtonAction::DisplayLow => {
                    println!("Resolution changed to Low");
                    let mut window = windows.single_mut();                
                    let res = resolution.low;
                    window.resolution.set(res.x, res.y);
                }
                MenuButtonAction::DisplayMedium => {
                    println!("Resolution changed to Medium");
                    let mut window = windows.single_mut();              
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                }
                MenuButtonAction::DisplayHigh => {
                    println!("Resolution changed to High");
                    let mut window = windows.single_mut();                  
                    let res = resolution.high;
                    window.resolution.set(res.x, res.y);
                }
                _ => {
                    println!("Something Else to deal with!");
                }
            }
        }
    }
}



pub fn enter_mm_quit_confirm_menu(mut commands: Commands) {
    println!("Entering MM Quit Confirm menu.");
    let mut menu = Menu::new();
    for (action, text) in [                            
            (MenuButtonAction::Cancel, "Cancel"),
            (MenuButtonAction::Quit, "Confirm"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}

pub fn enter_mm_settings_menu(mut commands: Commands) {
    println!("Entering MM Quit Confirm menu.");
    let mut menu = Menu::new();
    for (action, text) in [
            (MenuButtonAction::SettingsDisplay, "Display"),
            //(MenuButtonAction::SettingsSound, "Sound"),
            (MenuButtonAction::BackToMainMenu, "Back"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}

pub fn enter_mm_display_menu(mut commands: Commands) {
    println!("Entering MM Quit Confirm menu.");
    let mut menu = Menu::new();
    for (action, text) in [
        (MenuButtonAction::DisplayLow, "Low"),
        (MenuButtonAction::DisplayMedium, "Medium"),
        (MenuButtonAction::DisplayHigh, "High"),
        (MenuButtonAction::BackToSettings, "Back"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}



pub fn spawn_main_menu(
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