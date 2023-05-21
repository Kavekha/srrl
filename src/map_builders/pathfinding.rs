use crate::TILE_SIZE;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32);
impl Position {
    pub fn distance(&self, other: &Position) -> i32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as i32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub position: Position,
    pub cost: i32,
}

//TODO : Check why absolute version. y sur grid est l'inverse de y sur world : +y on descends sur grid, -y on descends sur world.
        // REMEMBER: Difficile de prendre en compte les collisions à cause de la conversion world_to_grid_units.
        // Dans le cas où une cellule = 10 pixels:
        // Grille 1,1 donne world_units ==> 10,10 (Haut gauche du sprite) ==> pas de probleme.
        // Mais 19,19 world units vers grid donne 1,1, même si seulement le pixel en haut à gauche du sprite est dans la cellule: en pixel units, il sera quasiment en 3,3.
        // Si le Pathfinding demande d'aller à 1,0 ensuite, le sprite ira vers 19,9 ensuite: dans l'affichage il sera principalement sur du 3,2 alors que la conversion grid le donnerait à 2,1.
pub fn world_to_grid_position(
    x: f32,
    y: f32 
) -> (i32, i32) {
    //value.abs()
    let mut x_index = 0;
    let mut y_index = 0;
    //let y_index = y / TILE_SIZE as f32;
    // si modulo > 25% de Tile Size, on considère qu'il est dans la case +1, +1.
    // Exemple: TILE_SIZE = 0.05. Personnage en World Unit à 0.05, -0.05 => Il est en 0,0 si on ignore le modulo, mais en réalité c'est le haut gauche du sprite qui s'y trouve: l'essentiel de son corps est en 1,1.
    
    if x % TILE_SIZE < TILE_SIZE / 2.0 {
        x_index = (x / TILE_SIZE) as i32;
    } else {
        x_index = (x / TILE_SIZE) as i32 + 1;
    }

    if y % TILE_SIZE < TILE_SIZE / 2.0 {
        y_index = (y / TILE_SIZE) as i32 - 1;
    } else {
        y_index = (y / TILE_SIZE) as i32;       
    }

    (x_index.abs() as i32, y_index.abs() as i32)
}

pub fn grid_to_world_position(
    x: i32,
    y: i32
) -> (f32, f32) {
    //let world_x = x as f32 * TILE_SIZE;
    //let world_y = -(y as f32) * TILE_SIZE;
    //println!("g2w : Grid: {},{} ---> World {},{}",x,y,world_x,world_y);
    (x as f32 * TILE_SIZE,
    -(y as f32) * TILE_SIZE)
}
