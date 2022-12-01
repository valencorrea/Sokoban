use std::env;
use crate::file_service::FileError;
use crate::sokoban_service::play;
use crate::user_interface::show_welcome;

mod user_interface;
mod sokoban_service;
mod file_service;
mod command_service;
mod utils;


fn main() -> Result<(), FileError> { // todo generalizar error
    let map: Vec<String> = env::args().collect();

    match play(&map[1]) { // todo mencionar como ventaja del lenguaje
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}
