use bevy::{
    prelude::*
};

use pathfinding::prelude::astar;

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    GameState, despawn_screen, TILE_SIZE,
    game::{Player, Stats, TileCollider},
    commons::tile_collision_check,
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
            .add_systems(Update, display_pathfinding.run_if(in_state(GameState::GameMap)))       //DEBUG
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
pub struct Monster;

#[derive(Component)]
pub struct DisplayedPath;

#[derive(Component)]
pub struct Pathfinding{
    pub start: Position,
    pub goal: Position,
    pub path: Vec<Position>,
    pub step: usize,
    pub dirty: bool,    //Si True, verifie la position vs Step Destination pour savoir si chemin atteint et next ordre de mouvement necessaire.
    pub debug: bool,
}

#[derive(Component)]
pub struct MoveTo{
    pub x: i32, //f32,
    pub y: i32, //f32,
    pub destination: Position
}


pub fn spawn_npc(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet,
    x: f32,
    y: f32,
    name: String,
    glyph: usize,
) -> Entity {
    let npc = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        glyph as usize,
        Color::rgb(0.3, 0.9, 0.4),
        Vec3::new(x, y, 900.0), //(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(npc)
        .insert(Npc)
        .insert(Name::new(name))
        .insert(Stats {speed: 2.0});

    npc
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
            let pathfind = Pathfinding{
                start,
                goal,
                path,
                step:0,       // Always 0... refacto car deprecated: logique changée.
                dirty:false,
                debug:false       //debug : Display path false
            };
            commands.entity(entity).insert(pathfind);
            continue;   // AU SUIVANT !
        } else {
            //TODO : Cas de merde, car on va revenir sur lui alors qu'il ne sert à rien, et tout recalculer !
            println!("No Path Found!");
        }
    }
}

/// display Pathfinding
/// TO REWORK

fn display_pathfinding(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding)>,
    ascii: Res<AsciiSheet>, //ascii: &AsciiSheet,
    keys: Res<Input<KeyCode>>,
    entity_displayedpath_query: Query<(Entity, &DisplayedPath)>,    
) {
    if keys.just_pressed(KeyCode::Space) {
        //toggle on / off.
        println!("Clic: Espace!");
        let mut delete_all = false;
        for (_entity, mut pathfinder) in entity_pathfinding_query.iter_mut() {
            if !pathfinder.debug {
                let mut current_step = 0;
                for path in pathfinder.path.iter() {
                    if current_step > pathfinder.step {
                        println!("Current step {}> pathfinder step", current_step);
                        let path_destination = (path.0, path.1);
                        let (npc_x, npc_y) = grid_to_world_position(path_destination.0, path_destination.1);
                        println!("npc x, y are : {},{}",npc_x, npc_y);

                        let displayed_path = spawn_npc(&mut commands, &ascii, npc_x, npc_y, format!("Path"), '*' as usize); 
                        commands.entity(displayed_path).insert(DisplayedPath);

                        current_step += 1;
                    }                  
                }     
                pathfinder.debug = true;    
            } else {
                pathfinder.debug = false;
                delete_all = true;
            }
        } 
        if delete_all {
            // On supprime les path displayed.
            for (displayed_path_entity, _displayed_path) in entity_displayedpath_query.iter(){
                commands.entity(displayed_path_entity).despawn_recursive();
            }
        }
        if delete_all{
            println!("-- display off --");
        } else {
            println!("-- display on --");
        }
    }
}


fn next_step_destination_old(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding, &Transform),With<Npc>>,
){
    for (entity, mut pathfinding, &transform) in entity_pathfinding_query.iter_mut() {
        println!("Next step for entity {:?}", entity);
        // J'ai fini mon path, plus besoin de lui.
        if pathfinding.step > pathfinding.path.len() -1 {         // If 4 entrys in 'Path', path.len() will be 4. But the first entry is 0 (path[0]), not path[1].
            println!("Pathfinding.step > pathfinding.path.len()");
            commands.entity(entity).remove::<Pathfinding>();
            continue;
        }

        // Je choisi l'etape actuel de mon chemin:
        let destination = pathfinding.path[pathfinding.step];
        commands.entity(entity).insert(MoveTo{x:destination.0, y:destination.1, destination:destination});
        //let (move_to_x, move_to_y) = grid_to_world_position(destination.0, destination.1);
        //commands.entity(entity).insert(MoveTo{x:move_to_x as f32, y:move_to_y as f32});

        // J'augmente Step pour la prochaine fois.
        pathfinding.step += 1;
    }
}

// TODO !
/// Take an Entity owner of a Pathfinding to the next step of its goal. WORK IN PROGRESS.
fn next_step_destination(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding, &Transform),With<Npc>>,
){
    for (entity, mut pathfinding, &transform) in entity_pathfinding_query.iter_mut() {
        if !pathfinding.dirty {
            println!("{:?} : Step {} - Allons à la nouvelle destination ! ", entity, pathfinding.step);
            println!("{:?} : Path {:?}", entity, pathfinding.path);

            let destination = pathfinding.path[pathfinding.step];   // Nouveau step.

            let (x, y) = (destination.0, destination.1);
            let (current_x, current_y) = world_to_grid_position(transform.translation.x, transform.translation.y);
            println!("Entity is at {:?}", world_to_grid_position(transform.translation.x, transform.translation.y));

            let mut x_delta= x - current_x;
            let mut y_delta = y - current_y;

            println!("{:?} : Deplacement : {},{}", entity, x_delta, y_delta);

            //let (move_to_x, move_to_y) = grid_to_world_position(destination.0, destination.1);
            //commands.entity(entity).insert(MoveTo{x:move_to_x as f32, y:move_to_y as f32});
            commands.entity(entity).insert(MoveTo{
                x:x_delta as i32,
                y:y_delta as i32, 
                destination: destination
            });
            //println!("{:?}: moveto inserted : {},{}", entity, move_to_x, move_to_y);

            // On passe en dirty:True pour checker si j'ai atteint ma position.
            pathfinding.dirty = true;
        } else {
            let destination = pathfinding.path[pathfinding.step];       //Grid value
            let current_position = world_to_grid_position(transform.translation.x, transform.translation.y); 
            println!("{:?} : Current position vs destination : {:?} vs {:?}", entity, current_position, destination);
            // J'ai atteint ma position?
            if (current_position.0, current_position.1) == (destination.0, destination.1)
            {
                println!("{:?} : Step completé", entity);
                // Step atteint.    //DOUBLON Debut de la boucle TODO refacto
                pathfinding.step += 1;
                // J'ai atteint la fin du Path?
                if pathfinding.step > pathfinding.path.len() -1 {   
                    println!("{:?} : Step > len : objectif atteint.", entity);
                    commands.entity(entity).remove::<Pathfinding>();
                    continue;
                } else {
                    pathfinding.dirty = false;  // Donne moi un move au prochain cycle. //TODO: Je perds un cycle.
                }
            }
            // Je reste en dirty True pour qu'on check si je me deplace.
        }
    }
}




/*
        println!("Next step for entity {:?}", entity);
        // J'ai fini mon path, plus besoin de lui.
        if pathfinding.step > pathfinding.path.len() -1 {         // If 4 entrys in 'Path', path.len() will be 4. But the first entry is 0 (path[0]), not path[1].
            println!("Pathfinding.step > pathfinding.path.len()");
            commands.entity(entity).remove::<Pathfinding>();
            continue;
        }

        // ORIGINAL: Is working.
        /*
        // Je choisi l'etape actuel de mon chemin:
        let destination = pathfinding.path[pathfinding.step];
        let (move_to_x, move_to_y) = grid_to_world_position(destination.0, destination.1);
        commands.entity(entity).insert(MoveTo{x:move_to_x as f32, y:move_to_y as f32});

        // J'augmente Step pour la prochaine fois.
        pathfinding.step += 1;
        */
        
        // ATTENTION: Ca, ca casse tout.
        // Si j'ai atteint l'etape souhaitée, je passe à la suivante.
        
        if pathfinding.step > 0 {
            let mut destination = pathfinding.path[pathfinding.step];       //Grid value
            let current_position = world_to_grid_position(transform.translation.x, transform.translation.y); 
            println!("Current position vs destination : {:?} vs {:?}", current_position, destination);
            if (current_position.0, current_position.1) == (destination.0, destination.1)
            {
                println!("Step completé");
                // Step atteint.    //DOUBLON Debut de la boucle TODO refacto
                pathfinding.step += 1;
                if pathfinding.step > pathfinding.path.len() -1 {   // Objectif atteint.
                    println!("Step > len : objectif atteint.");
                    continue;
                }
            }
        }
        println!("Allons à la nouvelle destination ! ");
        let destination = pathfinding.path[pathfinding.step];   // Nouveau step.
        println!("Nouvelle destination: {},{}", destination.0, destination.1);
        let (move_to_x, move_to_y) = grid_to_world_position(destination.0, destination.1);
        commands.entity(entity).insert(MoveTo{x:move_to_x as f32, y:move_to_y as f32});
    }
}
*/

/// Deplace le Transform vers la position transmise.
fn move_to_next_step(
    mut commands: Commands,
    mut moveto_query: Query<(Entity, &MoveTo, &mut Transform, &Stats, &Pathfinding)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<MoveTo>)>,
    time: Res<Time>
){
    //TODO: Collision for soft movements.
    for (entity, moveto, mut transform, stats, pathfind) in moveto_query.iter_mut(){

        // QUICK & DIRTY FIX : on se teleporte à la destination.... ==> SUPER RAPIDE.
        /*
        transform.translation.x = destination.x;
        transform.translation.y = destination.y;
        */

        // Est-ce que je suis arrivé?
        if (moveto.destination.0, moveto.destination.1) != world_to_grid_position(transform.translation.x, transform.translation.y) {
            // OLD 
            /*
            let (x, y) = (destination.x, destination.y);
            let (current_x, current_y) = world_to_grid_position(transform.translation.x, transform.translation.y);
            println!("Entity is at {:?}", world_to_grid_position(transform.translation.x, transform.translation.y));

            let mut x_delta= current_x as f32 - x as f32;
            let mut y_delta = current_y as f32 - y as f32;
            */

            let mut x_delta= moveto.x as f32;
            let mut y_delta = 0.0 - moveto.y as f32;      // To go down the grid, you have to +y. To go up, you have to -y. Counter-intuitive,

            println!("delta are : {},{}", x_delta, y_delta);

            x_delta *= stats.speed * TILE_SIZE * time.delta_seconds();
            y_delta *= stats.speed * TILE_SIZE * time.delta_seconds();
            println!("delta modified are now: {},{}", x_delta, y_delta);

            //Collision check before moving:
            //TODO: Refacto : Duplicate code with various colliding tests! (Check exit, check player, check eaten by ghouls)
            let target_x = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
            if !wall_query
            .iter()
            .any(|&transform|tile_collision_check(target_x, transform.translation))
            {
                transform.translation = target_x;
            }

            let target_y = transform.translation + Vec3::new(0.0, y_delta, 0.0);
            if !wall_query
            .iter()
            .any(|&transform|tile_collision_check(target_y, transform.translation))
            {
                transform.translation = target_y;
            }      

            println!("{:?} : ordre de mouvement vers world {},{}", entity, transform.translation.x, transform.translation.y);
            println!("{:?} : ordre de mouvement vers grid : {:?}", entity, world_to_grid_position(transform.translation.x, transform.translation.y));

        } else {
            commands.entity(entity).remove::<MoveTo>();
        }

        /*
        // OLD VERSION
        // We want the delta for modifications.
        let mut x_delta = destination.x - transform.translation.x;
        let mut y_delta = destination.y - transform.translation.y;

        // On prends en compte stats.speed & delta
        x_delta *= stats.speed * time.delta_seconds();
        y_delta *= stats.speed * time.delta_seconds();

        

        //Collision check before moving:
        //TODO: Refacto : Duplicate code with various colliding tests! (Check exit, check player, check eaten by ghouls)
        let target_x = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target_x, transform.translation))
        {
            transform.translation = target_x;
        }

        let target_y = transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target_y, transform.translation))
        {
            transform.translation = target_y;
        }        

        println!("{:?} : ordre de mouvement vers world {},{}", entity, transform.translation.x, transform.translation.y);
        println!("{:?} : ordre de mouvement vers grid : {:?}", entity, world_to_grid_position(transform.translation.x, transform.translation.y));
        // Atteint la demande?
        if (destination.x, destination.y) == (transform.translation.x, transform.translation.y)
        {
            commands.entity(entity).remove::<MoveTo>();
        }
        */
    }
}

fn monster_step_check(
    player_query: Query<(&Player, &mut Transform)>,
    npc_query: Query<&Transform, (With<Monster>, Without<Player>)>,
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