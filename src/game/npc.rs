use std::path;

use bevy::{
    prelude::*
};
use::rand::prelude::*;
use pathfinding::prelude::astar;

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    TILE_SIZE, GameState, despawn_screen,
    game::{Player, Stats, TileCollider},
    game::player::{tile_collision_check},
    map_builders::{
        pathfinding::{Position, Successor, world_to_grid_position, grid_to_world_position},
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
        .insert(Stats {speed: 6.0});
}

/// Update, remove or add a new Pathfinding Component.
fn behavior_decision(
    mut commands: Commands,
    map: Res<Map>,
    player_query: Query<(&Player, &mut Transform)>,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding),With<Npc>>,
    entity_transform_query: Query<(Entity, &mut Transform), (Without<Player>, Without<Pathfinding>, With<Npc>)>,
) {
    // TODO REFACTO : Peut être regarder dés le debut si Pathfinding ou pathfinding obsolete?
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
        let mut path:Vec<Position> = Vec::new();    //Empty. Serie de positions pour se rendre au goal.
        let step = 0;   // Le nombre de pas à faire avant d'atteindre le goal.
    
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
            path = result.0;        // Liste des positions successives à suivre.
            println!("Path len is {}", path.len());
            commands.entity(entity).insert(Pathfinding{
                start,
                goal,
                path,
                step        // Always 0... refacto car deprecated: logique changée.
            });
            continue;   // AU SUIVANT !
        } else {
            //TODO : Cas de merde, car on va revenir sur lui alors qu'il ne sert à rien, et tout recalculer !
            println!("No Path Found!");
            path = Vec::new();
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
        println!("Path is {:?}, step is {}.", pathfinding.path, pathfinding.step);
        println!("Path[step] is {:?}", pathfinding.path[pathfinding.step]);
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


    //Start: Ma position de NPC
    //Goal : La position du joueur.
    
    //Est-ce que je suis à une distance suffisante de lui?
        //NON ==> J'attends.
        //OUI ==>
            // est ce que j'ai deja un Pathfinding?
                // NON =>
                    // Je calcule mon Pathfinding jusqu'à lui.      // FONCTION 1.
                    // Est ce que je peux l'atteindre?
                        //NON => /!\ Je vais refaire ce calcul à chaque fois.   /!\
                            // Je me deplace aleatoirement au cas ou? // HORS SCOPE, faisons avec. //
                        //OUI =>
                            // Pathfinding vient d'être créé, je peux commencer ma traque.
                // OUI =>
                    // Est- ce que Goal == Pathfinding.goal ? Le Goal est tjrs à la meme place?
                        // NON => Je supprime Pathfinding.
                            // Je calcule mon pathfinding jusqu'à lui.  // FONCTION 1 ou Attendre prochaine iteration : Pas de Pathfinding.
                        // OUI =>
                            // Est-ce que Step >= path.len()?
                                // OUI => Je n'ai plus de trajet, je suis sur ma cible. /!\
                                // NON =>  Je suis à jour, je poursuis normalement.

    // J'ai un pathfinding normalement valide, je peux agir.
    //  Est-ce que j'ai un Step 1+ ?
        // NON  => /!\ Je suis normalement sur lui, la partie est finie./!\
        // OUI  => 
            // Je recois un MoveTo avec ma destination path[step], forcement valide car Blocked checké dans Pathfinding.
            // Step est augmenté de 1.
            // REFLEXION : Peut être que Step = Etape à aller, plutot que de supprimer? Si Step > path.len() alors j'ai fini mon voyage.

    // J'ai un MoveTo.
        // Je me deplace dans la direction, comme demandé.

    // FONCTION 1 : Calcul du Pathfinding.
        // Je retourne les infos necessaires : 
            // ==> Success : Path & Step 
            // ==> Nok: Pas de chemin.




/// IA chasse la cible. Player as target, npc as hostile.   //TODO: More flexible maybe, for IA vs IA. Later.
/// L'IA supprime son Pathfinding si Obsolete ou bien se deplace.
fn hostile_ia_decision(
    mut commands: Commands,
    map: Res<Map>,
    player_query: Query<(&Player, &mut Transform)>,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding),With<Npc>>, // Donne moi Entité + Pathfinding des entités avec Npc.
    //pathfinding_query: Query<(&Npc, &mut Pathfinding)>,
    entity_transform_query: Query<(Entity, &mut Transform), (Without<Player>, Without<Pathfinding>, With<Npc>)>,
    //transform_query: Query<(&Npc, &mut Transform), (Without<Player>, Without<Pathfinding>)>, 
) {
    // TODO : Pathfinding, work in progress.
    let (_player, player_transform) = player_query.single();

    let (target_pos_x, target_pos_y) = world_to_grid_position(player_transform.translation.x, player_transform.translation.y);
    let target_pos = Position(target_pos_x, target_pos_y);

    let goal = target_pos;

    // Est-ce qu'il a deja un component Pathfinding?
    let mut entity_nb = 0;  //DEBUG
    for (entity, mut pathfinding) in entity_pathfinding_query.iter_mut(){
        entity_nb += 1;     //DEBUG
        // Goal pas à jour.
        if pathfinding.goal != goal {
            commands.entity(entity).remove::<Pathfinding>();
            // Calculer nouveau Pathfinding.
            println!("Entity {} doit calculer un nouveau Pathfinding car pathfinding.goal != goal ", {entity_nb});
            continue;
        } else {
            if goal.distance(&pathfinding.start) > 10 {
                //Trop loin!
                println!("Entity {} est trop loin de sa cible.", {entity_nb});
                continue;
            } else {
                // Je suis à jour, je me deplacerai.      
                    // Convertir en World Units.
                    println!("Player is at world: {},{} AND grid {},{}", player_transform.translation.x, player_transform.translation.y, target_pos_x, target_pos_y);
                    println!("Goal is now : {:?}", target_pos);

                let (move_to_x_grid, move_to_y_grid) = (pathfinding.path[0].0, pathfinding.path[0].1);
                println!("Entity {} se rends à {},{} - Grid units", entity_nb, move_to_x_grid, move_to_y_grid);
                println!("Entity {} : pathfinding is {:?}", entity_nb, pathfinding.path);
                let (move_to_x, move_to_y) = grid_to_world_position(pathfinding.path[0].0, pathfinding.path[0].1);
                println!("Entity {} se rends à {},{} - World Units", entity_nb, move_to_x, move_to_y);
                //DEBUG : Back to grid unit pour confirmer
                let (move_to_x_grid_back, move_to_y_grid_back) = world_to_grid_position(move_to_x, move_to_y);
                println!("CHECK : Entity {} va se rendre à {},{} - Grid units", entity_nb, move_to_x_grid_back, move_to_y_grid_back);
                println!("CHECK: Goal is grid: {:?}, world: {:?}", (pathfinding.goal.0, pathfinding.goal.1), grid_to_world_position(pathfinding.goal.0, pathfinding.goal.1));
                pathfinding.path.remove(0);
                pathfinding.step -= 1;
                commands.entity(entity).insert(MoveTo{x:move_to_x as f32, y:move_to_y as f32});

                if pathfinding.step == 0 {
                    commands.entity(entity).remove::<Pathfinding>();
                    // Calculer nouveau Pathfinding.
                }
            }   
        }
    }

    // Pas de component Pathfinding:  
    for (entity, npc_transform) in entity_transform_query.iter() {
        let (hostile_pos_x, hostile_pos_y) = world_to_grid_position(npc_transform.translation.x, npc_transform.translation.y);
        let hostile_pos = Position(hostile_pos_x, hostile_pos_y);
        let start = hostile_pos;

        let mut path:Vec<Position> = Vec::new();    //Empty. Serie de positions pour se rendre au goal.
        let mut step = 0;   // Le nombre de pas à faire avant d'atteindre le goal.
    
        // Let's ask for a path to the player   //TODO: Maybe in another function
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
            path = result.0;
            step = path.len();
        } else {
            println!("No Path Found!");
            path = Vec::new();
            step = 0;
        }

        // Je créé un componant Pathfinding et je me l'ajoute.
        if step >= 1 {
            commands.entity(entity).insert(Pathfinding{
                start,
                goal,
                path,
                step
            });
        } else {
            println!("NPC has no way to attack the player");
        }
    }  

    // STEP:
    //https://github.com/frederickjjoubert/bevy-pathfinding/blob/6fa935f1a1d9fb848455c738b4e2bb41163450f5/src/game.rs#L159


}


/// Deprecated. Ne prends pas en compte Pathfinding.
fn npc_movement(
    mut npc_query: Query<(&Npc, &mut Transform, &Stats)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Npc>)>,
    time: Res<Time>
) {
    let mut rng = rand::thread_rng();
    for (_npc, mut npc_transform, stats) in npc_query.iter_mut(){
        // Random direction
        let mut x_delta = rng.gen_range(-1.. 2) as f32;
        let mut y_delta = rng.gen_range(-1.. 2) as f32;

        // How much will they move
        x_delta *= stats.speed * TILE_SIZE * time.delta_seconds();
        y_delta *= stats.speed * TILE_SIZE * time.delta_seconds();

        // Can they move to it?
        let target:Vec3 = npc_transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if !wall_query
        .iter()
        .any(|&npc_transform|tile_collision_check(target, npc_transform.translation))
        {
            npc_transform.translation = target;
        }

        let target:Vec3 = npc_transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !wall_query
        .iter()
        .any(|&npc_transform|tile_collision_check(target, npc_transform.translation))
        {
            npc_transform.translation = target;
        }
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