use bevy::{prelude::*};

use crate::{states::GameState, globals::CHAR_SIZE, ascii::spawn_ascii_text, despawn_screen,};

use super::{components::OnScreenMenu, mainmenu::menu_camera, AsciiSheet};



// TODO: Refacto Victory & GameOver en un seul: Recap Screen?

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::GameOverScreen), display_gameover_screen)
            .add_systems(OnEnter(GameState::GameOverScreen), menu_camera)
            .add_systems(Update, gameover_menu_input.run_if(in_state(GameState::GameOverScreen)))
            .add_systems(OnExit(GameState::GameOverScreen), despawn_screen::<OnScreenMenu>);    //TODO : Despawn propre. Render
    }
}



fn display_gameover_screen(
    mut commands: Commands,
    ascii: Res<AsciiSheet>
){
    println!("Afficher YOU DIED"); //DEBUG
    let gameover_message = "YOU DIED.";
    let gameover_description= "A ghoul has eaten you.";
    let mut y: f32 = 0.0;

    let text_to_display = vec![gameover_message, gameover_description];

    for text in text_to_display{
        let x = - (text.len() as f32 / 2.0 * CHAR_SIZE);
        let text_placement = Vec3::new(x, y, 0.0);
        let ascii_text = spawn_ascii_text(
            &mut commands,
            &ascii,
            &text,
            text_placement
        );
        commands.entity(ascii_text)
        .insert(OnScreenMenu);

        y -= 2.0 * CHAR_SIZE;    
    }

}

fn gameover_menu_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        game_state.set(GameState::NewGame);
    }
}