use bevy::{prelude::*};

use crate::{
    despawn_screen, AppState, GameState, TILE_SIZE, RESOLUTION,
    ascii::{spawn_ascii_text, AsciiSheet, NineSliceIndices, spawn_nine_slice},
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
            //.add_systems(OnEnter(MenuState::MainMenu), menu_setup)
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
    nine_slices_indices: Res<NineSliceIndices>
){
    let box_height = 3.0;
    let box_center_y = -1.0 + box_height * TILE_SIZE / 2.0;

    let start_game_text = "Start game";
    let start_game_width = (start_game_text.len()+ 2) as f32;
    let start_game_center_x = RESOLUTION - (start_game_width * TILE_SIZE) / 2.0;

    spawn_menu_button(
        &mut commands, 
        &ascii, 
        &nine_slices_indices, 
        Vec3::new(start_game_center_x, box_center_y, 100.0),
        start_game_text, 
        MainMenuOptions::StartGame,
        Vec2::new(start_game_width, box_height)
    );
}

fn menu_setup(
    mut commands: Commands,
    ascii: Res<AsciiSheet>
) {
    let title_drop = "SHADOWRUN";
    let x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let text_to_display = vec![title_drop];

    for text in text_to_display{
        //TODO : Ce qui suit est dégueu, et utilisé pour centrer le texte.
        let ox = (text.len()/2) as f32; 
        let mo = ox /10.0;      // Je le divise par 10 car trop gros mais impossible à diviser par 20 auparavant car arrondi à 0.
        let final_x: f32 = x - mo;
        
        let text_placement= Vec3::new(final_x, y, 0.0);
        let ascii_text = spawn_ascii_text(
            &mut commands,
            &ascii,
            &text,
            text_placement
        );
        commands.entity(ascii_text)
        .insert(OnScreenMenu);

        y -= 0.2
    }
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
