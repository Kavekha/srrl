use bevy::{prelude::*, input::mouse::MouseButtonInput};

use crate::{
    engine::asset_loaders::GraphicsAssets, game::{manager::{MessageEvent, StartGameMessage}, states::GameState} 
    //render::ascii::spawn_ascii_text
};

use super::{
    components::OnScreenMenu, 
    menu_camera, clean_menu};



pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::VictoryScreen), display_victory_screen)
            .add_systems(OnEnter(GameState::VictoryScreen), menu_camera)
            //.add_systems(Update, victory_menu_input.run_if(in_state(GameState::VictoryScreen)))
            .add_systems(Update, end_game_menu_input.run_if(in_state(GameState::VictoryScreen)))            
            .add_systems(OnExit(GameState::VictoryScreen), clean_menu); 
    }
}


fn display_victory_screen(
    mut commands: Commands,
    graph_assets: Res<GraphicsAssets>
) {

    commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                OnScreenMenu
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Victory !",
                    TextStyle {
                        font: graph_assets.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "You flee the place.",
                    TextStyle {
                        font: graph_assets.font.clone(),
                        font_size: 20.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
            });

}



fn victory_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,    
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    // v0.15.2 remove this
    /* 
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Enter]) {
        game_state.set(GameState::NewGame);
    }
    for event in mouse_button_input_events.read() {
        //sr_rl::menus::menus_input: MouseButtonInput { button: Left, state: Pressed }
        if event.button == MouseButton::Left {
            game_state.set(GameState::NewGame);
        }
        info!("{:?}", event);
    }
    */
}

pub fn end_game_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,    
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ev_message: EventWriter<MessageEvent>   //NEW MESSAGE EVENT SYSTEM v0.15.2
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Enter]) {
        //game_state.set(GameState::NewGame);
        ev_message.send(MessageEvent(Box::new(StartGameMessage{step: 0})));      // NEW MESSAGE EVENT SYSTEM v0.15.2
    }
    for event in mouse_button_input_events.read() {
        //sr_rl::menus::menus_input: MouseButtonInput { button: Left, state: Pressed }
        if event.button == MouseButton::Left {
            //game_state.set(GameState::NewGame);
            ev_message.send(MessageEvent(Box::new(StartGameMessage{step:0})));      // NEW MESSAGE EVENT SYSTEM v0.15.2
        }
        info!("{:?}", event);
    }
}


/* 
fn display_victory_screen_old(
    mut commands: Commands,
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
*/
