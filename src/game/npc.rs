use bevy::{
    prelude::*
};

use pathfinding::prelude::astar;

use crate::{
    GameState, despawn_screen, TILE_SIZE,
    game::{
        Player, Stats, Npc, Monster,
        spawners::spawn_npc,
    },
    ascii::AsciiSheet,
    commons::tile_collision_check,
    map_builders::{
        pathfinding::{Position, world_to_grid_position, grid_to_world_position},
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
    pub x: f32,
    pub y: f32,
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
                //println!("{:?}:DISPLAY: {:?}", entity, paths);
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
                    commands.entity(entity_with_path).remove::<Pathfinding>();
                    break;
                } else {
                    // Je ne le vois plus: Je poursuis jusqu'au dernier endroit où je l'ai apperçu.
                    dirty_pathfinding = false;
                    can_chase_target = true;
                }
            } else {
                //Mon goal n'a pas changé, donc mon path est tjrs à jour.
                dirty_pathfinding = false;
                can_chase_target = true;
            }
            break;
        }  

        // Je ne vois pas ma cible, je n'ai pas deja d'objectif.
        if !can_chase_target {
            continue;
        }
        if !dirty_pathfinding {
            // J'ai un pathfinding à jour, pas la peine de refaire des calculs.
            continue;
        }

        ////println!("{:?}:behavior: J'ai besoin d'un novueau calcul + Pathfinding.", entity);

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
            //let debug_path = path.clone();      //FOR DEBUG
            let pathfind = Pathfinding{
                start,
                goal,
                path,
                step:0,       // Always 0... refacto car deprecated: logique changée.
                dirty:false,
                debug:false       //debug : Display path false
            };

            commands.entity(entity).insert(pathfind);
        } else {
            ////println!("{:?}:behavior: Je n'ai pas de chemin vers mon goal.", entity);
            if npc.home == goal {
                ////println!("Je suis à Home, pas de goal en vue.");
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
){
    for (entity, mut pathfinding, transform) in entity_pathfinding_query.iter_mut() {
        // REMEMBER : Premiere destination d'un Path = l'endroit où je me trouve.
        let destination = pathfinding.path[pathfinding.step];
        let (goal_x, goal_y) = grid_to_world_position(destination.0, destination.1);

        // Ai je atteint ma destination?
        if !(transform.translation.x > goal_x - (TILE_SIZE / 2.5)) 
        && !(transform.translation.x < goal_x + (TILE_SIZE / 2.5))
        && !(transform.translation.y > goal_y + (TILE_SIZE / 2.5))   // REMEMBER: Quand on descends dans le monde, on fait du negatif.
        && !(transform.translation.y < goal_y - (TILE_SIZE / 2.5)) {
            // Pas encore arrivé dans la marge acceptable.
            continue;
        }

        // J'y suis, passons à l'etape suivante.
        pathfinding.step += 1;      // REMEMBER: step:0 ===> Le point de depart. Ca fait donc sens de poursuivre directement par Step2 meme au debut.
        // Est-ce la fin du path?
        if pathfinding.step > pathfinding.path.len() -1 {
            // J'ai atteint la fin du path.
            //println!("{:?}:nextstep: Je suis arrivé à la fin de mon path.", entity);
            commands.entity(entity).remove::<Pathfinding>();
            continue;
        }

        // J'ai un autre pas, donne moi l'ordre d'y aller.
        let new_destination = pathfinding.path[pathfinding.step];   // Nouveau step du pathfinding.

        //Convert to grid to world.
        let (new_destination_x, new_destination_y) = grid_to_world_position(new_destination.0, new_destination.1);
        commands.entity(entity).remove::<MoveTo>(); // Remove to be sure.
        commands.entity(entity).insert(MoveTo{
            x: new_destination_x,
            y: new_destination_y,
            destination: new_destination
        });       

    }
}

fn move_to_next_step(
    mut commands: Commands,
    mut entity_pathfinding_query: Query<(Entity, &MoveTo, &mut Transform, &Stats)>,
    //wall_query: Query<&Transform, (With<TileCollider>,Without<MoveTo>)>,
    time: Res<Time>,
    //map: Res<Map>
){
    for (entity, moveto, mut transform, stats) in entity_pathfinding_query.iter_mut() {
        // Ou suis-je? 
        let (current_x, current_y) = (transform.translation.x, transform.translation.y);
        // Suis-je arrivé?
        let (goal_x, goal_y) = (moveto.x, moveto.y); 
        
        if (current_x, current_y) == (goal_x, goal_y) {
            commands.entity(entity).remove::<MoveTo>();
            continue; 
        }
    
        // Je dois avancer vers ma destination.
        // On doit calculer le Delta. REMEMBER : pour descendre dans la map, il faut faire du +y. Pour monter: -y.
        let mut x_delta = goal_x - current_x;
        let mut y_delta = goal_y - current_y;
        if x_delta > 0.0 {
            x_delta = 1.0;
        } else if x_delta < 0.0 {
            x_delta = -1.0
        } else {
            x_delta = 0.0;
        }
        if y_delta > 0.0 {
            y_delta = 1.0;
        } else if y_delta < 0.0 {
            y_delta = -1.0
        } else {
            y_delta = 0.0;
        }

        // Je calcule ma vitesse de deplacement pour cette iteration.
        x_delta *= stats.speed * TILE_SIZE * time.delta_seconds();
        y_delta *= stats.speed * TILE_SIZE * time.delta_seconds();

        // Collision: Ne devrait pas se produire car Pathfinding prends en compte les zones bloquées.
        let x_target = Vec3::new(x_delta, 0.0, 0.0);
        let y_target = Vec3::new(0.0, y_delta, 0.0);
        //println!("{:?}:moveto:target x {:?}, target y : {:?}", entity, x_target, y_target);

        // REMEMBER: Difficile de prendre en compte les collisions à cause de la conversion world_to_grid_units.
        // Dans le cas où une cellule = 10 pixels:
        // Grille 1,1 donne world_units ==> 10,10 (Haut gauche du sprite) ==> pas de probleme.
        // Mais 19,19 world units vers grid donne 1,1, même si seulement le pixel en haut à gauche du sprite est dans la cellule: en pixel units, il sera quasiment en 3,3.
        // Si le Pathfinding demande d'aller à 1,0 ensuite, le sprite ira vers 19,9 ensuite: dans l'affichage il sera principalement sur du 3,2 alors que la conversion grid le donnerait à 2,1.
  
        transform.translation += x_target + y_target;

        //println!("Marge acceptable: x > {:?} et x < {:?} -- y > {:?} && et y < {:?}", goal_x - (TILE_SIZE / 2.5), goal_x + (TILE_SIZE / 2.5), goal_y - (TILE_SIZE / 2.5), goal_y + (TILE_SIZE / 2.5)) ;
        if transform.translation.x > goal_x - (TILE_SIZE / 2.5) 
        && transform.translation.x < goal_x + (TILE_SIZE / 2.5) 
        && transform.translation.y > goal_y + (TILE_SIZE / 2.5)     // REMEMBER: Quand on descends dans le monde, on fait du negatif.
        && transform.translation.y < goal_y - (TILE_SIZE / 2.5){
            transform.translation.x = goal_x;
            transform.translation.y = goal_y;
            commands.entity(entity).remove::<MoveTo>();
            //println!("Dans la marge acceptable : Transform is now {:?}, goal is {:?}", transform.translation, (goal_x, goal_y));
            continue;
        }

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
            //println!("Eaten !");      //TOLOG   
            game_state.set(GameState::GameOverScreen);
        }
}