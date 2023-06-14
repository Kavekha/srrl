use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

mod pathfinder;
pub use pathfinder::find_path;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32
}

impl Vector2Int {
    pub const UP: Vector2Int = Vector2Int { x:0, y: -1 };
    pub const DOWN: Vector2Int = Vector2Int { x:0, y: 1 };
    pub const LEFT: Vector2Int = Vector2Int { x:-1, y: 0 };
    pub const RIGHT: Vector2Int = Vector2Int { x:1, y: 0 };
    pub const UPPER_LEFT: Vector2Int = Vector2Int { x:-1, y: -1 };
    pub const UPPER_RIGHT: Vector2Int = Vector2Int { x:1, y: -1 };
    pub const BOTTOM_LEFT: Vector2Int = Vector2Int { x:-1, y: 1 };
    pub const BOTTOM_RIGHT: Vector2Int = Vector2Int { x:1, y: 1 };

    pub fn new(x: i32, y: i32) -> Vector2Int {
        Vector2Int{x, y}
    }
    pub fn manhattan(&self, other: Vector2Int) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Vector2Int {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vector2Int::new(self.x + other.x, self.y + other.y);
    }
}

impl AddAssign for Vector2Int {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y};
    }
}

impl Sub for Vector2Int {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vector2Int::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector2Int {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{x: self.x - other.x, y: self.y - other.y};
    }
}

impl Div<i32> for Vector2Int {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        return Vector2Int::new(self.x / other, self.y / other)
    }
}

impl Mul<i32> for Vector2Int {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        return Vector2Int::new(self.x * other, self.y * other)
    }
}

impl Mul<Vector2Int> for i32 {
    type Output = Vector2Int;

    fn mul(self, other: Vector2Int) -> Vector2Int {
        return Vector2Int::new(other.x * self, other.y * self)
    }
}

pub const MULTI_DIRECTIONS: [Vector2Int; 8] = [
    Vector2Int::UP, Vector2Int::DOWN,
    Vector2Int::LEFT, Vector2Int::RIGHT,
    Vector2Int::UPPER_LEFT, Vector2Int::UPPER_RIGHT, 
    Vector2Int::BOTTOM_LEFT, Vector2Int::BOTTOM_RIGHT
];

pub fn get_direction_cost(v: Vector2Int) -> u32 {
    return v.x.abs() as u32 + v.y.abs() as u32
}
