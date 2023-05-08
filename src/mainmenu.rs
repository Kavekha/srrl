use bevy::{prelude::*};

use crate::{
    despawn_screen, AppState, GameState, TILE_SIZE, HEIGHT,
    ascii::{spawn_ascii_text, AsciiSheet, NineSliceIndices, spawn_nine_slice}
};



#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    MainMenu,
    Disabled
}


#[derive(Component)]
pub struct OnScreenMenu;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()
            .add_systems(OnEnter(MenuState::MainMenu), spawn_title)
            .add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)            
            .add_systems(Update, main_menu_input.run_if(in_state(MenuState::MainMenu)))
            .add_systems(OnExit(MenuState::MainMenu), despawn_screen::<OnScreenMenu>);
    }
}


pub fn menu_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

#[derive(Component)]
pub enum MainMenuOptions {
    StartGame,
    Quit
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
    let x_offset = (-size.x / 2.0 + 1.5) * TILE_SIZE;
    let text = spawn_ascii_text(commands, ascii, text, Vec3::new(x_offset, 0.0, 0.0));

    commands
        .spawn(Name::new("Button"))
        .insert(OnScreenMenu)
        .insert(SpatialBundle{
            transform: Transform::from_translation(translation),
            ..default()
        })
        .insert(id)
        .add_child(nine_slice)
        .add_child(text)
        .id()
}

fn spawn_main_menu(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    nine_slice_indices: Res<NineSliceIndices>
){
    let box_height = 3.0;
    let box_center_x = 0.0;

    let mut box_center_y = box_height * TILE_SIZE / 2.0;

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
 
     // Quit game button.

     let quit_app_text = "Quit";
     let quit_app_width = (quit_app_text.len()+ 2) as f32;
     box_center_y -= box_height * TILE_SIZE;

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
    ascii: Res<AsciiSheet>
) {
    let title_drop = "SHADOWRUN";
    //let title_width = (title_drop.len()+ 2) as f32; 

    let text_placement= Vec3::new(
        0.0,
        0.0,
        0.0);

    let ascii_text = spawn_ascii_text(
        &mut commands,
        &ascii,
        &title_drop,
        text_placement
    );

    commands.entity(ascii_text)
    .insert(OnScreenMenu);

}

fn main_menu_input(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>
) {
    if keys.pressed(KeyCode::Space) {
        println!("Go to game !");      //TOLOG
        app_state.set(AppState::Game);
        game_state.set(GameState::GameMap);
        menu_state.set(MenuState::Disabled);
    }
}
