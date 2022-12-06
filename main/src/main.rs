use std::env;
use crate::file_service::FileError;
use crate::sokoban_service::{play, SokobanError};
use crate::ux::show_welcome;

mod ux;
mod sokoban_service;
mod file_service;
mod command_service;
mod utils;
mod movement_service;
mod map_service;


fn main() -> Result<(), SokobanError> { // todo generalizar error
    let map: Vec<String> = env::args().collect();

    match play(&map[1]) { // todo mencionar como ventaja del lenguaje
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}
