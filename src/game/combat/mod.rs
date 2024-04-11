/*
== DOCUMENTATION ==

Le combat est déclenché via StartCombatMessage du Manager.
On donne des ActionPoints à toute Entité avec Health & Stats, ca devient des combattants.
Le Combat est suivi grace à CombatInfos.
=> CombatTurnStart.

Le CombatTurnStart redonne des AP à tout le monde.
NOTE: Mise à jour UI devrait pouvoir se faire autrement.
On mets tous les NPC dans la Queue puis le Player. Le dernier ajouté est le premier à jouer. 
=> CombatTurnNextEntityEvent

CombatTurnNextEntityEvent fait jouer le premier Entité de la Queue.
Regarde si elle est vivante ou a des PA. Si plus aucun, retire l'Entité de la Queue.
Si elle en a, lui donne le component "Turn" pour qu'on sache qu'elle peut jouer.
Si plus d'Entité, le Tour est fini. 
=> combat_turn_end.

combat_turn_entity_check regarde si y a toujours des PA pour que l'Entité joue. Sinon passe son tour.

combat_turn_end s'assure que la queue est vide.
Relance ensuite le CombatTurnStart après incrementation du CombatInfos.
NOTE: C'est ici que l'on verifierait si tous les enemis sont morts / autres critères de fin de combat.
=> CombatTurnStart

combat_input regarde les inputs du joueur.

npc_planning prends des décisions pour les NPC. 
NOTE: C'est sans doute ici qu'on gerera l'IA.

On note aussi que l'animation est aussi ici pour les deplacements.


*/


#[derive(SystemSet, Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CombatSet {
    #[default]
    Logic,
    Animation,
    Tick
}


use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod action_infos;
pub mod rules;
mod ia;
mod intent_systems;



use crate::{engine::animations::events::GraphicsWaitEvent, game::{
        combat::components::{ActionPoints, CombatInfos}, manager::game_messages::GameOverMessage, states::GameState 
    }};

use self::{
    action_infos::{update_action_infos, ActionInfos}, 
    components::{CurrentEntityTurnQueue, IsDead}, 
    intent_systems::{entity_dies, entity_get_hit, entity_miss_attack, entity_try_hit, entity_want_forfeit, entity_want_hit, on_event_entity_want_hit}, 
    events::{CombatTurnEndEvent, CombatTurnNextEntityEvent, CombatTurnQueue, CombatTurnStartEvent, RefreshActionCostEvent, TickEvent, Turn}, 
    ia::{components::{CheckGoal, Frozen}, IaPlugin}
};
use super::{manager::MessageEvent, pieces::components::{Health, Npc, Stats}, player::Player, ui::ReloadUiEvent};


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(IaPlugin)

            .init_resource::<CombatTurnQueue>()             // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.
            .insert_resource(ActionInfos { cost:None, path: None, target: None, entity: None, attack: None })

            .add_event::<CombatTurnStartEvent>()        // Lance le tour.
            .add_event::<CombatTurnNextEntityEvent>()   // Envoyé pour prendre le nouvel acteur.
            .add_event::<CombatTurnEndEvent>()          // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.
            .add_event::<RefreshActionCostEvent>()              // Recalcule le cout d'une action / deplacement.
            .add_event::<TickEvent>()                           // De retour en 0.19j : Donne le rythme en recheckant où en sont les acteurs du combat.
  
            .configure_sets(Update, CombatSet::Logic)      
            .configure_sets(Update, CombatSet::Tick.after(CombatSet::Logic))
            .configure_sets(Update, CombatSet::Animation.after(CombatSet::Tick))      
            
            // Init Combat.
            //USE STARTCOMBATMESSAGE 0.15.4      // On lance le Combat dés l'arrivée en jeu. //TODO : Gestion de l'entrée / sortie en combat.

           // Le tour commence.
           .add_systems(Update, combat_turn_start.run_if(on_event::<CombatTurnStartEvent>()).in_set(CombatSet::Logic))
           // On prends l'entité dont c'est le tour. On passe en TurnUpdate
           .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()).after(combat_turn_start).in_set(CombatSet::Logic))
            // toutes les entités ont fait leur tour.
            .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()).after(combat_turn_next_entity).in_set(CombatSet::Logic))
            .add_systems(Update, combat_player_death.after(combat_turn_end).in_set(CombatSet::Logic))         

            // 0.19b back to component. 
            .add_systems(Update, on_event_entity_want_hit.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick)) 
            .add_systems(Update, entity_want_hit.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(on_event_entity_want_hit))
            .add_systems(Update, entity_try_hit.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(entity_want_hit))
            .add_systems(Update, entity_miss_attack.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(entity_try_hit))
            .add_systems(Update, entity_get_hit.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(entity_try_hit))
            .add_systems(Update, entity_dies.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(entity_get_hit))               
            .add_systems(Update, entity_want_forfeit.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick))
  
            // Check de la situation PA-wise. Mise à jour.
            .add_systems(Update, tick.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick))
            .add_systems(Update, combat_turn_entity_check.run_if(on_event::<TickEvent>())) //was Logic, mit dans Tick. v0.19h
            .add_systems(Update, update_action_infos.run_if(resource_exists::<CombatInfos>).run_if(on_event::<RefreshActionCostEvent>()).in_set(CombatSet::Tick))

            // TODO: Quitter le combat. PLACEHOLDER.
            .add_systems(OnEnter(GameState::Disabled), combat_end)
                
            ;
    }
}


fn combat_player_death(
    mut ev_message: EventWriter<MessageEvent>,   //NEW MESSAGE EVENT SYSTEM v0.15.2
    dead_q: Query<(&IsDead, Option<&Player>)>
){
    for (_death, is_player) in dead_q.iter() {
        if is_player.is_some() {  
            ev_message.send(MessageEvent(Box::new(GameOverMessage)));
        }
    }
}

// 0.19j on remets ce tick qui regarde si on doit attendre la fin des animations.   // TODO audit du fonctionnel animation & l'usage de ce systeme.
// Tick event active la suite du cycle.
fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>
) {
    if ev_wait.read().len() == 0 {
        ev_tick.send(TickEvent);
    }
}

/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
/// // Est utilisé par un Message du Manager.
pub fn combat_start(    
    mut commands: Commands,
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>), Without<IsDead>>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, _fighter_health, _fighter_stat, _fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 0});
        println!("Action points added for {:?}", fighter_id);
    }
    commands.insert_resource(CombatInfos {turn: 0, current_entity: None});
    //combat_state.set(CombatState::StartTurn);
    ev_newturn.send(CombatTurnStartEvent);
    println!("Combat start!");
}


/// Ajoute les Participants du Turn au Combat dans la queue CombatTurnQueue.
fn combat_turn_start(
    // Obligé d'avoir ses 3 queues à cause de npc_query.iter() qui ajoute les entités presentes dans npc_query dans la queue.
    mut action_query: Query<(Entity, &mut ActionPoints)>,
    npc_query: Query<Entity, (With<ActionPoints>, Without<Player>)>,
    player_query: Query<Entity, (With<ActionPoints>, With<Player>)>,
 
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    // On redonne les PA à tout le monde.
    println!("Combat turn start");
    for (_entity, mut action_points) in action_query.iter_mut() {
        action_points.current = action_points.max;
    }
    // On mets à jour l'interface pour les AP du joueur.
    ev_interface.send(ReloadUiEvent);

    // On mets les gens dans la CombatTurnQueue pour ce tour.
    // Npc d'abord
    queue.0.extend(
        npc_query.iter()
    );  
    // Player à la fin pour qu'il joue en premier.
    if let Ok(player) = player_query.get_single() {
        queue.0.insert(0, player);
    }
    println!("Combat turn queue has {:?} messages.", queue.0.len());

    // On lance le TurnNextEntity pour faire jouer le premier de la Queue.
    println!("Sending Next Entity");
    ev_next.send(CombatTurnNextEntityEvent);
}


/// On récupère le prochain combattant, puisque le précédent a fini.
fn combat_turn_next_entity(
    mut commands: Commands,
    mut queue: ResMut<CombatTurnQueue>,    
    action_points_q: Query<&ActionPoints>,
    mut ev_turn_end: EventWriter<CombatTurnEndEvent>,
    mut current_combat: ResMut<CombatInfos>,
    mut ev_refresh_ap: EventWriter<RefreshActionCostEvent>,  
    npc_q: Query<&Npc>,  // 0.19h
) {
    let Some(entity) = queue.0.pop_front() else {
        // Plus de combattant: le tour est fini.
        println!("Combat Turn Next Entity: Plus de combattants dans la Queue.");        
        ev_turn_end.send(CombatTurnEndEvent);
        return;
    };

    // On mets à jour CombatInfos pour savoir qui est l'entité dont c'est le Tour.
    // Check pour voir si Entité existe tjrs, sinon crash. //TODO: Facon plus logique?
    let Ok(_action_points) = action_points_q.get(entity) else { return };
    current_combat.current_entity = Some(entity);

    // On lui donne le composant "Turn".
    commands.entity(entity).insert(Turn);   
    // v0.19h : On doit donner aux NPC le component CheckGoal pour qu'il planifie.
    if let Ok(_is_npc) = npc_q.get(entity) {
        commands.entity(entity).insert(CheckGoal);    
    };
    ev_refresh_ap.send(RefreshActionCostEvent);
}

fn combat_turn_end(    
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    mut queue: ResMut<CombatTurnQueue>,
){
    println!("Combat turn End.");    
    queue.0.clear();
    ev_newturn.send(CombatTurnStartEvent);
}


/// 0.19j c'est cette fonction qui donne le rythme ! REMEMBER => Elle est très importante.
/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
fn combat_turn_entity_check(
    mut commands: Commands,
    current_combat: ResMut<CombatInfos>,
    query_action_points: Query<(&ActionPoints, Option<&Player>, Option<&Frozen>)>,  // Frozen => entité qu'on ne veut pas utiliser car non active.
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,  
    //mut ev_tick: EventReader<TickEvent>
) {
    //println!("Combat turn entity check...");
    // On recupere l'entité de CombatInfos.
    if let Some(entity) = current_combat.current_entity {
        //println!("There is a current entity in CombatInfos");
        if let Ok(entity_infos) = query_action_points.get(entity) {
            //println!("This entity has action points.");
            let (ap_entity, is_player, is_frozen) = entity_infos;
            //If no AP anymore, next entity turn.
            if ap_entity.current <= 0 || is_frozen.is_some() {
                println!("This entity {:?} has no AP or is Frozen: let's turn to next entity event.", entity);
                commands.entity(entity).remove::<Turn>();
                ev_next.send(CombatTurnNextEntityEvent);
           } else if is_player.is_some() {
                //println!("This entity has AP and is the Player.");
                //combat_state.set(CombatState::PlayerTurn);
           } else {
            // 0.19h: Pour le NPC, on lui redemande de CheckGoal
            commands.entity(entity).insert(CheckGoal);
            println!("NPC {:?} : has done some action, will check their goal.", entity);
           }
        }
       //println!("Turn Entity check: {:?} turn.", entity);
    }    
}

/// Retire les ActionPoints, Remove CombatInfos, change State.
/// Sera utilisable par le Manager.
pub fn combat_end(
    mut commands: Commands,
    fighters: Query<(Entity, &ActionPoints)>,
    mut queue: ResMut<CombatTurnQueue>,
){
    for (entity, _fighter) in fighters.iter() {
        commands.entity(entity).remove::<ActionPoints>();
    }
    commands.remove_resource::<CombatInfos>();
    //combat_state.set(CombatState::None);
    queue.0.clear();
    println!("Combat end!");
}

