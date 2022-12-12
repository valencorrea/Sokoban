use crate::api::constants::{ENTER_STR, ENTER_STR2};

use super::{
    constants::{AIR_U8, ENTER_U8, TARGET_U8},
    coord_service::Coord,
};

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

pub fn delete_enters(input: &mut String) -> String {
    let mut output: String = String::new();
    for i in input.chars() {
        if i.to_string() != ENTER_STR && i.to_string() != ENTER_STR2 {
            output.push_str(&*i.to_string());
        }
    }
    output
}

// TODO OK
pub fn show_welcome() {
    println!("\nBienvenidos al Sokoban!\n");
    println!("El objetivo del juego es empujar cada caja a un objetivo. ¡Suerte!\n");
}

// TODO OK
pub fn show_goodbye() {
    println!("Gracias por jugar! Nos vemos!");
}

// TODO OK
// todo agregar h de ayuda y que muestre de nuevo los comandos
pub fn show_commands() {
    println!("Comandos validos:");
    println!("\tMOVE A - LEFT");
    println!("\tMOVE W - UP");
    println!("\tMOVE D - RIGHT");
    println!("\tMOVE S - DOWN");
    println!("\tQUIT - QUIT");
    println!("\n");
}

// TODO OK
pub fn show_victory() {
    println!("\nFelicitaciones!\nHas vencido el juego. Gracias por jugar.\n");
}

// TODO OK
pub fn invalid_command() {
    println!("Comando invalido.");
    ask_for_command();
}

// TODO OK
pub fn ask_for_command() {
    println!("Escribe tu movimiento o QUIT para cerrar el juego:")
}

fn rows(bytes: &[u8]) -> usize {
    let mut rows = 0;

    for row in bytes {
        if *row == ENTER_U8 {
            rows += 1;
        }
    }
    rows
}

fn columns(total_bytes: usize, rows: &usize) -> usize {
    (total_bytes / rows) - 1
}

pub fn get_dimentions(map: &String) -> (usize, usize) {
    let rows = rows(map.as_bytes());
    let columns = columns(map.len(), &rows);
    (rows, columns)
}

pub fn create_map(mut input: String, rows: usize, columns: usize) -> Vec<Vec<u8>> {
    let mut map = vec![vec![0; columns]; rows];
    let mut row = 0;
    let mut column = 0;

    while row < rows && !input.is_empty() {
        let cell = input.remove(0);
        map[row][column] = cell as u8;
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    map
}

pub fn refresh_map(
    map: &mut Vec<Vec<u8>>,
    coords_from: &mut Coord,
    coords_to: &Coord,
    target_coords: &Vec<Coord>,
    object: u8,
) {
    map[coords_to.y][coords_to.x] = object;

    if target_coords.contains(coords_from) {
        map[coords_from.y][coords_from.x] = TARGET_U8;
    } else {
        map[coords_from.y][coords_from.x] = AIR_U8;
    }
}

#[cfg(test)]
mod dimentions_tests {
    use crate::api::constants::ENTER_U8;

    use super::*;

    #[test]
    fn map_nxm_cant_rows_returns_n() {
        assert_eq!(rows(&[42, 42, 42, ENTER_U8, 42, 42, 42, ENTER_U8]), 2);
    }

    #[test]
    fn map_nxm_cant_columns_returns_m() {
        assert_eq!(columns(8, &(2 as usize)), 3);
    }
}
