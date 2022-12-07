mod api;
mod front;

use std::env::args;

use api::WHAT_TO_RUN_POS;

use crate::api::server;
use crate::api::client;

fn main() -> std::io::Result<()> {
    let argv = args().collect::<Vec<String>>();

    if argv.len() > 1 && argv[WHAT_TO_RUN_POS] == "client" {
        client::run().unwrap();
    } else {
        server::run().unwrap();
    }

    Ok(())
}
