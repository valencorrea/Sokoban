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
use std::env;

use api::WHAT_TO_RUN_POS;

use crate::api::server;
use crate::api::client;
use crate::api::_sokoban_service::{play, SokobanError};
//use crate::front::window::run_app;

fn main() -> Result<(), SokobanError> { // todo generalizar error
    let argv = args().collect::<Vec<String>>();

    match play(&argv[1]) { // todo mencionar como ventaja del lenguaje
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }.expect("TODO: panic message");


        //run_app().expect("TODO: panic message");
    
//fn main() -> std::io::Result<()> {
//    let argv = args().collect::<Vec<String>>();
    /*let map: Vec<String> = env::args().collect();

    match play(&map[1]) { // todo mencionar como ventaja del lenguaje
        //Ok(_) => run_app(),
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }.expect("TODO: panic message");*/

    /*if argv.len() > 1 && argv[WHAT_TO_RUN_POS] == "client" {
        client::run().unwrap();
    } else {
        server::run().unwrap();
    }*/

    Ok(())
}
