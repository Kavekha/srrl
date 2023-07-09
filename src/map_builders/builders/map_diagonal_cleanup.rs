use crate::map_builders::{MetaMapBuilder, BuilderMap, TileType};



pub struct DiagonalCleanUp {}

impl MetaMapBuilder for DiagonalCleanUp {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data : &mut BuilderMap) {
        self.diagonal_cleanup(build_data);
    }
}

impl DiagonalCleanUp {
    #[allow(dead_code)]
    pub fn new() -> Box<DiagonalCleanUp> {
        Box::new(DiagonalCleanUp {})
    }
    
    fn diagonal_cleanup(&mut self, build_data : &mut BuilderMap) {
        let mut neighbour_tiles: u8 = 0;
        for y in 0..build_data.map.height -2 {
            for x in 0..build_data.map.width -2 {
                if build_data.map.is_blocked(x, y) { neighbour_tiles += 1; }
                if build_data.map.is_blocked(x +1, y) { neighbour_tiles += 2; }
                if build_data.map.is_blocked(x, y + 1) { neighbour_tiles += 4; }
                if build_data.map.is_blocked(x +1, y +1) { neighbour_tiles += 8; }

                // 9 => Bloqué à NW et SE, donc diagonale libre de SW à NE. 6 => bloqué NE et SW, donc diagonale libre de NW à SE. On veut eviter ça.
                let diagonal_nw_se = build_data.map.xy_idx(x, y +1);    // 6
                let diagonal_sw_ne = build_data.map.xy_idx(x +1, y +1); //9
                match neighbour_tiles {
                    6 => { build_data.map.tiles[diagonal_nw_se] = TileType::Floor; }
                    9 => { build_data.map.tiles[diagonal_sw_ne] = TileType::Floor; },
                    _ => {}
                };
                neighbour_tiles = 0;
            }
        }
    }
}