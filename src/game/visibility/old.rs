// Too complex and not needed.

/*


 // 0.20c visibility system with component.
fn update_character_visibility_old(
    mut player_view_q: Query<(&mut View, &BoardPosition), With<Player>>,
    board: Res<Map>,
    game_map_render_q: Query<&GameMapRender>,
    mut visibility_q: Query<&mut Visibility>,
 ){
    let game_map_render = game_map_render_q.single();
    for ( mut view, board_position) in player_view_q.iter_mut() {
        //info!("I'm {:?} and my view visible tiles is : {:?}", entity, view.visible_tiles);

        let mut view_to_treat = get_tiles_around_range(board_position.v.x, board_position.v.y, view.range, board.width -1, board.height -1);
        //info!(">> My Original view_to_treat is : {:?}", view_to_treat);

        let mut current_view: Vec<Vector2Int> = Vec::new();
        let mut to_hide: Vec<Vector2Int> = Vec::new();
        let mut treated: Vec<Vector2Int> = Vec::new();

        // On pop chaque element de view.visible_tiles et on regarde si présente dans view_to_treat.
        // Si c'est le cas, elle reste visible, on l'ajoute à current_view et on la retire à view_to_treat. Sinon, on la hide.
        // A la fin on prends chaque element restant dans view_to_treat et on les passe en visible, et on les ajoute à current_view.
        for eval_tile in view.visible_tiles.iter() {
            if view_to_treat.contains(&eval_tile) {
                current_view.push(*eval_tile);  // Deja visible.
            } else {
                to_hide.push(*eval_tile);   // A rendre invisible.
            }
            treated.push(*eval_tile);   // Est ce que to_hide garde son contenu après deferencement? // TOLEARN
        }
        //info!("After evaluating view.visible_tiles, I have");
        //info!("- current_view = {:?}", current_view);
        //info!("- to_hide = {:?}", to_hide);
        //info!("- treated = {:?}", treated);

        // Rendre invisible.
        for hiden_tile in to_hide.iter() {
            // On rends invisible.
            if let Some(entity_floor) = get_floor_entity_at(game_map_render, hiden_tile.x, hiden_tile.y ) {
                if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                    * visibility_floor = Visibility::Hidden;
                }                    
            }
            if let Some(entity_wall) = get_wall_entity_at(game_map_render, hiden_tile.x, hiden_tile.y ) {
                if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                    * visibility_wall = Visibility::Hidden;
                }                    
            }
        }

        // On retire de view to treat tous les elements déjà traités, qui etait dans la view.visible_tiles. Ces elements doivent être passé à visible.
        view_to_treat = view_to_treat.iter().filter_map(|val|{
            if treated.contains(val) {
                return None
            }
            Some(*val)
        }).collect();
        //info!("Here, I have removed treated from view_to_treat. I have now in view_to_treat: {:?}", view_to_treat);

        for tile in view_to_treat.iter() {
            current_view.push(*tile);
            //rendre visible.
            if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile.x, tile.y ) {
                if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                    * visibility_floor = Visibility::Visible;
                }                    
            }
            if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile.x, tile.y ) {
                if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                    * visibility_wall = Visibility::Visible;
                }                    
            }
        }
        //info!("My current view is now : {:?}", current_view);
        //info!("It should be the same that My Original view to treat");
        // On mets la nouvelle view.
        view.visible_tiles = current_view;
    }
}



fn change_render_tile_visibility_status(
    game_map_render: &GameMapRender,
    tile_position: &Vector2Int,
    mut visibility_q: Query<&mut Visibility>,
    new_visibility: Visibility,
 )
 {
    if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
        if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
            *visibility_floor = new_visibility;                           
        }    
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                * visibility_wall = new_visibility;               
            }                    
        }                 
    }
 }

 // 0.20d mise à jour des tiles render. 
 fn update_tile_visibility_render_discard(
    board: Res<Map>,
    mut commands: Commands,
    tile_with_change_order_q: Query<(Entity, &ChangeTileVisibility, &BoardPosition), With<Tile>>,
    game_map_render_q: Query<&GameMapRender>, 
    mut visibility_q: Query<&mut Visibility>,
 ){
    let game_map_render = game_map_render_q.single();
    let mut to_remove = Vec::new();

    let mut tiles_become_visible = Vec::new();
    let mut tiles_become_hidden = Vec::new();
    let mut tiles_become_hidden_but_known = Vec::new();

    /* 
    let mut tiles_become_visible= HashMap::new();   // On place les infos dans ce Hashmap pour eviter les doublons de vector.
    let mut tiles_become_hidden = HashMap::new();
    let mut tiles_become_hidden_but_known = HashMap::new();
*/

    for (entity, new_visibility, position) in tile_with_change_order_q.iter() {
        // Une tuile logique = 25% d'une tuile graphique x 4 dû au DualGrid. La position x,y corresponds à la partie Nord Ouest de la tuile logique.
        // Il faut donc aussi traiter x-+1,y ; x, y+1 ; x+1,y+1. On ne doit pas depasser le board non plus.
        match new_visibility.new_status {
            ChangeTileVisibilityStatus::Visible => {
                //tiles_become_visible.push(Vector2Int { x: position.v.x, y: position.v.x });
                //tiles_become_visible.push(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)});
                //tiles_become_visible.push(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)});
                tiles_become_visible.push(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y});
            },
            ChangeTileVisibilityStatus::Hidden => {
                //tiles_become_hidden.push(Vector2Int { x: position.v.x, y: position.v.x });
                //tiles_become_hidden.push(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)});
                //tiles_become_hidden.push(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)});
                tiles_become_hidden.push(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y});              
            },
            ChangeTileVisibilityStatus::HiddenButKnown => {
                //tiles_become_hidden_but_known.push(Vector2Int { x: position.v.x, y: position.v.x });
                //tiles_become_hidden_but_known.push(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: cmp::min(position.v.y + 1, board.height - 1)});
                //tiles_become_hidden_but_known.push(Vector2Int { x: position.v.x, y: cmp::min(position.v.y + 1, board.height - 1)});
                tiles_become_hidden_but_known.push(Vector2Int { x: cmp::min(position.v.x + 1, board.width - 1), y: position.v.y});               
            }
        }
        to_remove.push(entity);
    }

    // On élimine les duplicatas.
    tiles_become_visible.sort();
    tiles_become_visible.dedup();
    tiles_become_hidden.sort();
    tiles_become_hidden.dedup();
    tiles_become_hidden_but_known.sort();
    tiles_become_hidden_but_known.dedup();

    // On veut aussi faire d'abord le visible puis le Hidden. On préfère que le joueur voit moins graphiquement la tile logique visible plutot qu'il voit des bouts de tiles logiques considérées comme non visibles.

    for tile_position in tiles_become_visible {
        //change_render_tile_visibility_status(game_map_render, &tile_position, visibility_q, Visibility::Visible);
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                * visibility_floor = Visibility::Visible  ;                           
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                * visibility_wall = Visibility::Visible;               
            }                    
        } 
    }
    // TODO : Non géré encore. Ici on deviendra visible mais avec changement Alpha des couleurs?
    for tile_position in &tiles_become_hidden_but_known[..] {
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                * visibility_floor = Visibility::Hidden;                           
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                * visibility_wall = Visibility::Hidden;               
            }                    
        } 
    }
    for tile_position in &tiles_become_hidden[..] {
        if let Some(entity_floor) = get_floor_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_floor) = visibility_q.get_mut(*entity_floor){
                * visibility_floor = Visibility::Hidden;                           
            }                    
        }
        if let Some(entity_wall) = get_wall_entity_at(game_map_render, tile_position.x, tile_position.y ) {
            if let Ok(mut visibility_wall) = visibility_q.get_mut(*entity_wall){
                * visibility_wall = Visibility::Hidden;               
            }                    
        } 
    }       
    for entity in to_remove {
        commands.entity(entity).remove::<ChangeTileVisibility>();
    }
 }

 */