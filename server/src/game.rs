use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Snake {
    pub head_pos_x: f64,
    pub head_pos_y: f64,
    pub length: i32,
    pub body: Vec<(f64, f64)>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let s = Snake {
            head_pos_x: 0.0,
            head_pos_y: 0.0,
            length: 0,
            body: vec![],
            direction: Direction::Right,
        };
        s
    }
    pub fn change_position(&mut self, direction: &Direction) {
        self.body.insert(0, (self.head_pos_x, self.head_pos_y));
        if self.body.len() as i32 > self.length {
            self.body.pop();
        }

        match direction {
            Direction::Up => self.head_pos_y -= 50.0,
            Direction::Down => self.head_pos_y += 50.0,
            Direction::Left => self.head_pos_x -= 50.0,
            Direction::Right => self.head_pos_x += 50.0,
        }
    }

    pub fn add_body_part(&mut self, direction: &Direction) {
        let (last_x, last_y) = match self.body.last() {
            Some(&(x, y)) => (x, y),
            None => (self.head_pos_x, self.head_pos_y),
        };

        let new_part = match direction {
            Direction::Up => (last_x, last_y - 50.0),
            Direction::Down => (last_x, last_y + 50.0),
            Direction::Left => (last_x - 50.0, last_y),
            Direction::Right => (last_x + 50.0, last_y),
        };

        self.body.push(new_part);
        self.length += 1;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub game_id: String,
    pub width: f64,
    pub height: f64,
    pub food: (f64, f64),
    pub game_over: bool,
    pub restart: bool,
    pub snake_vec: Vec<Snake>,
}

impl Game {}
