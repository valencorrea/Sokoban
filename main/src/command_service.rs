use crate::sokoban_service::Move;
use std::io;
use crate::FileError;
use crate::user_interface::ask_for_command;

pub const UP: &str = "W";
pub const DOWN: &str = "S";
pub const LEFT: &str = "A";
pub const RIGHT: &str = "D";
pub const QUIT: &str = "Q";


pub fn is_valid_input(input: &String) -> bool {
    return input == UP || input == DOWN || input == LEFT || input == RIGHT || input == QUIT;
}

pub fn move_player(input: &String) -> Move {
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

pub fn get_user_input() -> Result<String, FileError> {
    let mut command: String = String::new();
    loop {
        command.clear();
        ask_for_command();
        command = match get_command() {
            Ok(c) => c,
            Err(_) => return Err(FileError::ReadError(String::from("Error while getting user input.\n"))),
        };

        if is_valid_input(&command) {
            return Ok(command);
        }
    }
}

fn get_command() -> Result<String, FileError> {
    let mut command: String = String::new();
    match io::stdin()
        .read_line(&mut command) {
        Ok(c) => c,
        Err(error) => return Err(FileError::ReadError(error.to_string())),
    };
    let trimmed_len: usize = command.trim_end().len();
    command.truncate(trimmed_len);

    Ok(command)
}