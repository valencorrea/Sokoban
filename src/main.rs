// TODO Esto debería ir en un README

//! # Sokoban
//!
//! ###### Santiago Czop - xxxxxxxxx@fi.uba.ar -xxxxxxxx
//! ###### Carolina Di Matteo - cdimatteo@fi.uba.ar - 103963
//! ###### Valentina Laura Correa - vcorrea@fi.uba.ar - 104415
//! ______________
//! ##### Introducción
//! La presente entrega contiene las funcionalidades pedidas para
//! el trabajo practico final de la materia Teoría del Lenguaje - curso Ferrigno.
//!
//! ##### Objetivo
//! El objetivo de este trabajo practico consta de simular la logica del juego
//! [Sokoban](https://es.wikipedia.org/wiki/Sokoban)
//! implementandolo en
//! [Rust](https://doc.rust-lang.org/rust-by-example/index.html)
//! aplicando los conceptos trabajados en la primer parte de la materia.
//!
//! Para acceder al repositorio donde fue desarrollado el mismo
//! se puede visitar el siguiente [enlace](https://github.com/valencorrea/Sokoban).
//!
//! ##### Ejecución
//! Para comenzar a utilizar el programa se deberá hacer uso del comando *cargo run* seguido
//! de la ruta en donde se encuentra el archivo de entrada.
//! En particular, los archivos de entrada estan dentro de */XXXXXXXXXXXXXXXX/*. De acuerdo al
//! nivel seleccionado, será la dificultad que posea el juego.
//!
//! *Ejemplo: cargo run xxxxxxxxxxxxxxx*
//!
//! XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! TODO ver bien cuando terminemos de implementar el server
//!
//! Otros comandos de interes:
//! - *cargo test*: Corre todas las pruebas del proyecto, tanto las unitarias como las de integración.
//! - *cargo fmt*: Formatea el código.
//! - *cargo clippy*: Advierte warnings y mejoras de código.
//! - *cargo doc --open*: Abre la documentación en un archivo .html.

mod api;
mod front;

use std::env::args;
use std::thread;
use std::{env, sync::mpsc::channel};

use api::{
    client,
    constants::WHAT_TO_RUN_POS,
    server::Server,
    sokoban::{Sokoban, SokobanError},
};
use front::window::run_app;

fn main() -> Result<(), SokobanError> {
    let argv = args().collect::<Vec<String>>();

    if argv.len() > 1 && argv[WHAT_TO_RUN_POS] == "client" {
        // GTK-CLIENT to SERVER
        let (tx1, rx1) = channel();

        // SERVER to CLIENT-GTK
        let (tx2, rx2) = channel();

        let client_thread = thread::spawn(move || {
            if let Err(e) = client::run_from_gui(rx1, tx2) {
                println!("[CLIENT] Can't run from GUI {:?}", e);
            };
        });

        run_app(tx1, rx2);

        let tid = client_thread.thread().id();

        match client_thread.join() {
            Ok(_) => println!("[CLIENT - THREAD MANAGEMENT]: Cleaning thread {:?}", tid),
            Err(e) => println!(
                "[SERVER - THREAD MANAGEMENT]: Couldn't clean thread {:?}, {:?}",
                tid, e
            ),
        };

        client::run().unwrap();
    } else {
        let map: Vec<String> = env::args().collect();

        let sokoban = match Sokoban::create_from_path(&map[1]) {
            Ok(v) => v,
            Err(_) => panic!("SokobanError"),
        };

        let s = Server::create_from_map(sokoban);

        s.run().unwrap();
    }

    Ok(())
}
