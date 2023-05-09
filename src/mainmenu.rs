use bevy::{prelude::*, app::AppExit};

use crate::{
    despawn_screen, AppState, GameState, TILE_SIZE,
    ascii::{spawn_ascii_text, AsciiSheet, NineSliceIndices, spawn_nine_slice, NineSlice}
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
            .insert_resource(MainMenuSelection { selected: MainMenuOptions::StartGame })      
            .add_systems(Update, main_menu_input.run_if(in_state(MenuState::MainMenu)))
            .add_systems(Update, hightligh_menu_button.run_if(in_state(MenuState::MainMenu)))
            .add_systems(OnExit(MenuState::MainMenu), despawn_screen::<OnScreenMenu>)
            .add_systems(OnEnter(MenuState::MainMenu), play_music); //TEST AUDIO
    }
}


//TEST AUDIO ok with Cargo Run, not with Visual Code...... ONLY .OGG WORKING wat???
pub fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    //let music = asset_server.load("audios/01 Seattle 2050 (American).mp3");
    let music = asset_server.load("audios/Windless Slopes.ogg");
    audio.play(music);
}


pub fn menu_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

pub const MAIN_MENU_OPTIONS_COUNT: isize = 2;  //Necessaire pour la selection d'une option dans l'input.

#[derive(Component, PartialEq, Clone, Copy)]
pub enum MainMenuOptions {
    StartGame,
    Quit
}


#[derive(Resource)]
pub struct MainMenuSelection {
    selected: MainMenuOptions
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

    //TODO: Where the f is this? 0.0 = Center, -1.0 ==> totaly out of scope. There is a modifier somewhere? Tilesize?

    let text_placement= Vec3::new(
        -0.2,
        0.3,
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
    mut menu_state: ResMut<NextState<MenuState>>,
    mut menu_selection: ResMut<MainMenuSelection>,
    mut app_exit_events: EventWriter<AppExit>
) {
    let mut current_selection = menu_selection.selected as isize;
    if keys.any_just_pressed([KeyCode::Up, KeyCode::Z]) {
        current_selection -=1;
    }
    if keys.any_just_pressed([KeyCode::Down, KeyCode::D]) {
        current_selection -=1;
    }

    current_selection = (current_selection + MAIN_MENU_OPTIONS_COUNT) % MAIN_MENU_OPTIONS_COUNT;

    menu_selection.selected = match current_selection {
        0 => MainMenuOptions::StartGame,
        1 => MainMenuOptions::Quit,
        _ => unreachable!("Bad Main menu selection")
    };


    if keys.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        match menu_selection.selected {
            MainMenuOptions::StartGame => {
                println!("Go to game !");
                app_state.set(AppState::Game);
                game_state.set(GameState::CharacterCreation);
                menu_state.set(MenuState::Disabled);
            }
            MainMenuOptions::Quit => {
                println!("Quit App");   //TODO
                app_exit_events.send(AppExit);
            }
        }
    }
}
