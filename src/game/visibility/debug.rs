

// DEBUG : On affiche les tuiles autour, sans aucune contrainte.
// 0.20c : Recupère les tuiles autour du personnage, en accord avec le range donné.
// NOTE: Ne se préocupe pas des obstacles pour le moment.
// BUG : Range de 10; Gauche et haut donnent bien -10 par rapport à ma position. Bas / Droite donnent seulement +9. TopLeft donne +10 les autres +9.
// ==> OEUF CORSE. Range a...b : a inclusif, b exclusif...
// Check with 0.20g => OK pour le Range respecté des deux cotés.
fn get_tiles_around_range(
    x: i32, 
    y: i32,
    range: i32,
    max_x: i32, // map width
    max_y: i32,  // map height,
    obstacle_position_list: &HashSet< Vector2Int>       // Non utilisé.
 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    for x in (cmp::max(x - range, 0))..(cmp::min(x + range, max_x) +1) {
        for y in (cmp::max(y - range, 0))..(cmp::min(y + range, max_y) +1) {
            tiles_around_range.push(Vector2Int {x, y} )
        }
    }
    return tiles_around_range
 }


 // DEBUG : On cache les obstacles de la vue, mais on ne prends pas en compte la ligne de vue : ce qui est derrière est affiché.
 // 0.20f : Rework de get_tiles_around_range, n'affiche pas les obstacles. Pas utile, methode de debug.
 // Check with 0.20g => OK pour le Range respecté des deux cotés.
fn get_tiles_around_range_with_obstacles(
    x: i32, 
    y: i32,
    range: i32,
    max_x: i32, // map width
    max_y: i32,  // map height
    obstacle_position_list: &HashSet< Vector2Int>

 ) -> Vec<Vector2Int> {
    let mut tiles_around_range : Vec<Vector2Int> = Vec::new();
    // Rappel : for x in a..b, b est exclusif.
    for x in (cmp::max(x - range, 0))..(cmp::min(x + range, max_x) +1) {
        for y in (cmp::max(y - range, 0))..(cmp::min(y + range, max_y) +1) {
            if !obstacle_position_list.contains(&Vector2Int { x, y}) {
                tiles_around_range.push(Vector2Int {x, y} )
            }  
        }
    }
    return tiles_around_range
 }

