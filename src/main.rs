// TODO Esto debería ir en un README

//! # Sokoban
//!
//! ###### Santiago Czop - sczop@fi.uba.ar - 104057
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
//! ##### Ejecución [Exclusiva en LINUX]
//! Para comenzar a utilizar el programa se deberá hacer uso del comando *cargo run* seguido
//! de la ruta en donde se encuentra el archivo de entrada.
//!
//! Sólo se garantiza una correcta ejecución en Linux.
//!
//! En particular, los archivos de entrada estan dentro de la carpeta maps. De acuerdo al
//! nivel seleccionado, será la dificultad que posea el juego.
//!
//! Para levantar server: cargo run api/maps/level...
//! Para levantar cliente: cargo run client
//!
//! Otros comandos de interes:
//! - *cargo test*: Corre todas las pruebas del proyecto, tanto las unitarias como las de integración.
//! - *cargo fmt*: Formatea el código.
//! - *cargo clippy*: Advierte warnings y mejoras de código.
//! - *cargo doc --open*: Abre la documentación en un archivo .html.

mod api;

use std::env;
use std::env::args;

use api::{
    client,
    constants::WHAT_TO_RUN_POS,
    server::Server,
    sokoban::{Sokoban, SokobanError},
};

fn main() -> Result<(), SokobanError> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() > 1 && argv[WHAT_TO_RUN_POS] == "client" {
        client::run().unwrap();
    } else {
        let map: Vec<String> = env::args().collect();

        let sokoban = match Sokoban::create_from_path(&map[1]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        let s = Server::create_from_map(sokoban);
        s.run().unwrap();
    }
    Ok(())
}
