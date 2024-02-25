use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod event_systems;  //TODO deplacer les elements publiques?
mod npc_planning_systems;

use crate::{states::{GameState, EngineState}, game::combat::{components::{ActionPoints, CombatInfos}, events::AnimateEvent}, render::pieces_render::path_animator_update};

use self::{
    events::{CombatTurnQueue, CombatTurnStartEvent, CombatTurnNextEntityEvent, CombatTurnEndEvent, EntityEndTurnEvent, Turn, EntityMoveEvent, EntityTryMoveEvent, OnClickEvent, EntityHitTryEvent, EntityGetHitEvent, EntityDeathEvent, RefreshActionCostEvent}, 
    components::CurrentEntityTurnQueue, 
    event_systems::{action_entity_try_move, action_entity_move, action_entity_end_turn, walk_combat_animation, on_click_action, action_entity_try_attack, action_entity_get_hit, entity_dies, ActionInfos, create_action_infos}, npc_planning_systems::npc_planning
};

use super::{pieces::components::{Health, Stats}, player::{Player, Cursor}, ui::ReloadUiEvent};




pub const AP_COST_MOVE:u32 = 1;
pub const AP_COST_MELEE:u32 = 3;


#[derive(SystemSet, Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CombatSet {
    #[default]
    Logic,
    Animation,
    Tick
}


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        //SECONDE REFACTO
            .init_resource::<CombatTurnQueue>()             // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.
            .insert_resource(ActionInfos { cost:None, path: None, target: None, entity: None })

            .add_event::<CombatTurnStartEvent>()        // Lance le tour.
            .add_event::<CombatTurnNextEntityEvent>()   // Envoyé pour prendre le nouvel acteur.
            .add_event::<CombatTurnEndEvent>()          // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.
            .add_event::<RefreshActionCostEvent>()              // Recalcule le cout d'une action / deplacement.

            .add_event::<EntityEndTurnEvent>()         // Envoyé par l'Entité qui mets volontairement fin à son tour.    //TODO : Meilleur nom: c'est une Action d'un NPC. 
            .add_event::<OnClickEvent>()               // Joueur clique: Attaque ou mouvement?                
            .add_event::<EntityTryMoveEvent>()         // Tente deplacement: check si target ou simple mouvement.
            .add_event::<EntityMoveEvent>()            // Se deplace.
            .add_event::<EntityHitTryEvent>()          // Entity tente d'attaquer.
            .add_event::<EntityGetHitEvent>()          // Entity subit des degats d'une source.
            .add_event::<EntityDeathEvent>()           // L'entité vient de mourir: on transforme son corps et retire les composants.
            
   
            .add_event::<AnimateEvent>()    //Animation //TODO : Deplacer.

            .configure_sets(Update, CombatSet::Logic)      
            .configure_sets(Update, CombatSet::Tick.after(CombatSet::Logic))
            .configure_sets(Update, CombatSet::Animation.after(CombatSet::Tick))      
            
            
            
            // Init Combat.
            .add_systems(OnEnter(GameState::GameMap), combat_start)      // On lance le Combat dés l'arrivée en jeu. //TODO : Gestion de l'entrée / sortie en combat.
           // Le tour commence.
           .add_systems(Update, combat_turn_start.run_if(on_event::<CombatTurnStartEvent>()).in_set(CombatSet::Logic))
           // On prends l'entité dont c'est le tour. On passe en TurnUpdate
           .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()).after(combat_turn_start).in_set(CombatSet::Logic))
           
            // toutes les entités ont fait leur tour.
            .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()).after(combat_turn_next_entity).in_set(CombatSet::Logic))

            // Generation des actions à faire.
            .add_systems(Update, combat_input.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Logic))
            .add_systems(Update, on_click_action.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Logic).after(combat_input))
            
            // Plan NPC
            //.add_systems(Update, plan_action_forfeit.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Logic))
            .add_systems(Update, npc_planning.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Logic))
            
            // Check des actions demandées.
            .add_systems(Update, action_entity_try_move.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Logic))
            
            // Gestion des actions demandées.
            .add_systems(Update, action_entity_end_turn.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Tick))
            .add_systems(Update, action_entity_move.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Tick).after(action_entity_try_move))
            .add_systems(Update, action_entity_try_attack.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Tick).after(action_entity_try_move))
            .add_systems(Update, action_entity_get_hit.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Tick).after(action_entity_try_attack))
            .add_systems(Update, entity_dies.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Tick).after(action_entity_get_hit))
 

            // Check de la situation PA-wise.
            .add_systems(Update, combat_turn_entity_check.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Logic))
            // DEACTIVATE 0.13, TO CHECK    // .add_systems(Update, create_action_infos.run_if(resource_exists::<CombatInfos>()).run_if(on_event::<RefreshActionCostEvent>()).in_set(CombatSet::Tick).after(combat_turn_entity_check))

            // ANIME : //TODO : Changer d'endroit.
            .add_systems(Update, walk_combat_animation.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Animation))
            .add_systems(Update, path_animator_update.run_if(in_state(GameState::GameMap)).in_set(CombatSet::Animation))
            

            // TODO: Quitter le combat. PLACEHOLDER.
            .add_systems(OnExit(GameState::GameMap), combat_end)
                
            ;
    }
}



/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
pub fn combat_start(    
    mut commands: Commands,
    mut engine_state: ResMut<NextState<EngineState>>,   // TODO: Gerer le passage Combat / FreeMode.
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>)>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, _fighter_health, _fighter_stat, _fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 0});
        println!("Action points added for {:?}", fighter_id);
    }
    commands.insert_resource(CombatInfos {turn: 0, current_entity: None});
    //combat_state.set(CombatState::StartTurn);
    ev_newturn.send(CombatTurnStartEvent);
    engine_state.set(EngineState::None);
    println!("Combat start!");
}


/// Ajoute les Participants du Turn au Combat dans la queue CombatTurnQueue.
pub fn combat_turn_start(
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
pub fn combat_turn_next_entity(
    mut commands: Commands,
    mut queue: ResMut<CombatTurnQueue>,    
    action_points_q: Query<&ActionPoints>,
    //is_player_q: Query<Option<&Player>>,
    mut ev_turn_end: EventWriter<CombatTurnEndEvent>,
    mut current_combat: ResMut<CombatInfos>,
    mut ev_refresh_ap: EventWriter<RefreshActionCostEvent>,    
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
    ev_refresh_ap.send(RefreshActionCostEvent);
}

pub fn combat_turn_end(    
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    mut queue: ResMut<CombatTurnQueue>,
){
    println!("Combat turn End.");    
    queue.0.clear();
    ev_newturn.send(CombatTurnStartEvent);
}

/// Les events du Joueur.
pub fn combat_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,  
    //mut ev_try_move: EventWriter<EntityTryMoveEvent>,
    player_query: Query<(Entity, Has<Player>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    res_cursor: Res<Cursor>,    //TODO : On click event?
    mut ev_on_click: EventWriter<OnClickEvent>
){
    if keys.just_pressed(KeyCode::KeyT) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result.0;
        ev_endturn.send(EntityEndTurnEvent {entity});
        //println!("Player asked for End of round for {:?}.", entity);
    }
    if buttons.just_released(MouseButton::Left) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result.0;
        let destination = res_cursor.grid_position;

        println!("Click !");
        ev_on_click.send(OnClickEvent { entity: entity, tile: destination });

        /* 
        println!("Clic to move!");
        ev_try_move.send(EntityTryMoveEvent {entity: entity, destination: destination});
        */

    }
}

/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
pub fn combat_turn_entity_check(
    mut commands: Commands,
    current_combat: ResMut<CombatInfos>,
    query_action_points: Query<(&ActionPoints, Option<&Player>)>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
) {
    //println!("Combat turn entity check...");
    // On recupere l'entité de CombatInfos.
    if let Some(entity) = current_combat.current_entity {
        //println!("There is a current entity in CombatInfos");
        if let Ok(entity_infos) = query_action_points.get(entity) {
            //println!("This entity has action points.");
            let (ap_entity, is_player) = entity_infos;
            //If no AP anymore, next entity turn.
            if ap_entity.current <= 0 {
                //println!("This entity has no AP: let's turn to next entity event.");
                commands.entity(entity).remove::<Turn>();
                ev_next.send(CombatTurnNextEntityEvent);
           } else if is_player.is_some() {
                //println!("This entity has AP and is the Player.");
                //combat_state.set(CombatState::PlayerTurn);
           } else {
            //println!("This entity has AP but is not the player");
           }
        }
       // println!("Turn Entity check: {:?} turn.", entity);
    }    
}

/// Retire les ActionPoints, Remove CombatInfos, change State.
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

