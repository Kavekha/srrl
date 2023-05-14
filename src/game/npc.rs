use bevy::{
    prelude::*
};

use pathfinding::prelude::astar;

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    GameState, despawn_screen,
    game::{Player, Stats},
    game::player::{tile_collision_check},
    map_builders::{
        pathfinding::{Position, world_to_grid_position, grid_to_world_position},
        map::{Map},
    }
};

//const FIXED_TIMESTEP: f32 = 0.5;
const BASE_RANGED_VIEW:i32 = 8;     // Distance à laquelle un NPC "voit" le joueur. //TODO : real visibility check

pub struct NpcPlugin;


impl Plugin for NpcPlugin{
    fn build(&self, app: &mut App) {
        app         
            .add_systems(Update, monster_step_check.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, behavior_decision.run_if(in_state(GameState::GameMap)))  
            .add_systems(Update, next_step_destination.run_if(in_state(GameState::GameMap)))  //TODO: Should be done after Behavior.            
            .add_systems(Update, move_to_next_step.run_if(in_state(GameState::GameMap)))  
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Npc>)     //TODO : Refacto pour rassembler tout ca dans game?
            //.add_systems(Update, npc_movement.run_if(in_state(GameState::GameMap)))            
            //.add_systems(FixedUpdate, hostile_ia_decision.run_if(in_state(GameState::GameMap)))               
            //.insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            ;         
    }
}


#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct Pathfinding{
    pub start: Position,
    pub goal: Position,
    pub path: Vec<Position>,
    pub step: usize,
}

#[derive(Component)]
pub struct MoveTo{
    pub x: f32,
    pub y: f32
}


pub fn spawn_npc(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet,
    x: f32,
    y: f32
) {
    let npc = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        2,
        Color::rgb(0.3, 0.9, 0.4),
        Vec3::new(x, y, 900.0), //(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(npc)
        .insert(Npc)
        .insert(Name::new("Npc"))
        .insert(Stats {speed: 3.0});
}

/// Update, remove or add a new Pathfinding Component.
fn behavior_decision(
    mut commands: Commands,
    map: Res<Map>,
    player_query: Query<(&Player, &mut Transform)>,
    entity_pathfinding_query: Query<(Entity, &Pathfinding),With<Npc>>,
    entity_transform_query: Query<(Entity, &mut Transform), (Without<Player>, Without<Pathfinding>, With<Npc>)>,
) {
    // TODO REFACTO : Etre utilisable avec un but autre que Player? Par exemple si Clic souris pour deplacer son propre perso. LATER !
    // TODO REFACTO : Cas de merde: Je peux pas atteindre la cible, je ne bouge pas, elle ne bouge pas, mais je continue à calculer des Path. Dirty flag? /!\ TODO !
    // TODO: Si !distance & Pathfinding: Je poursuis jusqu'au goal car dernier point où target vue.
    // TODO: Si !distance & !Pathfinding: Plus de cible, je retourne au Home.
    // TODO: Si distance & Pathfinding No Way : Je ne peux plus atteindre ma cible. Je retourne à Home à la place.
    // Player is the Monster goal.
    let (_player, player_transform) = player_query.single();
    // Pathfinding operations are made with map.tiles.
    let (goal_pos_x, goal_pos_y) = world_to_grid_position(player_transform.translation.x, player_transform.translation.y);
    let goal = Position(goal_pos_x, goal_pos_y);

    for (entity, &npc_transform) in entity_transform_query.iter() {
        //as a NPC, where do I start?
        let (start_pos_x, start_pos_y) = world_to_grid_position(npc_transform.translation.x, npc_transform.translation.y);
        let start = Position(start_pos_x, start_pos_y);

        // Est ce que je vois ma cible?
        if start.distance(&goal) > BASE_RANGED_VIEW {
            continue;   // Nope, donc j'ai pas d'avis.
        }

        // Est ce que j'ai un pathfinding?
        let mut have_pathfinding = false;
        for (pathfinding_entity, pathfinding) in entity_pathfinding_query.iter() {
            if entity != pathfinding_entity {
                // Cette entity n'est pas moi.
                continue;
            } else {
                // C'est moi! Mon Goal est-il à jour?
                if goal != pathfinding.goal {
                    commands.entity(entity).remove::<Pathfinding>();
                    break;
                }
                have_pathfinding = true;    //Pathfinding à jour.
                break;  // J'ai fais mon traitement sur le NPC.
            } 
        }

        if have_pathfinding{
            continue;   // Plus rien à faire ici.
        }
        // J'ai pas de Pathfinding, il m'en faut un.
        // TODO : Fonction à part?
        // ---- PATHFINDING REQUESTED -----
    
        // Let's ask for a path to the player
        // TODO : Improvement : La Map peut être un Extrait "visible" de la Map, dans une distance de BASE_RANGED_VIEW.
        let result = astar(
            &start,
            |position| {
                map.get_successors(position)
                    .iter()
                    .map(|successor| (successor.position, successor.cost))
                    .collect::<Vec<_>>()
            },
            |position| position.distance(&goal),
            |position| *position == goal,
        );
        // Let's do thing with the result.
        if let Some(result) = result {
            println!("Path: {:?}", result.0);
            println!("Cost: {:?}", result.1);
            let path = result.0;        // Liste des positions successives à suivre.
            println!("Path len is {}", path.len());
            commands.entity(entity).insert(Pathfinding{
                start,
                goal,
                path,
                step:0       // Always 0... refacto car deprecated: logique changée.
            });
            continue;   // AU SUIVANT !
        } else {
            //TODO : Cas de merde, car on va revenir sur lui alors qu'il ne sert à rien, et tout recalculer !
            println!("No Path Found!");
        }
    }
}

/// Take an Entity owner of a Pathfinding to the next step of its goal.
fn next_step_destination(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding),With<Npc>>,
){
    for (entity, mut pathfinding) in entity_pathfinding_query.iter_mut() {
        // J'ai fini mon path, plus besoin de lui.
        if pathfinding.step > pathfinding.path.len() -1 {         // If 4 entrys in 'Path', path.len() will be 4. But the first entry is 0 (path[0]), not path[1].
            commands.entity(entity).remove::<Pathfinding>();
            continue;
        }
        // Je choisi l'etape actuel de mon chemin:
        let destination = pathfinding.path[pathfinding.step];
        let (move_to_x, move_to_y) = grid_to_world_position(destination.0, destination.1);
        commands.entity(entity).insert(MoveTo{x:move_to_x as f32, y:move_to_y as f32});

        // J'augmente Step pour la prochaine fois.
        pathfinding.step += 1;
    }
}

/// Deplace le Transform vers la position transmise.
fn move_to_next_step(
    mut commands: Commands,
    mut moveto_query: Query<(Entity, &MoveTo, &mut Transform, &Stats)>,
    time: Res<Time>
){
    for (entity, destination, mut transform, stats) in moveto_query.iter_mut(){
        // We want the delta for modifications.
        let mut x_delta = destination.x - transform.translation.x;
        let mut y_delta = destination.y - transform.translation.y;

        // On prends en compte stats.speed & delta
        x_delta *= stats.speed * time.delta_seconds();
        y_delta *= stats.speed * time.delta_seconds();

        //No check, Pathfinding already did it. //TODO : Refacto pour être plus coherent
        transform.translation.x += x_delta; 
        transform.translation.y += y_delta;

        commands.entity(entity).remove::<MoveTo>();
    }
}

fn monster_step_check(
    player_query: Query<(&Player, &mut Transform)>,
    npc_query: Query<&Transform, (With<Npc>, Without<Player>)>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // If player on collision with a ghoul...
    let (_player, player_transform) = player_query.single();
    if npc_query
        .iter()
        .any(|&transform|tile_collision_check(player_transform.translation, transform.translation))
        {
            println!("Eaten !");      //TOLOG   
            game_state.set(GameState::GameOverScreen);
        }
}