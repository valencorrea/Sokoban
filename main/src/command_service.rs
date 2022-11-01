use crate::sokoban_service::Move;
use std::io;

pub const UP: &str = "W";
pub const DOWN: &str = "S";
pub const LEFT: &str = "A";
pub const RIGHT: &str = "D";
pub const QUIT: &str = "Q";


pub fn is_valid_input(input: &String) -> bool {
    return input == UP || input == DOWN || input == LEFT || input == RIGHT || input == QUIT;
}

pub fn process_input(input: &String) -> Move {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else {
        Move::Right
    }
}

pub fn get_user_input() -> String {
    let mut input: String = String::new();
    loop {
        input.clear();
        println!("Escribe tu movimiento (WASD) o Q para cerrar el juego:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_len: usize = input.trim_end().len();
        input.truncate(trimmed_len);

        if is_valid_input(&input) {
            return input;
        }
    }
}