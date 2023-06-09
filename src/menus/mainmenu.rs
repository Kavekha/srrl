use bevy::{prelude::*, app::AppExit};

use crate::{states::{AppState, GameState}, ascii::{NineSliceIndices, spawn_ascii_text, spawn_nine_slice}, globals::{CHAR_SIZE, HEIGHT}, save_load_system::has_save_file};

use super::{
    components::{MainMenuOptions, OnScreenMenu, MainMenuSelection, MainMenuClickable}, 
    NineSlice, AsciiSheet, 
    menus_input::{menu_input_mouse, main_menu_input}, clean_menu, 
};




// PLUGIN
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_title)
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)      
            .add_systems(OnEnter(AppState::MainMenu), menu_camera)    
            .insert_resource(MainMenuSelection { selected: MainMenuOptions::StartGame })      
            .add_systems(Update, main_menu_input.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, menu_input_mouse.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, hightligh_menu_button.run_if(in_state(AppState::MainMenu)))

            
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

/// Camera centré sur 0.0,0.0 pour ne pas avoir contenu des menus off screen.
pub fn menu_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn hightligh_menu_button(
    menu_state: Res<MainMenuSelection>,
    button_query: Query<(&Children, &MainMenuOptions)>,
    nine_slice_query: Query<&Children, With<NineSlice>>,
    mut sprites_query: Query<&mut TextureAtlasSprite>
){
    // On se balade dans la hierarchie du menu pour choisir la couleur du bouton selon que son id = menu_state.selected
    for (button_children, button_id) in button_query.iter() {
        for button_child in button_children.iter() {
            if let Ok(nine_slice_children) = nine_slice_query.get(*button_child) {
                for nine_slice_child in nine_slice_children.iter() {
                    if let Ok(mut sprite) = sprites_query.get_mut(*nine_slice_child){
                        if menu_state.selected == *button_id {
                            sprite.color = Color::RED;
                        } else {
                            sprite.color = Color::WHITE;
                        }
                    }
                }
            }
        }
    }
}

fn spawn_menu_button(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    indices: &NineSliceIndices,
    translation: Vec3,
    text: &str,
    id: MainMenuOptions,
    size: Vec2
) -> Entity {
    let nine_slice = spawn_nine_slice(commands, ascii, indices, size.x, size.y);
    let x_offset = (-size.x / 2.0 + 1.5) * CHAR_SIZE;
    let text = spawn_ascii_text(commands, ascii, text, Vec3::new(x_offset, 0.0, 0.0));

    commands
        .spawn(Name::new("Button"))
        .insert(OnScreenMenu)
        .insert(SpatialBundle{
            transform: Transform::from_translation(translation),
            ..default()
        })
        .insert(id)
        .insert(MainMenuClickable {size: size, id: id})
        .add_child(nine_slice)
        .add_child(text)
        .id()
}

fn spawn_main_menu(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    nine_slice_indices: Res<NineSliceIndices>
){
    println!("Main Menu spawned");

    let box_height = 3.0;
    let box_center_x = 0.0;

    let mut box_center_y = box_height * CHAR_SIZE / 2.0;

    // Start game button
    let start_game_text = "Start game";
    let start_game_width = (start_game_text.len()+ 2) as f32;  

    spawn_menu_button(
        &mut commands, 
        &ascii, 
        &nine_slice_indices, 
        Vec3::new(box_center_x, box_center_y, 100.0),
        start_game_text, 
        MainMenuOptions::StartGame,
        Vec2::new(start_game_width, box_height)
    ); 
    
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
        println!("je n'ai pas de fichier de save")
    }

     // Quit game button.

     let quit_app_text = "Quit";
     let quit_app_width = (quit_app_text.len()+ 2) as f32;
     box_center_y -= box_height * CHAR_SIZE;

     spawn_menu_button(
        &mut commands, 
        &ascii, 
        &nine_slice_indices, 
        Vec3::new(box_center_x, box_center_y, 100.0),
        quit_app_text, 
        MainMenuOptions::Quit,
        Vec2::new(quit_app_width, box_height)
    ); 

}

fn spawn_title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let middle_upper = HEIGHT / 4.0;
    //let img_width = 384.0;
    //let img_height = 48.0;
    //let img_mid_x = (HEIGHT * RESOLUTION / 4.0) - (img_width / 2.0);

    let title_drop = commands.spawn(SpriteBundle {
        texture: asset_server.load("title/shadowrun_title_alone.png"),    //asset_server.load("temp_tiles/Sewers_wall.png"),
        transform: Transform {
            translation: Vec3::new(0.0, middle_upper / 2.0, 0.0),
            //translation: Vec3::new(-0.2, middle_upper, 0.0),
            scale: Vec3::splat(1.0),   //splat(1.0),
            ..default()
        },
        ..default()
    })
    .id();
    commands.entity(title_drop)
    .insert(OnScreenMenu);


}


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