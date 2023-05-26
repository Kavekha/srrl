

/* 
/// TODO: Broken lors de la separation Logic vs Render: on n'utilise plus de fonction generate_gamemap.
fn display_map_generation(
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands, 
    ascii:Res<AsciiSheet>,
    mut map_gen: ResMut<MapGenHistory>,
    time: Res<Time>,
    last_time: Local<f32>,
    asset_server: Res<AssetServer>,
){
    println!(
        "time since last fixed_update: {}\n",
        time.raw_elapsed_seconds() - *last_time
    );

    if !SHOW_MAPGEN_VISUALIZER{
        game_state.set(GameState::GameMap);
    }
    let map_generated = map_gen.history[map_gen.index].clone();
    println!("Current Snapshot from map history: {}", map_gen.index);
    generate_gamemap(&mut commands, &ascii, &asset_server,&map_generated);
    map_gen.index += 1;
    

    // End of map generation history:
    if map_gen.index >= map_gen.history.len(){
        println!("Fin de l'affichage de la generation history");
        game_state.set(GameState::GameMap);
    }
}
*/
