use std::env;
use crate::file_service::FileError;
use crate::sokoban_service::play;
use crate::user_interface::user_welcome;

mod user_interface;
mod sokoban_service;
mod file_service;
mod command_service;


fn main() -> Result<(), FileError> {
    let map: Vec<String> = env::args().collect();

    match play(&map[1]) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}
