mod api;
mod front;

use std::env::args;
use std::env;

use api::WHAT_TO_RUN_POS;

use crate::api::server;
use crate::api::client;
use crate::api::sokoban_service::{play, SokobanError};

fn main() -> Result<(), SokobanError> { // todo generalizar error

//fn main() -> std::io::Result<()> {
//    let argv = args().collect::<Vec<String>>();
    let map: Vec<String> = env::args().collect();


    match play(&map[1]) { // todo mencionar como ventaja del lenguaje
        //Ok(_) => run_app(),
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }.expect("TODO: panic message");

    /*if argv.len() > 1 && argv[WHAT_TO_RUN_POS] == "client" {
        client::run().unwrap();
    } else {
        server::run().unwrap();
    }*/

    Ok(())
}
