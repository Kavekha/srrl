use bevy::{prelude::*, app::AppExit};

use crate::{
    states::{AppState, GameState}, 
    //globals::{CHAR_SIZE, HEIGHT}, 
    //save_load_system::has_save_file, 
    asset_loaders::GraphicsAssets, 
//render::ascii::{NineSliceIndices, spawn_nine_slice, spawn_ascii_text}
};

use super::{
    components::OnScreenMenu,
    //components::{MainMenuOptions, OnScreenMenu, MainMenuSelection, MainMenuClickable}, 
    //NineSlice, 
    //menus_input::{menu_input_mouse, main_menu_input}, 
    clean_menu, 
};


// PLUGIN
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(AppState::MainMenu), spawn_title)
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)      
            .add_systems(OnEnter(AppState::MainMenu), menu_camera)    
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))          

            .add_systems(OnExit(AppState::MainMenu), clean_menu);
    }
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


/* 
pub fn main_menu_selecting(
    menu_selection: MainMenuOptions,
    app_state: &mut ResMut<NextState<AppState>>,
    game_state: &mut ResMut<NextState<GameState>>,
    app_exit_events: &mut EventWriter<AppExit>
) {
    match menu_selection {
        MainMenuOptions::StartGame => {
            println!("Go to game !");
            start_new_game(app_state, game_state);
        }
        MainMenuOptions::LoadGame => {
            println!("Load a saved game!");
            load_saved_game(app_state, game_state);
        }
        MainMenuOptions::Quit => {
            println!("Quit App");
            app_exit_events.send(AppExit);
        }
    }
}
*/

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

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Load,
    Settings,
    Quit
}

#[derive(Component)]
    struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
fn button_system(
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

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
    mut app_exit_events: EventWriter<AppExit>,
    //mut menu_state: ResMut<NextState<MenuState>>,
    //app_state: &mut ResMut<NextState<AppState>>,
    //game_state: &mut ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    //app_exit_events.send(AppExit);
                    println!("Quit App");
                    app_exit_events.send(AppExit);
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
                    //game_state.set(GameState::Game);
                    //menu_state.set(MenuState::Disabled);
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
                            MenuButtonAction::Quit,
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