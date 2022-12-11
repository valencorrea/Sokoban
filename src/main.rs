mod api;
mod front;

use std::env::args;
use std::env;

use api::{sokoban::{SokobanError, Sokoban}, WHAT_TO_RUN_POS, server::Server, client};

fn main() -> Result<(), SokobanError> { // todo generalizar error

    let argv = args().collect::<Vec<String>>();

    if argv.len() > 1 && argv[WHAT_TO_RUN_POS] == "client" {
        client::run().unwrap();
    } else {
        let map: Vec<String> = env::args().collect();

        let sokoban = match Sokoban::create_from_path(&map[1]) {
            Ok(v) => v,
            Err(e) => panic!("SokobanError"),
        };
    
        let s = Server::create_from_map(sokoban);

        s.run();
    }

    Ok(())
}
