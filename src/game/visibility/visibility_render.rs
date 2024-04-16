// ==> CONCEPTION 0.20h
/*
La View du joueur est devenue sale => Il faut mettre à jour.   <== C'est le role de update_character_view_with_blocked.
J'ai la liste des tuiles logiques dans sa view.
Je determine celles que je dois mettre à jour.
J'indique quelles tuiles logiques doivent changer de statut.

Une nouvelle fonction doit prendre la main.
A partir des tuiles logiques devant être mise à jour, on determine quelles tuiles render doivent être changées et on gère les doublons avec des ordres contradictoires.
On peut partir sur le principe du Mask de la DualGrid => wall_corners

Je suis la tile logique 10,10 et je dois être visible.
Mes équivalents render sont 25% de : 
    let ne = (x, y) = (10,10)
    let se = (x, y + 1) = (10,11);
    let sw = (x - 1, y + 1) = (9,11);
    let nw = (x - 1, y) = (9,10);

Comme on doit plutot s'interresser aux angles, on va plutot regarder les tuiles logiques diagonales.
Si la tile logique NW (x -1, y -1) est hidden, alors la tuile render NW (x -1, y) de la tuile logique sera hidden.
Si la tile logique SW (x -1, y +1) est hidden, alors la tuile render SW (x -1, y +1) de la tuile logique sera hidden.
Si la tile logique NE (x +1, y -1) est hidden, alors la tuile render NE (x,y) de la tuile logique sera hidden.
Si la tile logique SE (x +1, y +1) est visible, alors seule la tuile render SE (x, y +1) de la tuile logique sera visible. 
==> En representation, cela donnera un angle "tournant".

Sauf que d'autres tuiles peuvent donner des informations contradictoires.
On peut alors donner un score de visibilité à chaque tile render. Si à 1+ alors visible, si à 0- alors Hidden. si au moins un visible => HiddenKnown.
*/

use std::cmp;

use bevy::{prelude::*, utils::{HashMap, HashSet}};

use crate::{engine::render::components::GameMapRender, game::{pieces::components::Npc, player::Player, tileboard::components::{BoardPosition, Tile}}, map_builders::map::Map, vectors::Vector2Int};

use super::components::{ChangeTileVisibility, ChangeTileVisibilityStatus, View};



 // 0.20c Get Entity from game_map_render pour floor ou wall.
 fn get_floor_entity_at(
    game_map_render:&GameMapRender,
    x: i32,
    y: i32
 ) -> Option<&Entity> {    
    if game_map_render.floor.contains_key(&Vector2Int {x, y}) {
        let option_entity_floor = game_map_render.floor.get(&Vector2Int {x, y});
        option_entity_floor
    } else {
        None
    }
 }

 fn get_wall_entity_at(
    game_map_render:&GameMapRender,
    x: i32,
    y: i32
 ) -> Option<&Entity> {    
    if game_map_render.wall.contains_key(&Vector2Int {x, y}) {
        let option_entity_wall = game_map_render.wall.get(&Vector2Int {x, y});
        option_entity_wall
    } else {
        None
    }
 }

 // Par rapport à la tile logic, quelles valeurs x y faut il ajouter pour voir la tuile au N, S, E, W, etc.
const POS_N: (i32, i32) = (0, -1);
const POS_NE: (i32, i32) = (1, -1);
const POS_E: (i32, i32) = (1, 0);
const POS_SE: (i32, i32) = (1, 1);
const POS_S: (i32, i32) = (0, 1);
const POS_SW: (i32, i32) = (-1, 1);
const POS_W: (i32, i32) = (-1, 0);
const POS_NW: (i32, i32) = (-1, -1);

// Quelles tuiles logiques determinent le Render de la tuile que l'on consulte.
// POS_RENDER_NE corresponds au cadre NE d'une tuile. il est donc partagé avec 
const POS_RENDER_NE:[(i32,i32);3] = [POS_E, POS_NE, POS_N ];
const POS_RENDER_SE:[(i32,i32);3] = [POS_E, POS_SE, POS_S ];
const POS_RENDER_SW:[(i32,i32);3] = [POS_W, POS_SW, POS_S ];
const POS_RENDER_NW:[(i32,i32);3] = [POS_W, POS_NW, POS_N ];

// RENDER_SW corresponds à 0,0.
const RENDER_SW:(i32, i32) = (0, 0);
const RENDER_NW:(i32, i32) = (0, -1);
const RENDER_NE:(i32, i32) = (1, -1);
const RENDER_SE:(i32, i32) = (1, 0);


// 0.20h-3 
// Cas #0 : Range 0 tile = On ne voit rien, pas même la place du joueur.
// Cas #1 : Range 1 tile = placé sur le joueur. => On voit qq chose.
// Cas #2 : Range 1 visibility => Est-ce que les bonnes tiles logiques sont marquées comme visibles?

pub fn update_tile_visibility_render(
    board: Res<Map>,
    mut commands: Commands,
    mut tile_with_change_order_q: Query<(Entity, &mut ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    view_q: Query<&View, With<Player>>,
    mut visibility_q: Query<&mut Visibility>,
    player_position_q: Query<&BoardPosition, With<Player>>,
 ){
    let game_map_render = game_map_render_q.single();
    let position = player_position_q.single();
    println!("Player position is at {:?}", position.v);
    // Je récupère la vue du personnage joueur 
    for view in view_q.iter() {
        // Je regarde les Logic tiles marquées.
        println!("These logic tiles changed their visibility status.");
        for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
            println!("{:?}", position.v);
        }
    }

    /*
            



        // On regarde toutes les tuiles autour.
        for (x,y) in [POS_N, POS_NE, POS_E, POS_SE, POS_S, POS_SW, POS_W, POS_NW] {
            for 
        
        [RENDER_NE, RENDER_SE, RENDER_SW, RENDER_NW] {



    let game_map_render = game_map_render_q.single();
    let position = player_position_q.single();
    println!("Player position is at {:?}", position.v);
    for view in view_q.iter() {
        for (x,y) in [RENDER_NE, RENDER_SE, RENDER_SW, RENDER_NW] {
            //floor
            let new_x = position.v.x + x;
            let new_y = position.v.y + y;
            println!("We are looking at {x},{y} => position {:?}", (new_x, new_y));
            if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: new_x, y: position.v.y + y}) {
                if let Ok(mut visibility) = visibility_q.get_mut(*render_tile_floor_entity){
                    * visibility = Visibility::Visible;
                    println!("floor is visible");
                } else {
                    println!("No Visibility for entity {:?}", render_tile_floor_entity);
                }
            } else {
                println!("No floor entity for game map render floor at {:?}", (new_x, new_y));
            }
            //wall 
            if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x + x, y: position.v.y + y}) {
                if let Ok(mut visibility) = visibility_q.get_mut(*render_tile_wall_entity){
                    * visibility = Visibility::Visible;
                    println!("Wall is visible");
                } else {
                    println!("No render_tile_wall_entity for entity {:?}", render_tile_wall_entity);
                }
            } else {
                println!("No wall entity for game map render floor at {:?}", (new_x, new_y));
            }
        }
    } */
 }


// 0.20h-2 Revision: mise à jour des tiles render.
// Cas #0 : Range 0 tile = On ne voit rien, pas même la place du joueur.
// Cas #1 : Range 1 tile = placé sur le joueur. => On voit qq chose.
// ==> Avec Range 0, on devrait se voir. Et Range 1, on devrait voir à 1 tile.
pub fn update_tile_visibility_render_v2(
    board: Res<Map>,
    mut commands: Commands,
    mut tile_with_change_order_q: Query<(Entity, &mut ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    view_q: Query<&View, With<Player>>,
    mut visibility_q: Query<&mut Visibility>,
 ){
    /*A partir des tuiles logiques devant être mise à jour, on determine quelles tuiles render doivent être changées et on gère les doublons avec des ordres contradictoires.
    On peut partir sur le principe du Mask de la DualGrid => wall_corners

    Je suis la tile logique 10,10 et je dois être visible.
    Mes équivalents render sont 25% de : 
        let ne = (x, y) = (10,10)
        let se = (x, y + 1) = (10,11);
        let sw = (x - 1, y + 1) = (9,11);
        let nw = (x - 1, y) = (9,10);

    Je regarde chaque autres tuiles logiques et donne 1 pt si Visible, 0 pt si Hidden.
    N, E, NE me donnent les points pour Render NE.
    S, E, SE me donnent les points pour Render SE.
    S, W, SW me donnent les points pour Render SW.
    N, W, NW me donnent les points pour Render NW.

    Si j'ai au moins 1 point sur ma tile Render alors je dois être affichée.
    ==> Ce systeme ne marche que pour les Walls.
    ==> Avec le floor, il faut une autre règle sinon on affiche une demi floor non visible. Cela ne nous interesse que si nous avons un mur au dessus. TODO
 
    */

    let game_map_render = game_map_render_q.single();
    // Je récupère la vue du personnage joueur 
    for view in view_q.iter() {
        // Je récupère chaque tuile logique concernée par une mise à jour.
        //let mut rendering_tiles_visible = Vec::new();    // Ici on mets les entity tiles qui devront être hidden.
        let mut to_remove = Vec::new();
        let mut render_tile_score: HashMap<Entity, u32> = HashMap::new();

        for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
            to_remove.push(entity);
            // Je consulte toutes les tiles logiques autour d'une tuile. On commence avec le NORD.
            for (x, y) in [POS_N, POS_NE, POS_E, POS_SE, POS_S, POS_SW, POS_W, POS_NW] {
                // La tuile logique autour (Nor par exemple) est visible.
                if view.visible_tiles.contains(&Vector2Int { x: position.v.x + x, y: position.v.y + y }) {
                    // On enregistre ici les Render tile concernées par cette position. 
                    // Par exemple si on consulte Nord, cela concerne NE & NW. Si on consulte NW, ca concerne NE, NW, W. On enregistre les positions concernées.
                    let mut render_tiles_covering_positions = Vec::new();

                    if POS_RENDER_NE.contains(&(x,y)) {
                        render_tiles_covering_positions.push(RENDER_NE);
                    } 
                    if POS_RENDER_NW.contains(&(x,y)) {
                        render_tiles_covering_positions.push(RENDER_NW);
                    }
                    if POS_RENDER_SE.contains(&(x,y)) {
                        render_tiles_covering_positions.push(RENDER_SE);
                    } 
                    if POS_RENDER_SW.contains(&(x,y)) {
                        render_tiles_covering_positions.push(RENDER_SW);
                    } 
                    // Il ne devrait pas y avoir de cas où on a pas une Render SW... en théorie?

                    // On monte le score de chacune des tiles concernées.
                    for render_position in render_tiles_covering_positions {
                        let (render_x, render_y) = render_position;
                        //floor
                        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x + render_x, y: position.v.y + render_y}) {
                            if let Some(score) = render_tile_score.get_mut(render_tile_floor_entity) {  // En réalité pas besoin, dés que c'est 1 ça suffit.
                                *score += 1;
                            } else {
                                render_tile_score.insert(*render_tile_floor_entity, 1);
                            }
                        }
                        //wall 
                        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x + render_x, y: position.v.y + render_y}) {
                            if let Some(score) = render_tile_score.get_mut(render_tile_wall_entity) {  // En réalité pas besoin, dés que c'est 1 ça suffit.
                                *score += 1;
                            } else {
                                render_tile_score.insert(*render_tile_wall_entity, 1);
                            }
                        }
                    }
                }
                /* 
                // Iteration #1
                // Si la tile autour de notre position est visible alors on donne à l'équivalent Render 1 pt.
                if view.visible_tiles.contains(&Vector2Int { x: position.v.x + x, y: position.v.y + y }) {
                    let mut render_x: i32;
                    let mut render_y: i32; 

                    if POS_RENDER_NE.contains(&(x,y)) {
                        (render_x, render_y) = RENDER_NE;
                    } else if POS_RENDER_NW.contains(&(x,y)) {
                        (render_x, render_y) = RENDER_NW;
                    } else if POS_RENDER_SE.contains(&(x,y)) {
                        (render_x, render_y) = RENDER_SE;
                    } else if POS_RENDER_SW.contains(&(x,y)) {
                        (render_x, render_y) = RENDER_SW;
                    } else {
                        continue;   // On devrait d'abord checker ca, puis la visibilité?
                    }

                    //floor
                    if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x + render_x, y: position.v.y + render_y}) {
                        if let Some(score) = render_tile_score.get_mut(render_tile_floor_entity) {  // En réalité pas besoin, dés que c'est 1 ça suffit.
                            *score += 1;
                        } else {
                            render_tile_score.insert(*render_tile_floor_entity, 1);
                        }
                    }
                    //wall 
                    if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x + render_x, y: position.v.y + render_y}) {
                        if let Some(score) = render_tile_score.get_mut(render_tile_wall_entity) {  // En réalité pas besoin, dés que c'est 1 ça suffit.
                            *score += 1;
                        } else {
                            render_tile_score.insert(*render_tile_wall_entity, 1);
                        }
                    }
                }
                */
            }
        }
        for (entity, score) in render_tile_score {
            if score > 0 {
                if let Ok(mut visibility) = visibility_q.get_mut(entity){
                    * visibility = Visibility::Visible
                }
            } else {
                if let Ok(mut visibility) = visibility_q.get_mut(entity){
                    * visibility = Visibility::Hidden
                }
            }
        }
    }
 }


            /* 

            // Je consulte une tuile qui doit changer.
            // Je regarde chaque tuile logique dans l'angle (diagonale):
            // --> tuile logique au Nord-Ouest (-1, -1) => Render (-1, 0)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x -1, y: position.v.y -1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
            // --> Tuile logique au Sud Ouest (-1,+1) => (-1,+1)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x -1, y: position.v.y +1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1}) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1}) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1 }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1 }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
            // --> Tuile logique au Nord Est (+1,-1) => Render (0,0)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x +1, y: position.v.y -1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y}) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y}) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
            // --> Tuile logique au Sud Est (+1,+1) => Render (0, +1)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x +1, y: position.v.y +1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y +1}) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y +1}) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y +1 }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y +1 }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
        }

        // On donne des scores à chaque entité.
        let mut score_list: HashMap<Entity, i32> = HashMap::new();
        for entity in rendering_tiles_visible_floor {
            if let Some(score) = score_list.get_mut(entity) {
                *score += 4;
            } else {
                score_list.insert(*entity, 2);
            }
        }
        for entity in rendering_tiles_visible_wall {
            if let Some(score) = score_list.get_mut(entity) {
                *score += 100;
            } else {
                score_list.insert(*entity, 100);
            }
        }            
        for entity in rendering_tiles_hidden_floor {
            if let Some(score) = score_list.get_mut(entity) {
                *score -= 1;
            } else {
                score_list.insert(*entity, -1);
            }
        }
        for entity in rendering_tiles_hidden_wall {
            if let Some(score) = score_list.get_mut(entity) {
                *score -= 1;
            } else {
                score_list.insert(*entity, -1);
            }
        }

        // On lit les scores et attribue la visibility ou non.
        for (entity, score) in score_list {
            if score >= 0 {
                if let Ok(mut visibility) = visibility_q.get_mut(entity){
                    * visibility = Visibility::Visible
                }
            } else {
                if let Ok(mut visibility) = visibility_q.get_mut(entity){
                    * visibility = Visibility::Hidden
                }
            }
        }

        /* 
        // On prends les entity de la liste et on les passe en visible.
        for entity in rendering_tiles_visible_floor {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Visible
            }
        }
        for entity in rendering_tiles_visible_wall {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Visible
            }
        }
        // Idem pour celles à Hidden NOTE : Hidden se fera en dernier ici.
        for entity in rendering_tiles_hidden_floor {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Hidden
            }
        }
        for entity in rendering_tiles_hidden_wall {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Hidden
            }
        }
        // On remove l'ordre de changer la tile.
        for entity in to_remove {
            commands.entity(entity).remove::<ChangeTileVisibility>();
        }
        */
    }
}
*/

// 0.20h-1 Revision: mise à jour des tiles render.
pub fn update_tile_visibility_render_discard(
    board: Res<Map>,
    mut commands: Commands,
    mut tile_with_change_order_q: Query<(Entity, &mut ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    view_q: Query<&View, With<Player>>,
    mut visibility_q: Query<&mut Visibility>,
 ){
    /*A partir des tuiles logiques devant être mise à jour, on determine quelles tuiles render doivent être changées et on gère les doublons avec des ordres contradictoires.
    On peut partir sur le principe du Mask de la DualGrid => wall_corners

    Je suis la tile logique 10,10 et je dois être visible.
    Mes équivalents render sont 25% de : 
        let ne = (x, y) = (10,10)
        let se = (x, y + 1) = (10,11);
        let sw = (x - 1, y + 1) = (9,11);
        let nw = (x - 1, y) = (9,10);

    Comme on doit plutot s'interresser aux angles, on va plutot regarder les tuiles logiques diagonales.
    Si la tile logique NW (x -1, y -1) est hidden, alors la tuile render NW (x -1, y) de la tuile logique sera hidden.
    Si la tile logique SW (x -1, y +1) est hidden, alors la tuile render SW (x -1, y +1) de la tuile logique sera hidden.
    Si la tile logique NE (x +1, y -1) est hidden, alors la tuile render NE (x,y) de la tuile logique sera hidden.
    Si la tile logique SE (x +1, y +1) est visible, alors seule la tuile render SE (x, y +1) de la tuile logique sera visible. 
    ==> En representation, cela donnera un angle "tournant".

    Sauf que d'autres tuiles peuvent donner des informations contradictoires.
    On peut alors donner un score de visibilité à chaque tile render. Si à 1+ alors visible, si à 0- alors Hidden. si au moins un visible => HiddenKnown.
    */

    let game_map_render = game_map_render_q.single();
    // Je récupère la vue du personnage joueur 
    for view in view_q.iter() {
        // Je récupère chaque tuile logique concernée par une mise à jour.
        let mut rendering_tiles_visible_floor = Vec::new();    // Ici on mets les entity des tiles qui devront être visibles.
        let mut rendering_tiles_hidden_floor = Vec::new();    // Ici on mets les entity tiles qui devront être hidden.
        let mut rendering_tiles_visible_wall = Vec::new();    // Ici on mets les entity des tiles qui devront être visibles.
        let mut rendering_tiles_hidden_wall = Vec::new();    // Ici on mets les entity tiles qui devront être hidden.

        let mut to_remove = Vec::new();

        for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
            to_remove.push(entity);
            // Je consulte une tuile qui doit changer.
            // Je regarde chaque tuile logique dans l'angle (diagonale):
            // --> tuile logique au Nord-Ouest (-1, -1) => Render (-1, 0)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x -1, y: position.v.y -1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
            // --> Tuile logique au Sud Ouest (-1,+1) => (-1,+1)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x -1, y: position.v.y +1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1}) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1}) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1 }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x -1, y: position.v.y +1 }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
            // --> Tuile logique au Nord Est (+1,-1) => Render (0,0)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x +1, y: position.v.y -1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y}) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y}) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
            // --> Tuile logique au Sud Est (+1,+1) => Render (0, +1)
            if view.visible_tiles.contains(&Vector2Int { x: position.v.x +1, y: position.v.y +1}) {  // Est ce que le personnage voit la tile?
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y +1}) {
                    rendering_tiles_visible_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y +1}) {
                    rendering_tiles_visible_wall.push(render_tile_wall_entity);
                }
            } else {
                if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: position.v.x, y: position.v.y +1 }) {
                    rendering_tiles_hidden_floor.push(render_tile_floor_entity);
                }
                if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: position.v.x, y: position.v.y +1 }) {
                    rendering_tiles_hidden_wall.push(render_tile_wall_entity);
                }                
            }
        }

        // On donne des scores à chaque entité.
        let mut score_list: HashMap<Entity, i32> = HashMap::new();
        for entity in rendering_tiles_visible_floor {
            if let Some(score) = score_list.get_mut(entity) {
                *score += 4;
            } else {
                score_list.insert(*entity, 2);
            }
        }
        for entity in rendering_tiles_visible_wall {
            if let Some(score) = score_list.get_mut(entity) {
                *score += 100;
            } else {
                score_list.insert(*entity, 100);
            }
        }            
        for entity in rendering_tiles_hidden_floor {
            if let Some(score) = score_list.get_mut(entity) {
                *score -= 1;
            } else {
                score_list.insert(*entity, -1);
            }
        }
        for entity in rendering_tiles_hidden_wall {
            if let Some(score) = score_list.get_mut(entity) {
                *score -= 1;
            } else {
                score_list.insert(*entity, -1);
            }
        }

        // On lit les scores et attribue la visibility ou non.
        for (entity, score) in score_list {
            if score >= 0 {
                if let Ok(mut visibility) = visibility_q.get_mut(entity){
                    * visibility = Visibility::Visible
                }
            } else {
                if let Ok(mut visibility) = visibility_q.get_mut(entity){
                    * visibility = Visibility::Hidden
                }
            }
        }

        /* 
        // On prends les entity de la liste et on les passe en visible.
        for entity in rendering_tiles_visible_floor {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Visible
            }
        }
        for entity in rendering_tiles_visible_wall {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Visible
            }
        }
        // Idem pour celles à Hidden NOTE : Hidden se fera en dernier ici.
        for entity in rendering_tiles_hidden_floor {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Hidden
            }
        }
        for entity in rendering_tiles_hidden_wall {
            if let Ok(mut visibility) = visibility_q.get_mut(*entity){
                * visibility = Visibility::Hidden
            }
        }
        // On remove l'ordre de changer la tile.
        for entity in to_remove {
            commands.entity(entity).remove::<ChangeTileVisibility>();
        }
        */
    }
}

/*
    let game_map_render = game_map_render_q.single();
    let mut to_remove = Vec::new();
    let mut tiles_to_change = HashMap::new();   // On place les infos dans ce Hashmap pour eviter les doublons de vector.

    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        //info!("I have to render the following tiles : {:?}", position.v);
        // Une tuile logique = 25% d'une tuile graphique x 4 dû au DualGrid. La position x,y corresponds à la partie Nord Ouest de la tuile logique.
        // Il faut donc aussi traiter x-+1,y ; x, y+1 ; x+1,y+1. On ne doit pas depasser le board non plus.
        // ===> En fait seul { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y} permets d'avoir un affichage propre et ok avec le range. 
        // Le reste semble s'overdriver et s'écraser mutuellement.
        
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: position.v.x }, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y}, new_visibility);

        to_remove.push(entity);
    }

    for (tile_position, new_visibility) in tiles_to_change {
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_floor = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_floor = Visibility::Hidden,
                    //ChangeTileVisibilityStatus::HiddenButKnown => {} // visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_wall = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_wall = Visibility::Hidden,
                    //ChangeTileVisibilityStatus::HiddenButKnown => {} // visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
    }       
    for entity in to_remove {
        commands.entity(entity).remove::<ChangeTileVisibility>();
    } 
*/



// 0.20d mise à jour des tiles render.
pub fn update_tile_visibility_render_older(
    board: Res<Map>,
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut visibility_q: Query<&mut Visibility>,
 ){
    let game_map_render = game_map_render_q.single();
    let mut to_remove = Vec::new();
    let mut tiles_to_change = HashMap::new();   // On place les infos dans ce Hashmap pour eviter les doublons de vector.

    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        //info!("I have to render the following tiles : {:?}", position.v);
        // Une tuile logique = 25% d'une tuile graphique x 4 dû au DualGrid. La position x,y corresponds à la partie Nord Ouest de la tuile logique.
        // Il faut donc aussi traiter x-+1,y ; x, y+1 ; x+1,y+1. On ne doit pas depasser le board non plus.
        // ===> En fait seul { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y} permets d'avoir un affichage propre et ok avec le range. 
        // Le reste semble s'overdriver et s'écraser mutuellement.
        
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: position.v.x }, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        //tiles_to_change.insert(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)}, new_visibility);
        tiles_to_change.insert(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y}, new_visibility);

        to_remove.push(entity);
    }

    for (tile_position, new_visibility) in tiles_to_change {
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_floor = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_floor = Visibility::Hidden,
                    //ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                match new_visibility.new_status {
                    ChangeTileVisibilityStatus::Visible => * visibility_wall = Visibility::Visible,
                    ChangeTileVisibilityStatus::Hidden => * visibility_wall = Visibility::Hidden,
                    //ChangeTileVisibilityStatus::HiddenButKnown => {} //* visibility_floor = Visibility::Hidden,
                }                
            }                    
        }
    }       
    for entity in to_remove {
        commands.entity(entity).remove::<ChangeTileVisibility>();
    }
 }
 
  // 0.20e ici on modifie l'affichage. L'intelligence "Je suis pas visible" va dans les autres systèmes.
  pub fn update_npc_visibility_status(
    player_view_q: Query<&View, With<Player>>,
    npc_position_q: Query<(Entity, &BoardPosition), With <Npc>>,
    mut npc_visibility_q: Query<&mut Visibility, With<Npc>>,
 ){
    for view in player_view_q.iter() {
        let all_npc_positions:&HashSet<(Entity, Vector2Int)> = &npc_position_q.iter().map(|(npc_entity, npc_position)| (npc_entity, npc_position.v)).collect();
        
        //info!("My view is : {:?}", view.visible_tiles);
        for (entity, position) in all_npc_positions{
            let Ok(mut npc_visibility) = npc_visibility_q.get_mut(*entity) else { continue };
            if view.visible_tiles.contains(position) {
                //info!("Entity {:?} is in my view at {:?}", entity, position);                
                *npc_visibility = Visibility::Visible;
            } else {
                //info!("Entity {:?} is not in view sight, because at {:?}", entity, position);
                *npc_visibility = Visibility::Hidden;
            }            
        }
    }
 }
 
