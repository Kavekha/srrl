// Part of the refacto: what is commons to all menus
// TEMP

//Resume (ig menu) not working
//Music de combat reste quand retour au main menu depuis IG Menu.
//Reverifier coherence des menus.

//== DOCUMENTATION
// On commence en MenuState:Splashscreen, GameState:Disabled.
// Transmets un MessageEvent pour Afficher le Main Menu. Cet Event passe MenuState en Open.
// Le button_system contient tous les ordres de circulation dans les Menus, et est disponible dés que MenuState::Open.
// La gestion du ClearMenu reste pas ouf, il faut bien penser à l'ajouter à chaque fois. La mettre dans le Open semble faire le Clear après l'envoi du Menu... -_-

//== AMELIORATIONS A FAIRE:
// Encore beaucoup de doublons entre IG & MainMenu, à cause de la circulation. Peut être enregistrer l'option "Previous" dans le Menu à chaque fois?
// Desactiver les Controles "IG" / mettre GameState en Unavailable pendant le IG menu.



use bevy::prelude::*;

use crate::{commons::despawn_component, engine::asset_loaders::GraphicsAssets, game::states::MenuState, globals::{HEIGHT, RESOLUTION}, menu_builders::spawn_menu};

use self::{components::{OnScreenMenu, ResolutionSettings, SelectedOption}, menu_char_selection::{components::PlayerCreation, select_char_menu::selecting_kind}, menu_systems::{common_menu_action, splashscreen}};

use super::Menu;

pub mod components;
pub mod menu_systems;
pub mod menu_char_selection;



// Menu colors
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);          // TODO : Même couleur que le fond si on veut le cacher. Defaut background button est blanc.
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);



pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ResolutionSettings{
            low:Vec2::new(640.0, 360.0),
            medium:Vec2::new(HEIGHT * RESOLUTION, HEIGHT),
            high:Vec2::new(1920.0, 1080.0)
        })
        .insert_resource(PlayerCreation::new())
        .add_event::<MenuEvent>()
        .add_systems(Update, menu_event_reader.run_if(on_event::<MenuEvent>()))   


        .init_state::<MenuState>()

        //On fait ça pour utiliser la commande d'envoi du MainMenu, car on ne peut se baser sur le MenuState::Open.
        .add_systems(OnEnter(MenuState::Splashscreen), splashscreen)    
        .add_systems(OnEnter(MenuState::Splashscreen), menu_camera)  

        //.add_systems(OnEnter(MenuState::Open), menu_camera)
        .add_systems(Update, button_system.run_if(not(in_state(MenuState::Disabled))))
        .add_systems(Update, common_menu_action.run_if(not(in_state(MenuState::Disabled))))  // La gestion des actions IG Menu.
       
       .add_systems(Update, selecting_kind.run_if(not(in_state(MenuState::Disabled))))
             
        //Specific IG Menu            

        ;        
    }
}


// AMELIORATION : Refaire, car pas souple du tout. Ca construit le Menu par procuration, car on recoit un Event depuis World. C'est très moche.
#[derive(Event)]
pub struct MenuEvent{
    pub menu: Menu,
    pub menu_type: MenuType     //Type pour savoir quel menu on créé? Au cas où pôur le moment.
}

pub enum MenuType {
    RECAPMENU,
    MAINMENU,
    SETTINGS,
    DISPLAY,
    QUIT,
    AUDIO
}

fn menu_event_reader(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuEvent>,
    graph_assets: Res<GraphicsAssets>,
) {
    for event in ev_menu.read() {
        //println!("Je suis dans Menu Event Reader avec pour type: {:?}.", event.menu_type);
        println!("Menu reçu et envoyé.");
        let menu = &event.menu;
        spawn_menu(&mut commands, graph_assets, menu);
        break;      // Degueu, mais seul le premier m'interesse et c peu probable que j'en ai d'autres.
    }    
}


pub fn clean_menu(
    mut commands: Commands,
    despawn_onscreenmenu: Query<Entity, With<OnScreenMenu>>,
) {
    println!("Cleaning menu");
    despawn_component(despawn_onscreenmenu, &mut commands);
}

/// Camera centré sur 0.0,0.0 pour ne pas avoir contenu des menus off screen.
pub fn menu_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    println!("menu camera: ON");
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}


// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
