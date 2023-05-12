use bevy::{prelude::*};

use crate::{
    GameState, despawn_screen,
    ascii::{spawn_ascii_text, AsciiSheet},
    mainmenu::{menu_camera, OnScreenMenu}
};


// TODO: Refacto Victory & GameOver en un seul: Recap Screen?

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::GameOverScreen), display_gameover_screen)
            .add_systems(OnEnter(GameState::GameOverScreen), menu_camera)
            .add_systems(Update, gameover_menu_input.run_if(in_state(GameState::GameOverScreen)))
            .add_systems(OnExit(GameState::GameOverScreen), despawn_screen::<OnScreenMenu>); 
    }
}



fn display_gameover_screen(
    mut commands: Commands,
    ascii: Res<AsciiSheet>
){
    println!("Afficher YOU DIED"); //DEBUG
    let gameover_message = "YOU DIED.";
    let gameover_description= "A ghoul has eaten you.";
    let x: f32 = -0.2; //0.0;
    let mut y: f32 = 0.0;

    let text_to_display = vec![gameover_message, gameover_description];

    for text in text_to_display{
        //TODO : Ce qui suit est dégueu, et utilisé pour centrer le texte.
        //let ox = (text.len()/2) as f32; 
        //let mo = ox /10.0;      // Je le divise par 10 car trop gros mais impossible à diviser par 20 auparavant car arrondi à 0.
        //let final_x: f32 = x - mo;
        
        //let text_placement= Vec3::new(final_x, y, 0.0);
        let text_placement = Vec3::new(x, y, 0.0);
        let ascii_text = spawn_ascii_text(
            &mut commands,
            &ascii,
            &text,
            text_placement
        );
        commands.entity(ascii_text)
        .insert(OnScreenMenu);

        y -= 0.1    //0.2
    }
    println!("Le message est affiché");


}

fn gameover_menu_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        println!("Back to game from Game Over Screen !");      //TOLOG
        game_state.set(GameState::NewGame);
    }
    println!("Dans le Game Over Screen!")
}