/* 
use bevy::{prelude::*, input::{mouse::MouseButtonInput, ButtonState}, app::AppExit};

use crate::{
    globals::{CHAR_SIZE, MAIN_MENU_OPTIONS_COUNT},
    //menus::mainmenu::main_menu_selecting, 
    states::{AppState, GameState}, save_load_system::has_save_file};

//use super::components::{MainMenuClickable, MainMenuSelection, MainMenuOptions};

/* 



// TODO : Deplacer avec meilleure visibilité dans un Mod menu?
pub fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_selection: ResMut<MainMenuSelection>,
    mut app_exit_events: EventWriter<AppExit>
) {
    let mut current_selection = menu_selection.selected as isize;
    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyZ]) {
        current_selection -=1;
        //TODO : crado, si pas de save.
        //let has_file = Path::new("assets/scenes/load_scene_example.scn.ron").exists();
        if current_selection == 1 && !has_save_file() { // !has_file{
            current_selection -= 1;
        }
    }
    if keys.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyD]) {
        current_selection +=1;
        //TODO : crado, si pas de save.
        //let has_file = Path::new("assets/scenes/load_scene_example.scn.ron").exists();
        if current_selection == 1 && !has_save_file() {     // !has_file{
            current_selection += 1;
        }
    }

    current_selection = (current_selection + MAIN_MENU_OPTIONS_COUNT) % MAIN_MENU_OPTIONS_COUNT;

    menu_selection.selected = match current_selection {
        0 => MainMenuOptions::StartGame,
        1 => {
            //let has_file = Path::new("assets/scenes/load_scene_example.scn.ron").exists();
            if has_save_file() {    //if has_file {
                MainMenuOptions::LoadGame
            } else {
                println!("No file, no load");
                MainMenuOptions::StartGame
            }         
        },
        2 => MainMenuOptions::Quit,
        _ => unreachable!("Bad Main menu selection")
    };


    if keys.any_just_pressed([KeyCode::Space, KeyCode::Enter]) {
        main_menu_selecting(menu_selection.selected, &mut app_state, &mut game_state, &mut app_exit_events);
        /*
        match menu_selection.selected {
            MainMenuOptions::StartGame => {
                println!("Go to game !");
                start_new_game(app_state, game_state);
            }
            MainMenuOptions::LoadGame => {
                println!("Load a saved game!");
                load_saved_game(app_state, game_state);
            }
            MainMenuOptions::Quit => {
                println!("Quit App");   //TODO
                app_exit_events.send(AppExit);
            }
        }
        */
    }
}

pub fn menu_input_mouse(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    //mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    //mut mouse_wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window>,
    button_query: Query<(&MainMenuClickable, &Transform)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut menu_selection: ResMut<MainMenuSelection>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>
) {
    for event in mouse_button_input_events.read() {
        //sr_rl::menus::menus_input: MouseButtonInput { button: Left, state: Pressed }
        if event.button == MouseButton::Left && event.state == ButtonState::Released {
            main_menu_selecting(menu_selection.selected, &mut app_state, &mut game_state, &mut app_exit_events);
        }
        info!("{:?}", event);
    }
    
    /* 
    for event in mouse_motion_events.iter() {
        //info!("{:?}", event);
    }
    */
    for _event in cursor_moved_events.read() {
        // Needed to convert cursor position on window to World coords.
        let (camera, camera_transform) = camera_q.single();
        if let Some(world_position) = window_query.single().cursor_position() 
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())      {
            // Cursor in window
            let cursor_world_position = Vec3 {x:world_position.x, y:world_position.y, z:0.0};
            /* TO DELETE : clic on sprite in menu. Obsolete. 
            for (clickable, transform) in button_query.iter() {
                if mouse_on_clickable(cursor_world_position, transform.translation, clickable.size) {
                    menu_selection.selected = clickable.id;
                    //println!("Button selected: {:?}", menu_selection.selected)
                }
                //println!("Mouse collide between {:?} and {:?} ? : {:?}", world_position, transform.translation, mouse_on_clickable(cursor_world_position, transform.translation, clickable.size));
            }
            */
        }
        //info!("{:?}", event);
    }
    /*
    for event in mouse_wheel_events.iter() {
        //info!("{:?}", event);
    }
    */
}
*/

/* TO REMOVE  
pub fn mouse_on_clickable(
    target_pos: Vec3,
    some_translation: Vec3,
    some_size: Vec2
) -> bool {
    //println!("Collision check : Size is {:?} vs {:?}", Vec2::splat(1.0), some_size);

    let collision = collide(
        target_pos,
        Vec2::splat(1.0), 
        some_translation,
        some_size * CHAR_SIZE * 0.9 // TODO : Ugly fix for two buttons too close.
    );
    collision.is_some()
}
*/
