use crate::snake::Snake;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub food: (f64, f64),
    pub game_over: bool,
    pub restart: bool,
    pub snake_vec: Vec<Snake>,
    pub game_id: String,
}
