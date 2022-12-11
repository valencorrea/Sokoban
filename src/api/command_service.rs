use std::io;
use glib::FileError;
use crate::api::constants::{AIR_U8, BOX_ON_TARGET_U8, BOX_U8, DOWN, ENTER_U8, ERR_GETTING_INPUT, LEFT, PLAYER_U8, QUIT, RIGHT, TARGET_U8, UP, WALL_U8};
use crate::api::ux::{ask_for_command};
use crate::SokobanError;
use crate::SokobanError::CommandError;

pub fn is_valid_input(input: &String) -> bool {
    return input == UP || input == DOWN || input == LEFT || input == RIGHT || input == QUIT;
}

pub fn get_user_input() -> Result<String, SokobanError> {
    let mut command: String = String::new();
    while !is_valid_input(&command) {
        command.clear();
        ask_for_command();
        command = match get_command() {
            Ok(c) => c,
            Err(_) => return Err(CommandError(ERR_GETTING_INPUT.to_string())),
        };
    }
    Ok(command)
}

fn get_command() -> Result<String, SokobanError> {
    let mut command: String = String::new();
    match io::stdin()
        .read_line(&mut command) {
        Ok(c) => c,
        Err(error) => return Err(CommandError(error.to_string())),
    };
    let trimmed_len: usize = command.trim_end().len();
    command.truncate(trimmed_len);

    Ok(command)
}

pub fn valid_map_object(command: u8) -> bool {
    return if (command != BOX_U8)
        && (command != WALL_U8)
        && (command != TARGET_U8)
        && (command != ENTER_U8)
        && (command != PLAYER_U8)
        && (command != AIR_U8)
        && (command != BOX_ON_TARGET_U8) {
        false
    } else { true }
}