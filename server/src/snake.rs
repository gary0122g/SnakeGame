use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)] // 添加 Clone trait
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snake {
    pub head_pos_x: f64,
    pub head_pos_y: f64,
    pub length: i32,
    pub body: Vec<(f64, f64)>,
    pub direction: Direction,
}
