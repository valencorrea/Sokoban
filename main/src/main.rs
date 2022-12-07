/*mod ux;
mod sokoban_service;
mod file_service;
mod command_service;
mod utils;
mod movement_service;
mod map_service;*/
mod api;

use crate::api::server::Server;

/*
use crate::api::sokoban_service::{play, SokobanError};


fn main() -> Result<(), SokobanError> { // todo generalizar error
    let map: Vec<String> = env::args().collect();

    match play(&map[1]) { // todo mencionar como ventaja del lenguaje
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}*/

fn main() -> std::io::Result<()> {
    println!("HERE");
    let server: Server = Server::create();
    server.run();
    Ok(())
}
