use client::*;
use game::Game;
use piston_window::*;
use serde_json;
mod client;
mod game;
mod snake;

fn main() {
    let ws_client = WebSocketClient::new("ws://localhost:3012/socket");
    let client_id = ws_client.connect_to_server().clone();
    let snake_index: usize = ws_client.create_game(client_id).parse().unwrap();

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [1500.0, 900.0])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        let res = ws_client.get_game_state();
        let game: Game = parse_game_state(&res);

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _device| {
                clear([1.0; 4], g);

                for snake in &game.snake_vec {
                    rectangle(
                        [0.0, 0.0, 1.0, 1.0],
                        [snake.head_pos_x, snake.head_pos_y, 50.0, 50.0],
                        c.transform,
                        g,
                    );
                    for (i, v) in snake.body.iter().enumerate() {
                        rectangle([0.0, 0.0, 1.0, 1.0], [v.0, v.1, 50.0, 50.0], c.transform, g);
                    }
                }

                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [game.food.0, game.food.1, 50.0, 50.0],
                    c.transform,
                    g,
                );
            });
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            let direction = match key {
                Key::Up => String::from("Up"),
                Key::Down => String::from("Down"),
                Key::Left => String::from("Left"),
                Key::Right => String::from("Right"),
                _ => String::from(""), // Handle other cases if needed
            };
            ws_client.set_direction(direction, snake_index);
        }
    }
}

fn parse_game_state(json_str: &str) -> Game {
    match serde_json::from_str(json_str) {
        Ok(game) => game,
        Err(err) => {
            eprintln!("{}", err);
            Game {
                game_id: String::new(),
                width: 0.0,
                height: 0.0,
                food: (0.0, 0.0),
                game_over: false,
                restart: false,
                snake_vec: Vec::new(),
            }
        }
    }
}
