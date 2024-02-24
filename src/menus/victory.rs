use bevy::{prelude::*, input::mouse::MouseButtonInput};

use crate::{states::GameState, globals::CHAR_SIZE, asset_loaders::GraphicsAssets, render::ascii::spawn_ascii_text};

use super::{components::OnScreenMenu, mainmenu::menu_camera, clean_menu};



pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::VictoryScreen), display_victory_screen)
            .add_systems(OnEnter(GameState::VictoryScreen), menu_camera)
            .add_systems(Update, victory_menu_input.run_if(in_state(GameState::VictoryScreen)))
            .add_systems(OnExit(GameState::VictoryScreen), clean_menu); 
    }
}



fn display_victory_screen(
    mut commands: Commands,
    //ascii: Res<AsciiSheet>
    ascii: Res<GraphicsAssets>,
){
    let victory_message = "VICTORY !";
    let victory_description= "You flee the place.";
    //let x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let text_to_display = vec![victory_message, victory_description];

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


fn victory_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,    
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        game_state.set(GameState::NewGame);
    }
    for event in mouse_button_input_events.read() {
        //sr_rl::menus::menus_input: MouseButtonInput { button: Left, state: Pressed }
        if event.button == MouseButton::Left {
            game_state.set(GameState::NewGame);
        }
        info!("{:?}", event);
    }
}