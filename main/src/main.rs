mod api;
mod front;

use crate::api::server::Server;

/*
use crate::api::sokoban_service::{play, SokobanError};
use crate::front::window::run_app;

fn main() -> Result<(), SokobanError> { // todo generalizar error
    let map: Vec<String> = env::args().collect();

    match play(&map[1]) { // todo mencionar como ventaja del lenguaje
        //Ok(_) => run_app(),
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
