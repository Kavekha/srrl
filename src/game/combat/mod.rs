/*
== DOCUMENTATION : Combat system. ==

Le combat est déclenché via StartCombatMessage du Manager, avec un Event.

=> combat_start donne des ActionPoints à toute Entité avec Health & Stats, ca devient des combattants.
Le Combat est suivi grace à CombatInfos.
Envoi Event <CombatTurnStartEvent> & <tick>

=> Le CombatTurnStart considère qu'il s'agit d'un nouveau tour.
Il redonne des AP à tout le monde.
Il mets tous les NPC dans la queue.
Il ajoute le joueur.
Envoi Event <CombatTurnNextEntityEvent>.

=> CombatTurnNextEntity determine la nouvelle entité qui doit jouer.
Si aucune: envoi Event <CombatTurnEndEvent> pour conclure le tour.
Sinon prends la suivante, verifie sa validité et lui mets le Component "Turn".

=> combat_turn_entity_check verifie si l'entité actuelle conserve son tour. 
Si elle n'a plus d'AP ou si elle est Frozen, alors on lui retire son tour et on envoit <CombatTurnNextEntityEvent>.
Si il conserve son tour & que c'est un NPC on lui donne un CheckGoal pour qu'il plannifie ses actions.

=> combat_turn_end, recoit l'event de CombatTurnNextEntity quand sa queue est vide.
Relance un nouveau tour.
Regarde si tous les enemis sont morts => Game Over.

=> combat_end
Se lance à la désactivation du jeu. 
N'est pas utilisé pour le moment.
TODO : Gerer la sortie du combat et permettre une animation de victoire / game over?
----

=> combat_input regarde les inputs du joueur.
=> IA gère les décisions des NPC.


On note aussi que l'animation est aussi ici pour les deplacements.


*/


#[derive(SystemSet, Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CombatSet {
    #[default]
    Logic,
    Animation,
    Tick
}

// TODO : pour IA & system pj?
#[derive(SystemSet, Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ActionSet {
    #[default]
    Planning,
    Execute
}



use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod action_infos;
pub mod rules;
pub mod combat_system;
mod ia;




use crate::{engine::animations::events::GraphicsWaitEvent, game::{
        combat::{combat_system::components::ActionPoints, components:: CombatInfos}, manager::{change_state_messages::QuitGameMessage, game_messages::GameOverMessage}, states::GameState 
    }};

use self::{
    action_infos::{update_action_infos, ActionInfos, CharacterAction}, 
    combat_system::{components::{AttackType, IsDead}, CombatSystemPlugin}, 
    components::CurrentEntityTurnQueue, 
    events::{CombatEndEvent, CombatTurnEndEvent, CombatTurnNextEntityEvent, CombatTurnQueue, CombatTurnStartEvent, RefreshActionCostEvent, TickEvent, Turn}, 
    ia::{components::{CheckGoal, Frozen}, IaPlugin}, 
};
use super::{manager::MessageEvent, pieces::components::{Health, Npc, Stats}, player::Player};


pub struct CombatPlugin;

// 0.20o Rework pour répondre au bug du TurnStart relancé en boucle.
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(IaPlugin)
            .add_plugins(CombatSystemPlugin)

            .init_resource::<CombatTurnQueue>()             // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.
            .insert_resource(ActionInfos { available_action: CharacterAction::NONE, cost:None, path: None, target: None, attack: Some(AttackType::MELEE), entity: None })

            .add_event::<CombatTurnStartEvent>()        // Lance le tour.
            .add_event::<CombatTurnNextEntityEvent>()   // Envoyé pour prendre le nouvel acteur.
            .add_event::<CombatTurnEndEvent>()          // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.
            .add_event::<RefreshActionCostEvent>()              // Recalcule le cout d'une action / deplacement.
            .add_event::<TickEvent>()                           // De retour en 0.19j : Donne le rythme en recheckant où en sont les acteurs du combat.
            .add_event::<CombatEndEvent>()              // 0.20o demande la fin du combat.
  
            
            .configure_sets(Update, (CombatSet::Logic, CombatSet::Animation, CombatSet::Tick).chain().run_if(in_state(GameState::Running)))
            .configure_sets(Update, (ActionSet::Planning, ActionSet::Execute).chain().in_set(CombatSet::Logic))

            // Note: On entre en combat avec le STARTCOMBATMESSAGE
            .add_systems(Update, tick.in_set(CombatSet::Tick))      // Si pas d'anim en cours, passe à la suite du process (Consommation AP -> Changement Entité -> Changement Tour)
            .add_systems(Update, update_action_infos.run_if(resource_exists::<CombatInfos>).run_if(on_event::<RefreshActionCostEvent>()))
            // 0.20p
            .add_systems(Update,(
                combat_turn_start.run_if(on_event::<CombatTurnStartEvent>()),
                combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()),
                combat_turn_entity_check.run_if(on_event::<TickEvent>()),
                combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()),
                combat_end.run_if(on_event::<CombatEndEvent>())
            ).chain().run_if(resource_exists::<CombatInfos>))
            /* 
            .add_systems(Update,(
                combat_turn_start.run_if(resource_exists::<CombatInfos>).run_if(on_event::<CombatTurnStartEvent>()),
                combat_turn_next_entity.run_if(resource_exists::<CombatInfos>).run_if(on_event::<CombatTurnNextEntityEvent>()),
                combat_turn_entity_check.run_if(resource_exists::<CombatInfos>).run_if(on_event::<TickEvent>()),
                combat_turn_end.run_if(resource_exists::<CombatInfos>).run_if(on_event::<CombatTurnEndEvent>()),
                combat_end.run_if(on_event::<CombatEndEvent>())
            ).chain())
*/
            

            .add_systems(OnEnter(GameState::Disabled), quit_current_game) 
            ;
    }
}



// 0.19j on remets ce tick qui regarde si on doit attendre la fin des animations.   // TODO audit du fonctionnel animation & l'usage de ce systeme.
// Tick event active la suite du cycle.
fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>
) {
    if ev_wait.read().len() == 0 {
        //info!("tick: send tick event.");
        ev_tick.send(TickEvent);
    } else {
        info!("Graphic event, waiting...");
    }
}

/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
/// // Est utilisé par un Message du Manager.
pub fn combat_start(    
    mut commands: Commands,
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>), Without<IsDead>>,
    mut action_infos: ResMut<ActionInfos>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, _fighter_health, _fighter_stat, _fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 0});
    }
    action_infos.attack = Some(AttackType::MELEE);
    
    commands.insert_resource(CombatInfos {turn: 0, current_entity: None});
    //info!("Combat Start.");
    ev_newturn.send(CombatTurnStartEvent);
    //ev_tick.send(TickEvent);}
}

/// Ajoute les Participants du Turn au Combat dans la queue CombatTurnQueue.
fn combat_turn_start(
    // Obligé d'avoir ses 3 queues à cause de npc_query.iter() qui ajoute les entités presentes dans npc_query dans la queue.
    mut action_query: Query<(Entity, &mut ActionPoints)>,
    npc_query: Query<Entity, (With<ActionPoints>, Without<Player>)>,
    player_query: Query<Entity, (With<ActionPoints>, With<Player>)>, 
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,  
    mut combat_infos: ResMut<CombatInfos>,
    mut ev_refresh_ap: EventWriter<RefreshActionCostEvent>, 
) {
    //info!("Event received: CombatTurnStartEvent");
    
    combat_infos.turn += 1;
    // On redonne les PA à tout le monde.
    for (_entity, mut action_points) in action_query.iter_mut() {
        //println!("{:?} Entity {:?} received full ap.", step, entity);
        action_points.current = action_points.max;
    }
    info!("Turn {:?} : AP have been reset.", combat_infos.turn);
    // On mets à jour le calcul des AP, ce qui rafraichira l'UI.
    //ev_refresh_ap.send(RefreshActionCostEvent);

    // On mets les gens dans la CombatTurnQueue pour ce tour.
    // Npc d'abord
    queue.0.extend(
        npc_query.iter()
    );  
    // Player à la fin pour qu'il joue en premier.
    if let Ok(player) = player_query.get_single() {
        queue.0.insert(0, player);
    }
    ////info!("Combat turn queue has {:?} messages.", queue.0.len());

    // On lance le TurnNextEntity pour faire jouer le premier de la Queue.
    //info!("combat_turn_start send event for CombatTurnNextEntityEvent");
    ev_next.send(CombatTurnNextEntityEvent);
    ev_refresh_ap.send(RefreshActionCostEvent);
}


/// On récupère le prochain combattant, puisque le précédent a fini.
fn combat_turn_next_entity(
    mut commands: Commands,
    mut queue: ResMut<CombatTurnQueue>,    
    action_points_q: Query<(&ActionPoints, Option<&Npc>)>,  // Pas besoin de checker si IsDead, car un mort perds les ActionPoints.
    mut ev_turn_end: EventWriter<CombatTurnEndEvent>,
    mut current_combat: ResMut<CombatInfos>,    
    mut ev_refresh_ap: EventWriter<RefreshActionCostEvent>, 
) {
    ////info!("combat_turn_next_entity: received event <CombatTurnNextEntityEvent>");
    let Some(entity) = queue.0.pop_front() else {
        // Plus de combattant: le tour est fini.
        //info!("combat_turn_next_entity: Plus aucun combattant pour ce tour. Fin du tour => <CombatTurnEndEvent>");
        ev_turn_end.send(CombatTurnEndEvent);
        return;
    };
    // On récupère les informations de l'entité a tjrs des AP et existe tjrs sinon crash.
    let Ok(action_infos ) = action_points_q.get(entity) else { return };
    let (_action_points, is_npc) = action_infos;

    current_combat.current_entity = Some(entity);
    // On lui donne le composant "Turn".
    commands.entity(entity).insert(Turn);   
    info!("It's {:?} turn now.", entity);
    // v0.19h : On doit donner aux NPC le component CheckGoal pour qu'il planifie.

    if is_npc.is_some() {
        commands.entity(entity).insert(CheckGoal);    
    } else {
        ev_refresh_ap.send(RefreshActionCostEvent);
    }   
    //info!("combat_turn_next_entity: finished for {:?}.", entity)    
}

fn combat_turn_end(    
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    mut queue: ResMut<CombatTurnQueue>,
    dead_q: Query<(&IsDead, Option<&Player>)>,
    mut ev_combat_end: EventWriter<CombatEndEvent>,
){
    ////info!("Combat turn End: executed."); 
    let mut player_dead = false;
    for (_death, is_player) in dead_q.iter() {
        if is_player.is_some() {  
            ////info!("Player is dead, Game Over.");
            player_dead = true;
        }
    }
    if player_dead {
        println!("combat_turn_end:Player is dead.");
        ev_combat_end.send(CombatEndEvent); // 0.20o
        //ev_message.send(MessageEvent(Box::new(GameOverMessage)));
        return;
    }
    queue.0.clear();    
    //info!("Combat turn End: Send event CombatTurnStartEvent.");    
    ev_newturn.send(CombatTurnStartEvent); // 

}

/// 0.19j c'est cette fonction qui donne le rythme ! REMEMBER => Elle est très importante.
/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
fn combat_turn_entity_check(
    mut commands: Commands,
    current_combat: ResMut<CombatInfos>,
    query_action_points: Query<(&ActionPoints, Option<&Npc>, Option<&Frozen>)>,  // Frozen => entité qu'on ne veut pas utiliser car non active.
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,  
    mut ev_refresh_ap: EventWriter<RefreshActionCostEvent>,
) {
    ////info!("Combat turn entity: starting.");
    // On recupere l'entité de CombatInfos.
    if let Some(entity) = current_combat.current_entity {
        if let Ok(entity_infos) = query_action_points.get(entity) {
            let (ap_entity, is_npc, is_frozen) = entity_infos;
            if ap_entity.current <= 0 || is_frozen.is_some() {
                ////info!("This entity {:?} has no AP or is Frozen: let's turn to next entity event.", entity);
                commands.entity(entity).remove::<Turn>();                
                //info!("combat_turn_entity_check: send event for CombatTurnNextEntityEvent");
                ev_next.send(CombatTurnNextEntityEvent);
                return 
            }
            if is_npc.is_some() {
            // 0.19h: Pour le NPC, on lui redemande de CheckGoal
            commands.entity(entity).insert(CheckGoal);
           } else {
                ev_refresh_ap.send(RefreshActionCostEvent);
           }
        }
        ////info!("Combat turn entity : Entity checked was {:?}.", entity);
    }
}

/// Retire les ActionPoints, Remove CombatInfos, change State.
/// Sera utilisable par le Manager.
/// NOTE : On constate que quitter une partie via menu: la boucle d'event CombatTurnStartEvent continue car rien n'y mets fin.
pub fn combat_end(
    mut commands: Commands,
    fighters: Query<(Entity, &ActionPoints)>,
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_message: EventWriter<MessageEvent>,
){
    for (entity, _fighter) in fighters.iter() {
        commands.entity(entity).remove::<ActionPoints>();
    }
    commands.remove_resource::<CombatInfos>();
    queue.0.clear();
    println!("Combat end!");
    ev_message.send(MessageEvent(Box::new(GameOverMessage)));
}

// v0.20o : A terme, a deplacer ailleurs.
pub fn quit_current_game(
    mut commands: Commands,
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_message: EventWriter<MessageEvent>,
){    
    commands.remove_resource::<CombatInfos>();
    queue.0.clear();
    info!("Quit current game");
    ev_message.send(MessageEvent(Box::new(QuitGameMessage)));
}
