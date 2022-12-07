use std::io;
use crate::api::file_service::FileError;
use crate::api::constants::{DOWN, LEFT, QUIT, RIGHT, UP};
use crate::api::ux::ask_for_command;

// todo add help command
pub fn is_valid_input(input: &String) -> bool {
    return input == UP || input == DOWN || input == LEFT || input == RIGHT || input == QUIT;
}

// todo si es invalido volver a pedir o mostrar ayuda
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

