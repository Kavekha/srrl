

// Cette fonction prends les tuiles logiques dont la visibilité doit changer, et fait les calculs pour determiner quelles tuiles render sont concernées.
// DISCARD : Provoque d'importants ralentissements!!!
pub fn update_convert_logic_tile_visibility_to_render_v2_discard(
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    view_q: Query<&View>,
) {
    let Ok(game_map_render) = game_map_render_q.get_single() else { return; };
    
    let mut component_to_delete = Vec::new();
        
    // Je recupère les entités logiques
    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        component_to_delete.push(entity);
        // Je recupere le nouveau statut.
        let mut visible_status;
        match new_visibility.new_status {
            ChangeVisibilityStatus::Visible => visible_status = 1,
            ChangeVisibilityStatus::Hidden => visible_status = -1,
        }

        let view = view_q.single();
        // Si visible, on regarde pour chaque Tuile graphique si les autres tuiles logiques qu'elles couvrent sont visibles.
        // Sinon, on a le comportement habituel.
        for render_cover in [RENDER_SW_COVER, RENDER_NW_COVER, RENDER_NE_COVER, RENDER_SE_COVER] {
            if visible_status > 0 {
                let mut score = 0;
                for logic_tile_around in render_cover {
                    if view.visible_tiles.contains(&Vector2Int {x:position.v.x + logic_tile_around.0, y:position.v.y + logic_tile_around.1 } ) {
                         score += 1;
                        break;
                    }
                }
                if score == 0 {
                    visible_status = -1;    // Elle sera "dans l'obscurité".
                }
            }
            let rendering_corner :(i32, i32);
            println!("Render cover is {:?}", render_cover);
            match render_cover {
                RENDER_SW_COVER => rendering_corner = RENDER_SW,
                RENDER_NW_COVER => rendering_corner = RENDER_NW,
                RENDER_NE_COVER => rendering_corner = RENDER_NE,
                RENDER_SE_COVER => rendering_corner = RENDER_SE,
                _ => rendering_corner = RENDER_SW, // Should not happen.
            };
            println!("Rendering corner {:?}", rendering_corner);
            
            if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x:position.v.x + rendering_corner.0, y:position.v.y + rendering_corner.1 }) {
                commands.entity(* render_tile_floor_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
            }
            if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x:position.v.x + rendering_corner.0, y:position.v.y + rendering_corner.1 }) {
                commands.entity(* render_tile_wall_entity).insert(RenderVisibilityTile { visibility_score : visible_status });
            } 
        }      
    }
    for entity in component_to_delete {
        commands.entity(entity).remove::<ChangeVisibility>();
    }
}



// 0.20h Donne les 4 render tiles FLOOR d'une tile logic.
fn get_render_tiles_floor_for_logical_tile_at(
    game_map_render:&GameMapRender,
    pos_x: i32,
    pos_y: i32
 ) -> Option<Vec<&Entity>> {
    let mut render_tiles= Vec::new();
    for (x,y) in [RENDER_NE, RENDER_SE, RENDER_SW, RENDER_NW] {
        if let Some(render_tile_floor_entity) = game_map_render.floor.get(&Vector2Int { x: pos_x + x, y: pos_y + y}) {
            render_tiles.push(render_tile_floor_entity);
        }
    }
    if render_tiles.is_empty() {
        return None
    } else {
        return Some(render_tiles)
    }
}

// 0.20h Donne les 4 render tiles WALL d'une tile logic.
fn get_render_tiles_wall_for_logical_tile_at(
    game_map_render:&GameMapRender,
    pos_x: i32,
    pos_y: i32
 ) -> Option<Vec<&Entity>> {
    let mut render_tiles= Vec::new();
    for (x,y) in [RENDER_NE, RENDER_SE, RENDER_SW, RENDER_NW] {
        if let Some(render_tile_wall_entity) = game_map_render.wall.get(&Vector2Int { x: pos_x + x, y: pos_y + y}) {
            render_tiles.push(render_tile_wall_entity);
        }
    }
    if render_tiles.is_empty() {
        return None
    } else {
        return Some(render_tiles)
    }
}

// 0.20i v0.7 : on ne Hide plus la tuile, on change sa couleur. // REMEMBER : Si d'autres jouent avec la couleur, ca va foutre la merde.
// 0.20h v0.6
// Cas #0 : Range 0 tile = On ne voit rien, pas même la place du joueur. => Non, on voit le joueur + 0.5
// Cas #1 : Range 1 tile = placé sur le joueur. => Non, on voit la tile joueur
// Cas #2 : Range 1 visibility => Est-ce que les bonnes tiles logiques sont marquées comme visibles? Oui. J'en ai 9.
// Cas #2b : Range 1 visibility => les bonnes tiles logiques sont marquées comme devant rester visibles? Oui.
// Cas #2c : Range 1 visibility => Quand je me deplace, les cases qui ne sont plus visibles sont bien signalées comme hidden? Oui.

pub fn update_tile_visibility_render_v7(
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut sprite_q: Query<&mut Sprite>,
    mut visibility_q: Query<&mut Visibility>
 ){
    let Ok(game_map_render) = game_map_render_q.get_single() else { return; };
    
    let mut component_to_delete = Vec::new();
    let mut entity_change_status = HashMap::new();
    
    // Je recupère les entités logiques
    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        component_to_delete.push(entity);
          // Je recupere le nouveau statut.
        let visible_status;
         match new_visibility.new_status {
            ChangeVisibilityStatus::Visible => visible_status = 1,
            ChangeVisibilityStatus::Hidden => visible_status = -1,
        }
        // Pour les 4 tuiles Render de cette tuile logique
        if let Some(render_tiles_floor) = get_render_tiles_floor_for_logical_tile_at(game_map_render, position.v.x, position.v.y) {
            for render_tile_floor_entity in render_tiles_floor {
                if let Some(render_tile_floor_entity_entry) = entity_change_status.get_mut(render_tile_floor_entity) {
                    *render_tile_floor_entity_entry += visible_status;
                } else {
                    entity_change_status.insert(render_tile_floor_entity, visible_status);
                }
            }
        }
        if let Some(render_tiles_wall) = get_render_tiles_wall_for_logical_tile_at(game_map_render, position.v.x, position.v.y) {
            for render_tile_wall_entity in render_tiles_wall {
                if let Some(render_tile_wall_entity_entry) = entity_change_status.get_mut(render_tile_wall_entity) {
                    *render_tile_wall_entity_entry += visible_status;
                } else {
                    entity_change_status.insert(render_tile_wall_entity, visible_status);
                }
            }
        }
    }
    for (entity, score) in entity_change_status {
        if let Ok(mut visibility) = visibility_q.get_mut(*entity) {
            * visibility = Visibility::Visible;
            // Hidden but known
            if score < 0 {
                if let Ok(mut sprite) = sprite_q.get_mut(*entity){
                    sprite.color.set_a(0.5);
                    sprite.color.set_r(0.5);
                    sprite.color.set_g(0.5);
                    sprite.color.set_b(0.5);
                }
            // visible
            } else {
                if let Ok(mut sprite) = sprite_q.get_mut(*entity){
                    sprite.color.set_a(1.0);
                    sprite.color.set_r(1.0);
                    sprite.color.set_g(1.0);
                    sprite.color.set_b(1.0);
                }
            }
        }
    }
    for entity in component_to_delete {
        commands.entity(entity).remove::<ChangeVisibility>();
    }
 }
