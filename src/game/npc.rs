use bevy::{
    prelude::*
};

use pathfinding::prelude::astar;

use crate::{
    GameState, despawn_screen, TILE_SIZE,
    game::{
        Player, Stats, TileCollider, Npc, Monster,
        spawners::spawn_npc,
    },
    ascii::AsciiSheet,
    commons::tile_collision_check,
    map_builders::{
        pathfinding::{Position, world_to_grid_position},
        map::{Map},
    }
};

const FIXED_TIMESTEP: f32 = 0.1;
const BASE_RANGED_VIEW:i32 = 12;     // Distance à laquelle un NPC "voit" le joueur. //TODO : real visibility check

pub struct NpcPlugin;


impl Plugin for NpcPlugin{
    fn build(&self, app: &mut App) {
        app         
            .add_systems(Update, monster_step_check.run_if(in_state(GameState::GameMap)))
            .add_systems(FixedUpdate, behavior_decision.run_if(in_state(GameState::GameMap)))  // Run at FIXED_TIMESTEP FixedUpdate 
            .add_systems(Update, next_step_destination.run_if(in_state(GameState::GameMap)))  //TODO: Should be done after Behavior.            
            .add_systems(Update, move_to_next_step.run_if(in_state(GameState::GameMap)))  
            .add_systems(Update, display_pathfinding.run_if(in_state(GameState::GameMap)))            //DEBUG pas ouf 
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Npc>)     //TODO : Refacto pour rassembler tout ca dans game?     
            //.add_systems(FixedUpdate, hostile_ia_decision.run_if(in_state(GameState::GameMap)))               
            .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            ;         
    }
}



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
    //pub x: i32, //f32,
    //pub y: i32, //f32,
    pub destination: Position
}


fn display_pathfinding(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding)>,
    entity_display_query: Query<(Entity, &mut DisplayedPath)>,
    ascii: Res<AsciiSheet>,
    keys: Res<Input<KeyCode>>,
){
    if keys.just_pressed(KeyCode::Space){
        //if any False, ==> all true.
        //if none False, then ==> all false.
        let mut pathdisplay_toggle = true;
        for (entity, mut pathfinding) in entity_pathfinding_query.iter_mut(){
            if !pathfinding.debug{
                let paths = pathfinding.path.clone();
                println!("{:?}:DISPLAY: {:?}", entity, paths);
                for path in paths{
                    let (path_x, path_y) = (path.0, path.1);
                    let wp_x = path_x as f32* TILE_SIZE;
                    let wp_y = -(path_y as f32) * TILE_SIZE;
                    let displaying = spawn_npc(&mut commands, &ascii, wp_x, wp_y, format!("pathfinding {:?}",entity),'*' as usize);
                    commands.entity(displaying).insert(DisplayedPath);
                    commands.entity(displaying).remove::<Npc>();
                }
                pathfinding.debug = true;
                pathdisplay_toggle = true;
            } else {
                pathdisplay_toggle = false;
            }
        } 
        if !pathdisplay_toggle{
            for (display_entity, _displayedpath) in entity_display_query.iter(){
                commands.entity(display_entity).despawn_recursive();
            }
        }
    }
}


/// Quel est mon goal, puis-je l'atteindre, que dois je faire sinon?
/// Créé ou remplace le pathfinding, qui determine le trajet du NPC.
/// TODO: J'ai perdu de vue ma cible, mais je continue au moins jusqu'à l'endroit où elle se trouvait plutot que de rentrer chez moi.
fn behavior_decision(
    mut commands: Commands,
    map: Res<Map>,
    player_query: Query<(&Player, &mut Transform)>,
    entity_transform_query: Query<(Entity, &mut Transform, &Npc), Without<Player>>,
    entity_pathfinding_query: Query<(Entity, &mut Pathfinding)>,
){
    // Pour chaque NPC:
    for (entity, &npc_transform, npc) in entity_transform_query.iter() {
        // Mon point de depart.
        let (start_pos_x, start_pos_y) = world_to_grid_position(npc_transform.translation.x, npc_transform.translation.y);
        let start = Position(start_pos_x, start_pos_y);

        // Mon goal a moi (Joueur pour le moment donc pourrait être hors boucle, mais plus pertinent pour le futur)
        let (_player, player_transform) = player_query.single();    
        let (goal_pos_x, goal_pos_y) = world_to_grid_position(player_transform.translation.x, player_transform.translation.y);
        let goal = Position(goal_pos_x, goal_pos_y);

        // Suis-je à sa portée?
        let mut can_chase_target = false;
        if start.distance(&goal) < BASE_RANGED_VIEW {
            can_chase_target = true;
        } else {
            //println!("{:?}: behavior: Mon goal est trop loin: {:?} vs {:?}", entity, start, goal);
            //J'ai besoin de savoir si j'avais deja un goal avant de perdre ma cible de vue, pour aller au moins au dernier endroit avant de rentrer chez moi.
            //goal = Position(npc.home.0, npc.home.1);  //AVANT : on rentrait à la maison.
        }

        // Gerer le pathfinding existant.
        let mut dirty_pathfinding = true;
        for (entity_with_path, pathfinding) in entity_pathfinding_query.iter() {
            // Est-ce de moi dont il s'agit?
            if entity_with_path != entity{
                continue;
            }   

            // Est-ce que mon objectif a changé de position?
            if pathfinding.goal != goal{
                // Si je peux encore le voir, je dois recalculer mon pathfinding pour avoir un nouveau chemin.
                if can_chase_target {
                    //println!("{:?} : behavior: Mon goal {:?} est different de mon pathfinding: {:?}. Il me faut un nouveau Pathfinding.", entity, goal, pathfinding.path);
                    commands.entity(entity_with_path).remove::<Pathfinding>();
                    commands.entity(entity_with_path).remove::<MoveTo>();
                    break;
                } else {
                    // Je ne le vois plus: Je poursuis jusqu'au dernier endroit où je l'ai apperçu.
                    dirty_pathfinding = false;
                }
            } else {
                //Mon goal n'a pas changé, donc mon path est tjrs à jour.
                dirty_pathfinding = false;
            }
            break;
        }  
        if !dirty_pathfinding {
            // J'ai un pathfinding à jour, pas la peine de refaire des calculs.
            //println!("{:?} : behavior: Mon pathfinding est à jour.", entity);
            continue;
        }
        //println!("{:?}:behavior: J'ai besoin d'un novueau calcul + Pathfinding.", entity);

        // Donne moi mon trajet.
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

        // Puis-je me rendre au goal?
        if let Some(result) = result {
            // Oui
            let path = result.0;        // Liste des positions successives à suivre.
            let debug_path = path.clone();      //FOR DEBUG
            let pathfind = Pathfinding{
                start,
                goal,
                path,
                step:0,       // Always 0... refacto car deprecated: logique changée.
                dirty:false,
                debug:false       //debug : Display path false
            };
            
            //DEBUG: Est-ce que ce pathfinding est juste?!!!!!
 
            println!("{:?} mon chemin est : {:?}", entity, debug_path);

            //println!("{:?}:behavior: Mon chemin est {:?}. PathLen-1 is {:?}", entity, pathfind.path, pathfind.path.len() -1);
            commands.entity(entity).insert(pathfind);
        } else {
            //println!("{:?}:behavior: Je n'ai pas de chemin vers mon goal.", entity);
            // Non: Est-ce que je checkais home?
            if npc.home == goal {
                //println!("Je suis à Home, pas de goal en vue.");
                continue;
            } else {
                //goal = Position(npc.home.0, npc.home.1);    // Not used if no calcul.
                // TODO : relancer le calcul.
                continue;
            }
        }        
    }
}


/// J'ai un Pathfinding valide, je donne et gère les ordres de mouvement.
fn next_step_destination(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &mut Pathfinding, &Transform)>,
    entity_moveto_query: Query<(Entity, &MoveTo)>
){
    for (entity, mut pathfinding, transform) in entity_pathfinding_query.iter_mut() {
        // REMEMBER : Premiere destination d'un Path = l'endroit où je me trouve.
        // Ai je atteint ma destination?
        let (current_position_x, current_position_y) = world_to_grid_position(transform.translation.x, transform.translation.y);
        let current_position = Position(current_position_x, current_position_y);    
        let destination = pathfinding.path[pathfinding.step];

        if current_position != destination{
            //DEBUG
            //Ai-je un MoveTo?
            for (moveto_entity, _moveto) in entity_moveto_query.iter(){
                if entity == moveto_entity {
                    //je ne suis pas encore arrivé.
                    println!("{:?}:nextstep: Je suis à {:?} - ({},{}), je ne suis pas encore arrivé à {:?}", entity, current_position, current_position_x, current_position_y, destination);
                    ;
                } else {
                    continue;   // On cherche mon entité à moi.
                }
            }
            //fin DEBUG.
            continue;  // Je n'ai plus rien à faire pour ce NPC.
            // REMEMBER: Le premier pas du Pathfinding est ma position: je suis donc par defaut à current = destination.
        }
        //println!("{:?}:nextstep:Je suis arrivé à destination.", entity);
        // J'y suis, passons à l'etape suivante.
        pathfinding.step += 1;      // REMEMBER: step:0 ===> Le point de depart. Ca fait donc sens de poursuivre directement par Step2 meme au debut.
        // Est-ce la fin du path?
        if pathfinding.step > pathfinding.path.len() -1 {
            // J'ai atteint la fin du path.
            //println!("{:?}:nextstep: Je suis arrivé à la fin de mon path.", entity);
            commands.entity(entity).remove::<Pathfinding>();
            continue;
        } else {
            // J'ai un autre pas, donne moi l'ordre d'y aller.
            //println!("{:?}:nextstep: Il me faut un nouvel ordre.", entity);
            let new_destination = pathfinding.path[pathfinding.step];   // Nouveau step du pathfinding.
            //println!("{:?}:nextstep: Nouvelle destination => {:?}", entity, new_destination);
            commands.entity(entity).insert(MoveTo{
                destination: new_destination
            });
        }
    }
}

fn move_to_next_step(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &MoveTo, &mut Transform, &Stats)>,
    wall_query: Query<&Transform, (With<TileCollider>,Without<MoveTo>)>,
    time: Res<Time>,
    map: Res<Map>
){
    for (entity, moveto, mut transform, stats) in entity_pathfinding_query.iter_mut() {
        // Ou suis-je? 
        let (current_x, current_y) = world_to_grid_position(transform.translation.x, transform.translation.y);
        // Suis-je arrivé?
        let (goal_x, goal_y) = (moveto.destination.0, moveto.destination.1); 
        if (current_x, current_y) == (goal_x, goal_y){
            //println!("{:?}:moveto: Je suis arrivé à destination.", entity);
            commands.entity(entity).remove::<MoveTo>();
            continue;
        } else {
            println!("{:?}:moveto: Ma position actuelle est {},{}. Mon ordre de destination est {:?}", entity, current_x, current_y, moveto.destination);
            //println!("ma position est bloquée? {:?} Ma destination est bloquée? {:?}", map.is_blocked(current_x, current_y), map.is_blocked(moveto.destination.0, moveto.destination.1));
        }

        // Je dois avancer vers ma destination.
        // On doit calculer le Delta. REMEMBER : pour descendre dans la map, il faut faire du +y. Pour monter: -y.
        let mut x_delta= goal_x as f32 - current_x as f32;
        let mut y_delta = 0.0 - (goal_y as f32 - current_y as f32); // 0 - (1) ==> Je veux monter dans le monde, donc je soustrais du y dans la map.
        println!("{:?}:moveto: Mon delta est {},{}", entity, x_delta, y_delta);

        // Je calcule ma vitesse de deplacement pour cette iteration.
        x_delta *= stats.speed * TILE_SIZE * time.delta_seconds();
        y_delta *= stats.speed * TILE_SIZE * time.delta_seconds();

        //transform.translation += Vec3::new(x_delta, y_delta, 0.0);

        // Collision: Ne devrait pas se produire car Pathfinding prends en compte les zones bloquées.

        //TODO: Refacto car doublon avec ce qu'à le joueur.
        let current_world_position = transform.translation;

        let mut final_target = Vec3::new(0.0, 0.0, 0.0);
        let x_target = Vec3::new(x_delta, 0.0, 0.0);
        let y_target = Vec3::new(0.0, y_delta, 0.0);
        //println!("{:?}:moveto:target x {:?}, target y : {:?}", entity, x_target, y_target);

        let target_pos_x = transform.translation + x_target;

        if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target_pos_x, transform.translation))
        {
            final_target += x_target;
            //println!("X: ma final target est {:?}. J'ai ajouté x :{:?}", final_target, x_target);
        }

        let target_pos_y = transform.translation + y_target;
        if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target_pos_y, transform.translation))
        {
            final_target += y_target;
            //println!("Y: ma final target est {:?}. J'ai ajouté x :{:?}", final_target, y_target);
        }
        //println!("Final après collision : {:?}", final_target);
        
        transform.translation += final_target;

        let final_world_position = transform.translation;
        //println!("{:?}:moveto: World pos avant calcul : {:?}. Final target : {:?} - world pos après : {:?}", entity, current_world_position, final_target, final_world_position);
  
        let (now_x, now_y) = world_to_grid_position(transform.translation.x, transform.translation.y);
        println!("{:?}:moveto: Je suis arrivé à {},{}. Mon ordre de destination etait {:?}", entity, now_x, now_y, moveto.destination);        
        //println!("ma nouvelle position est bloquée? {:?} Ma destination est bloquée? {:?}", map.is_blocked(now_x, now_y), map.is_blocked(moveto.destination.0, moveto.destination.1));

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