use std::env;
use crate::sokoban_service::play;
use crate::user_interface::user_welcome;

mod user_interface;
mod sokoban_service;
mod file_service;

fn main() {
    let map: Vec<String> = env::args().collect();
    play(&map[1]).expect("TODO: panic message"); // todo refactor
}
