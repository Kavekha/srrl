// ==> DOCUMENTATION 0.19
/*
InGame, player_mouse_input regarde si la mouse a des events et envoi un RefreshActionCostEvent ==> Utilité? 
ingame, combat_input permets:
    - De passer le tour (consomme tous les AP),
    - Faire un clic sur la map.
        - Enregistre la position de la tuile cliquée.
        - Selon le CursorMode (Rien a faire là)
            - Annonce l'intention de se deplacer (Ou attaquer si tuile occupée par un combattant, les deux se melangent encore)
            - Annonce l'intention d'une attaque à distance.
InGame, player_choose_action_input permets de choisir le mode d'attaque, enregistré dans le CursorMode pour aucune bonne raison:
    - Attaque Melee / Move.
    - Attaque à distance.
InGame, ig_call_menu_input & ig_inside_menu_input permettent d'appeler ou quitter le IG Menu, en passant le GameState a Unaivailable. Devrait retirer l'UI "in game" du Curseur.

camera_smooth_follow fait que la camera suit le joueur.
    - Pas vraiment un input.
    - Pas vraiment un comportement genial: la camera reste très clucky. On devrait pouvoir deplacer la camera soit-même, en l'eloignant du personnage si on le souhaite.

exit_step_check est aussi un element hors input joueur, qui regarde si le joueur a atteint la sortie.
    - Devrait être un clic sur la sortie?
    - Devrait sortir de là.

mouse_scroll fait scroller le menu "Log" a l'ecran de Recap.
    - On devrait d'ailleurs avoir accès aux logs depuis le IG.
 */

use bevy::prelude::*;

mod camera_exit;
mod player_inputs;
pub mod components;
pub mod cursor;

pub use camera_exit::camera_center_on_player;
pub use components::Player;
pub use cursor::Cursor;
pub use cursor::cursor_position;

use self::{camera_exit::{camera_smooth_follow, exit_step_check}, components::{OnClickEvent, WantToMoveEvent}, player_inputs::{combat_input, debug_info_on_click, ig_call_menu_input, ig_inside_menu_input, mouse_scroll, player_choose_action_input, player_mouse_input}};

use crate::game::states::GameState;

use super::combat::{events::WantToHitEvent, CombatSet};


 


pub struct PlayerPlugin;

//TODO : Input instead maybe? 
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app          
            .add_event::<PlayerInputReadyEvent>()
            .add_event::<OnClickEvent>()               // Joueur clique: Attaque ou mouvement?    
            .add_event::<WantToHitEvent>()  // 0.19b
            .add_event::<WantToMoveEvent>()  // 0.19b
            
            .add_systems(Update, player_mouse_input.run_if(in_state(GameState::Running)))   

            // 0.19b
            .add_systems(Update, combat_input.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, player_choose_action_input.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic)) 

            .add_systems(Update, ig_call_menu_input.run_if(in_state(GameState::Running)))   // Appeler IG Menu si In Game.            
            .add_systems(Update, ig_inside_menu_input.run_if(in_state(GameState::Unavailable)))     // TODO : Put the game In Unavailable quand Menu Open 
            
            //.add_systems(Update, camera_center_on_player.run_if(in_state(GameState::Running)))
            .add_systems(Update, camera_smooth_follow.run_if(in_state(GameState::Running)))            
            .add_systems(Update, exit_step_check.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick))

            .add_systems(Update, mouse_scroll)

            //DEBUG
            .add_systems(Update, debug_info_on_click)
            ;
    }
}


#[derive(Event)]
pub struct PlayerInputReadyEvent;

//0.20a Ne semble plus utilisé.
/*
#[derive(Event)]
struct PlayerActionEvent;
 */