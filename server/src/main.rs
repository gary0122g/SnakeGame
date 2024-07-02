use game::{Direction, Game, Snake};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use user::User;
use uuid::Uuid;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
mod game;
mod user;

#[derive(Debug, Clone)]
struct GameRoom {
    game: Game,
    clients: HashMap<String, Sender>,
}

impl GameRoom {
    fn new() -> Self {
        let clients = HashMap::new();
        GameRoom {
            game: Game {
                game_id: String::new(),
                width: 1500.0,
                height: 900.0,
                food: (0.0, 0.0),
                game_over: false,
                restart: false,
                snake_vec: Vec::new(),
            },
            clients,
        }
    }

    fn print_clients(&self) {
        for (i, v) in &self.clients {
            println!("Client {} is connected.", i);
        }
    }

    pub fn randomize_food(&mut self) {
        let w = self.game.width / 50.0;
        let h = self.game.height / 50.0;
        let mut rng = rand::thread_rng();
        self.game.food = (
            (rng.gen_range(0..w as i32) as f64) * 50.0,
            (rng.gen_range(0..h as i32) as f64) * 50.0,
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
enum ClientMessage {
    Create { user_id: String },
    Connect,
    Join { user_id: String },
    Set { direction: String, index: usize },
    Get {},
}

struct Server {
    out: Sender,
    game_rooms: Arc<Mutex<GameRoom>>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Message::Text(msg_text) = msg {
            println!("Received text message: {}", msg_text);

            match serde_json::from_str::<ClientMessage>(&msg_text) {
                Ok(client_message) => match client_message {
                    ClientMessage::Connect => {
                        let user = User::new();
                        self.out
                            .send(Message::Text(user.user_id.to_string()))
                            .unwrap();
                    }
                    ClientMessage::Create { user_id } => {
                        let game_id = Uuid::new_v4().to_string();
                        let mut game_room = self.game_rooms.lock().unwrap();
                        let snake = Snake::new();
                        game_room.game.game_id = game_id.clone();
                        let snake_index = game_room.game.snake_vec.len().to_string();
                        game_room.game.snake_vec.push(snake);
                        game_room.clients.insert(user_id.clone(), self.out.clone());
                        println!("{:?}", self.game_rooms);
                        self.out.send(Message::Text(snake_index)).unwrap();
                    }
                    ClientMessage::Join { user_id } => {
                        let mut game_room = self.game_rooms.lock().unwrap();
                        game_room.clients.insert(user_id.clone(), self.out.clone());
                        game_room.print_clients();
                        let snake = Snake::new();
                        game_room.game.snake_vec.push(snake);
                        println!("{:?}", game_room);
                        self.out.send(Message::Text(String::from("JOIN"))).unwrap();
                    }
                    ClientMessage::Set { direction, index } => {
                        let mut game_room = self.game_rooms.lock().unwrap();
                        if let Some(snake) = game_room.game.snake_vec.get_mut(index) {
                            match direction.as_str() {
                                "Up" => snake.direction = Direction::Up,
                                "Down" => snake.direction = Direction::Down,
                                "Left" => snake.direction = Direction::Left,
                                "Right" => snake.direction = Direction::Right,
                                _ => {
                                    eprintln!("Invalid direction received: {}", direction);
                                }
                            }
                            let game_state = &game_room.game;
                            let serialized_message = serde_json::to_string(&game_state).unwrap();
                            self.out.send(Message::Text(serialized_message)).unwrap();
                        } else {
                            eprintln!("No snake found at index {}.", index);
                        }
                    }

                    ClientMessage::Get {} => {
                        let game_state = &self.game_rooms.lock().unwrap().game;
                        let serialized_message = serde_json::to_string(&game_state).unwrap();
                        self.out.send(Message::Text(serialized_message)).unwrap();
                    }
                },
                Err(e) => {
                    eprintln!("Error parsing message: {}", e);
                }
            }
        }
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("Connection closed.");
    }
}

fn main() {
    let game_rooms = Arc::new(Mutex::new(GameRoom::new()));

    let game_rooms_clone = Arc::clone(&game_rooms);
    thread::spawn(move || {
        let update_interval = Duration::from_millis(200);
        let mut last_update = Instant::now();

        loop {
            if last_update.elapsed() >= update_interval {
                let mut game_room = game_rooms_clone.lock().unwrap();
                let food_pos = game_room.game.food;
                let game_width = game_room.game.width;
                let game_height = game_room.game.height;
                let mut need_randomize_food = false;

                for snake in &mut game_room.game.snake_vec {
                    let snake_direction = snake.direction.clone();
                    if (snake.head_pos_x, snake.head_pos_y) == food_pos {
                        snake.add_body_part(&snake_direction);
                        need_randomize_food = true;
                    }
                    snake.change_position(&snake_direction);

                    if snake.head_pos_x < 0.0 {
                        snake.head_pos_x = 0.0;
                    } else if snake.head_pos_x > game_width - 50.0 {
                        snake.head_pos_x = game_width - 50.0;
                    }

                    if snake.head_pos_y < 0.0 {
                        snake.head_pos_y = 0.0;
                    } else if snake.head_pos_y > game_height - 50.0 {
                        snake.head_pos_y = game_height - 50.0;
                    }
                }

                if need_randomize_food {
                    game_room.randomize_food();
                }

                let game_state = &game_room.game;
                let serialized_message =
                    serde_json::to_string(&game_state).expect("Failed to serialize game state");
                for client in game_room.clients.values() {
                    client
                        .send(Message::Text(serialized_message.clone()))
                        .expect("Failed to send game state to client");
                }

                last_update = Instant::now();
            }

            thread::sleep(Duration::from_millis(400));
        }
    });

    listen("127.0.0.1:3012", |out| Server {
        out,
        game_rooms: Arc::clone(&game_rooms),
    })
    .unwrap();
}
