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
TODO: Regarder si tous les enemis sont morts / game over ici?

=> combat_end
Se lance à la désactivation du jeu. 
N'est pas utilisé pour le moment.
TODO : Gerer la sortie du combat et permettre une animation de victoire / game over?

=> combat_player_death
le game over se gère ici pour le moment.
TODO: Devrait plutot être géré dans le CombatEnd?

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
        combat::{combat_system::components::ActionPoints, components:: CombatInfos}, manager::game_messages::GameOverMessage, states::GameState 
    }};

use self::{
    action_infos::{update_action_infos, ActionInfos}, 
    combat_system::{components::IsDead, CombatSystemPlugin}, 
    components::CurrentEntityTurnQueue, 
    events::{CombatTurnEndEvent, CombatTurnNextEntityEvent, CombatTurnQueue, CombatTurnStartEvent, RefreshActionCostEvent, TickEvent, Turn}, 
    ia::{components::{CheckGoal, Frozen}, IaPlugin}, 
};
use super::{manager::MessageEvent, pieces::components::{Health, Npc, Stats}, player::Player, ui::events::ReloadUiEvent};


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(IaPlugin)
            .add_plugins(CombatSystemPlugin)

            .init_resource::<CombatTurnQueue>()             // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.
            .insert_resource(ActionInfos { cost:None, path: None, target: None, entity: None, attack: None })

            .add_event::<CombatTurnStartEvent>()        // Lance le tour.
            .add_event::<CombatTurnNextEntityEvent>()   // Envoyé pour prendre le nouvel acteur.
            .add_event::<CombatTurnEndEvent>()          // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.
            .add_event::<RefreshActionCostEvent>()              // Recalcule le cout d'une action / deplacement.
            .add_event::<TickEvent>()                           // De retour en 0.19j : Donne le rythme en recheckant où en sont les acteurs du combat.
  
            // 0.20c : refacto configure sets. Logic => Calcul, Animation => Affiche, Tick => Relance la machine. NOTE: Si OnEvent + in_set c'est ignoré. WHY?
            .configure_sets(Update, (CombatSet::Logic, CombatSet::Animation, CombatSet::Tick).chain().run_if(in_state(GameState::Running)))
            .configure_sets(Update, (ActionSet::Planning, ActionSet::Execute).chain().in_set(CombatSet::Logic))

            // Init Combat.
            //USE STARTCOMBATMESSAGE 0.15.4      // On lance le Combat dés l'arrivée en jeu. //TODO : Gestion de l'entrée / sortie en combat.
           // Le tour commence. 0.20c
           .add_systems(Update, combat_turn_start.run_if(on_event::<CombatTurnStartEvent>()))
           // On prends l'entité dont c'est le tour. On passe en TurnUpdate
           .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()))
           // toutes les entités ont fait leur tour.   
           .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()))      
              // Check de la situation PA-wise. Mise à jour.
            .add_systems(Update, combat_turn_entity_check.run_if(resource_exists::<CombatInfos>).run_if(on_event::<TickEvent>())) 
            .add_systems(Update, tick.in_set(CombatSet::Logic))
            .add_systems(Update, update_action_infos.run_if(resource_exists::<CombatInfos>).run_if(on_event::<RefreshActionCostEvent>()))
            // TODO: Quitter le combat. PLACEHOLDER.
            .add_systems(OnEnter(GameState::Disabled), combat_end) 
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
    }
}

/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
/// // Est utilisé par un Message du Manager.
pub fn combat_start(    
    mut commands: Commands,
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>), Without<IsDead>>,
    mut ev_tick: EventWriter<TickEvent>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, _fighter_health, _fighter_stat, _fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 0});
    }
    
    commands.insert_resource(CombatInfos {turn: 0, current_entity: None});
    //combat_state.set(CombatState::StartTurn);
    ev_newturn.send(CombatTurnStartEvent);
    println!("Combat start!");
    //info!("Combat Start.");
    ev_tick.send(TickEvent);}


/// Ajoute les Participants du Turn au Combat dans la queue CombatTurnQueue.
/// TODO : Avec un seul NPC + PJ, un tour semble donner x2 fois "X received full ap", et parfois encore pluss! (apres un forfeit?)
fn combat_turn_start(
    // Obligé d'avoir ses 3 queues à cause de npc_query.iter() qui ajoute les entités presentes dans npc_query dans la queue.
    mut action_query: Query<(Entity, &mut ActionPoints)>,
    npc_query: Query<Entity, (With<ActionPoints>, Without<Player>)>,
    player_query: Query<Entity, (With<ActionPoints>, With<Player>)>,
 
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    info!("Event received: CombatTurnStartEvent");
    // On redonne les PA à tout le monde.
    let mut step = 0;
    for (entity, mut action_points) in action_query.iter_mut() {
        step += 1;
        println!("{:?} Entity {:?} received full ap.", step, entity);
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
    //info!("Combat turn queue has {:?} messages.", queue.0.len());

    // On lance le TurnNextEntity pour faire jouer le premier de la Queue.
    //info!("combat_turn_start send event for CombatTurnNextEntityEvent");
    //println!("Sending Next Entity");
    ev_next.send(CombatTurnNextEntityEvent);
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
    //info!("combat_turn_next_entity: received event <CombatTurnNextEntityEvent>");
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
    // v0.19h : On doit donner aux NPC le component CheckGoal pour qu'il planifie.

    if is_npc.is_some() {
        commands.entity(entity).insert(CheckGoal);    
    };
    ev_refresh_ap.send(RefreshActionCostEvent);
 
    //info!("combat_turn_next_entity: finished for {:?}.", entity)
}

fn combat_turn_end(    
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_message: EventWriter<MessageEvent>,
    dead_q: Query<(&IsDead, Option<&Player>)>
){
    //info!("Combat turn End: executed."); 
    let mut player_dead = false;
    for (_death, is_player) in dead_q.iter() {
        if is_player.is_some() {  
            //info!("Player is dead, Game Over.");
            player_dead = true;
        }
    }
    if player_dead {
        ev_message.send(MessageEvent(Box::new(GameOverMessage)));
        return
    }
    queue.0.clear();
    ev_newturn.send(CombatTurnStartEvent);
    //info!("Combat turn End: Send event CombatTurnStartEvent.");    

}

/// 0.19j c'est cette fonction qui donne le rythme ! REMEMBER => Elle est très importante.
/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
fn combat_turn_entity_check(
    mut commands: Commands,
    current_combat: ResMut<CombatInfos>,
    query_action_points: Query<(&ActionPoints, Option<&Npc>, Option<&Frozen>)>,  // Frozen => entité qu'on ne veut pas utiliser car non active.
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,  
    //mut ev_tick: EventReader<TickEvent>
) {
    //info!("Combat turn entity: starting.");
    // On recupere l'entité de CombatInfos.
    if let Some(entity) = current_combat.current_entity {
        if let Ok(entity_infos) = query_action_points.get(entity) {
            let (ap_entity, is_npc, is_frozen) = entity_infos;
            if ap_entity.current <= 0 || is_frozen.is_some() {
                //info!("This entity {:?} has no AP or is Frozen: let's turn to next entity event.", entity);
                commands.entity(entity).remove::<Turn>();
                ev_next.send(CombatTurnNextEntityEvent);
                return 
            }
            if is_npc.is_some() {
            // 0.19h: Pour le NPC, on lui redemande de CheckGoal
            commands.entity(entity).insert(CheckGoal);
            //println!("NPC {:?} : has done some action, will check their goal.", entity);
           }
           //info!("Combat turn entity : treated.");
        } else {
            //info!("!!! Combat turn entity: current_entity n'a pas de points d'Actions.");
        }
        //info!("Combat turn entity : Entity checked was {:?}.", entity);
    } else {
        //info!("!!! Combat turn entity : Pas d'entité disponible dans current_combat.current_entity");
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

