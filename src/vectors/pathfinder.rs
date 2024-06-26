use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque}
};
 
use super::{MULTI_DIRECTIONS, Vector2Int, get_direction_cost};

pub fn find_path(
    start: Vector2Int,
    end: Vector2Int,
    tiles: &HashSet<Vector2Int>,
    blockers: &HashSet<Vector2Int>,
    end_not_check: bool,
) -> Option<VecDeque<Vector2Int>> {
    
    let mut queue = BinaryHeap::new();
    queue.push(Node { v: start, cost: 0});
    let mut visited = HashMap::new();
    visited.insert(start, 0);
    let mut came_from = HashMap::new();

    while let Some(Node { v, cost }) = queue.pop() {
        if v == end { break; }
        for dir in MULTI_DIRECTIONS {
            let n = v + dir;
            //let new_cost = cost + 1;
            let dir_cost = get_direction_cost(dir);
            let new_cost = cost.saturating_add(dir_cost);   // Diagonal = 2 donc cost > 1.
            // Si !tiles.contains(&n), ca veut dire qu'on est hors map.
            if !tiles.contains(&n) { continue }
            
            if !end_not_check && blockers.contains(&n) { continue }  // End path block check
            if blockers.contains(&n) && n != end { continue }   // No block check on end (for if we have a target here)

            match visited.get(&n) {
                Some(c) if *c <= new_cost => (),
                _ => {
                    visited.insert(n, new_cost);
                    queue.push(Node { v: n, cost: new_cost });
                    came_from.insert(n, v);
                }
            }
        }
    }
    let mut path = VecDeque::new();
    let mut cur = end;
    while let Some(v) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *v;
        if cur == start { return Some(path) }
    }
    None
}

// helper struct for the path finder
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pub v: Vector2Int,
    pub cost: u32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}