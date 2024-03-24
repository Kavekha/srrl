// Part of the refacto: what is commons to all menus
// TEMP

//Resume (ig menu) not working
//Music de combat reste quand retour au main menu depuis IG Menu.
//Reverifier coherence des menus.

//== DOCUMENTATION
// On commence en MenuState:Splashscreen, GameState:Disabled.
// Transmets un MessageEvent pour Afficher le Main Menu. Cet Event passe MenuState en Open.
// Le button_system contient tous les ordres de circulation dans les Menus, et est disponible dés que MenuState::Open.
// La gestion du ClearMenu reste pas ouf, il faut bien penser à l'ajouter à chaque fois. La mettre dans le Open semble faire le Clear après l'envoi du Menu... -_-

//== TODO 
// Encore beaucoup de doublons entre IG & MainMenu, à cause de la circulation. Peut être enregistrer l'option "Previous" dans le Menu à chaque fois?
// Desactiver les Controles "IG" / mettre GameState en Unavailable pendant le IG menu.


use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets,
    game::{
    manager::{game_messages::{QuitGameMessage, StartGameMessage}, menu_messages::{ClearMenuMessage, CloseMenuMessage, InGameMenuQuitMessage, InGameMenuSettingsOpenMessage, InGameSettingsDisplayMessage, MainMenuOpenMessage, MainMenuQuitMessage, MainMenuSettingsDisplayMessage, MainMenuSettingsMessage, OpenInGameMenuOpenMessage}, ExitAppMessage, MessageEvent}, 
    states::{GameState, MenuState}}, globals::{HEIGHT, NORMAL_BUTTON, RESOLUTION, TEXT_COLOR}};

use super::{button_system, components::{DisplayQuality, MenuButtonAction, OnScreenMenu, ResolutionSettings}, menu_camera};

pub struct CommonsMenuPlugin;

impl Plugin for CommonsMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DisplayQuality::Medium)
            .insert_resource(ResolutionSettings{
                low:Vec2::new(640.0, 360.0),
                medium:Vec2::new(HEIGHT * RESOLUTION, HEIGHT),
                high:Vec2::new(1920.0, 1080.0)
            })
            
            .init_state::<MenuState>()
  
            .add_systems(OnEnter(MenuState::Splashscreen), splashscreen)    
            .add_systems(OnEnter(MenuState::Splashscreen), menu_camera)  

             //Rassemblement Main Menu / IG MEnu : All actions. A la fin, devrait plutot se faire dans le MenuBuilder, associé à l'action
            .add_systems(OnEnter(MenuState::Open), menu_camera)
            .add_systems(Update, button_system.run_if(not(in_state(MenuState::Disabled))))
            .add_systems(Update, common_menu_action.run_if(not(in_state(MenuState::Disabled))))  // La gestion des actions IG Menu.
                 
            //Specific IG Menu            
            .add_systems(Update, ig_call_menu_input.run_if(in_state(GameState::Running)))   // Appeler IG Menu si In Game.            
            .add_systems(Update, ig_inside_menu_input.run_if(in_state(GameState::Unavailable)))     // TODO : Put the game In Unavailable quand Menu Open
            ;
        }
}

// GameState is Running, I can call Menu.
pub fn ig_call_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    //mut menu_state: ResMut<NextState<InGameMenuState>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Call for In Game Menu.");
        //menu_state.set(InGameMenuState::MainMenu);
        ev_message.send(MessageEvent(Box::new(OpenInGameMenuOpenMessage))); 
    }
}

// GameState is Unavailable, I can close the menu.
pub fn ig_inside_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    //mut menu_state: ResMut<NextState<InGameMenuState>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Back to game.");
        //menu_state.set(InGameMenuState::Disabled);
        ev_message.send(MessageEvent(Box::new(CloseMenuMessage))); 
    }
}


fn splashscreen(
    mut ev_message: EventWriter<MessageEvent>
){
    println!("Splashscreen: Open Main Menu");
    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage)));
}


// Ne contient que MainMenu pour le moment.
pub fn common_menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
    //mut game_state: ResMut<NextState<GameState>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
    mut ev_message: EventWriter<MessageEvent>   ,
    //mut ig_menu_state: ResMut<NextState<InGameMenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    println!("Go to game !");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage)));   
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));              
                }
                //TODO : Reactive LOAD.
                MenuButtonAction::Load => {
                    println!("Load a saved game!");
                    //load_saved_game(&mut app_state, &mut game_state); 
                    //load_game(app_state, game_state); 
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //menu_state.set(MainMenuState::Disabled);             
                }
                MenuButtonAction::MainMenuSettings => {
                    println!("Main Menu Settings!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsMessage)));                     
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to Main Menu.");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(QuitGameMessage)));
                    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage))); 
                }
                MenuButtonAction::MainMenuSettingsDisplay => {
                    println!("Main Menu Display Menu!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsDisplayMessage))); 
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
                MenuButtonAction::Quit => {
                    println!("Do you want to quit?");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuQuitMessage)));                     
                }                
                MenuButtonAction::QuitConfirm => {
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage)));                  
                }
                //Specific In Game Menu.
                MenuButtonAction::Close => {
                    println!("Close IG Menu");
                    ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));                  
                }
                MenuButtonAction::InGameMenuSettings => {
                    println!("IG Menu Setting");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(InGameMenuSettingsOpenMessage)));                  
                }
                MenuButtonAction::BackToInGameMenu => {
                    println!("Back to IG Menu");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(OpenInGameMenuOpenMessage)));                  
                }
                MenuButtonAction::InGameMenuQuit => {
                    println!("Back to IG Menu");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(InGameMenuQuitMessage)));                  
                }
                MenuButtonAction::InGameMenuDisplay => {
                    println!("Back to IG Menu");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(InGameSettingsDisplayMessage)));                  
                }
            }
        }
    }
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