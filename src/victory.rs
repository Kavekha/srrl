use bevy::{prelude::*};

use crate::{
    GameState, despawn_screen,
    ascii::{spawn_ascii_text, AsciiSheet},
    mainmenu::{menu_camera, OnScreenMenu}
};



pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::VictoryScreen), display_victory_screen)
            .add_systems(OnEnter(GameState::VictoryScreen), menu_camera)
            .add_systems(Update, victory_menu_input.run_if(in_state(GameState::VictoryScreen)))
            .add_systems(OnExit(GameState::VictoryScreen), despawn_screen::<OnScreenMenu>); 
    }
}



fn display_victory_screen(
    mut commands: Commands,
    ascii: Res<AsciiSheet>
){
    let victory_message = "VICTORY !";
    let victory_description= "You flee the place.";
    let x: f32 = -0.2; //0.0;
    let mut y: f32 = 0.0;

    let text_to_display = vec![victory_message, victory_description];

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


}


fn victory_menu_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if keys.pressed(KeyCode::Space) {
        println!("Back to game from Victory Screen !");      //TOLOG
        game_state.set(GameState::GameMap);
    }
}