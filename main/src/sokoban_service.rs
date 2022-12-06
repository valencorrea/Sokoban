use std::fmt::Debug;
use crate::command_service::{get_user_input, move_player, QUIT};
use crate::file_service::{FileError, read_file, validate_file};
use crate::user_interface::{mostrar_mapa, show_goodbye, show_victory};
use crate::show_welcome;
use crate::utils::{BOX_STR, BOX_U8, ENTER_U8, TARGET_STR, TARGET_U8};

#[derive(Debug)]
pub enum SokobanError {
    CoordError(String),
}

#[derive(Debug)]
pub struct Coord {
    x: u8,
    y: u8,
}

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
struct Sokoban {
    //map: Vec<Vec<u8>>,
    user_coords: Coord,
    boxes_coords: Vec<Coord>,
    target_coords: Vec<Coord>,
    rows: usize,
    columns: usize,
}

// todo otra ventaja: cargo clippy
//todo revisar
pub fn get_coords(mut coords: String, object: &str, rows: usize, columns: usize) -> Result<Vec<Coord>, SokobanError> {
    let mut row = 0;
    let mut column = 0;
    let mut coord_vec = vec![Coord{x:0, y:0}]; // todo al final eliminar el primero

    while row < rows && !coords.is_empty(){
        if coords.remove(0).to_string() == object.to_string() {
            let new_coord = Coord{x: row as u8, y: column as u8};
            coord_vec.push(new_coord);
        }
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    coord_vec.remove(0); // elimino el que use para inicializar
    Ok(coord_vec)
}

/*pub fn initialize_coords(sokoban: Vec<Vec<u8>>, coords: &Vec<Coord>, object: u8) -> Vec<Vec<u8>> {
    for coord in coords.len() {
        sokoban[coord.y][coord.x] = object;
    }
    sokoban
}*/

// todo mencionar como ventaja el que pueden ser estaticos o mutables
// todo otra ventaja lifetimes? usarlo en algun lado
impl Sokoban {
    pub fn new(map: String, rows: usize, columns: usize) -> Result<Self, SokobanError> {
        let mut sokoban = vec![vec![0; columns]; rows];

        let boxes_coords = match get_coords(map.clone(), BOX_STR, rows, columns){
            Ok(b) => b,
            Err(err) => return Err(err)
        };
        let target_coords = match get_coords(map.clone(), TARGET_STR, rows, columns){
            Ok(t) => t,
            Err(err) => return Err(err)
        };
        //sokoban = initialize_coords(sokoban, &boxes_coords, BOX_U8);
        //sokoban = initialize_coords(sokoban, &target_coords, TARGET_U8);

         Ok(Sokoban {
            //map: sokoban,
            user_coords: Coord { x: 0, y: 0},
            boxes_coords,
            target_coords,
            rows,
            columns
         })
    }
}

pub fn rows(bytes: &[u8]) -> usize {
    let mut rows = 0;

    for row in bytes {
        if *row == ENTER_U8 {
            rows += 1;
        }
    }
    rows
}

pub fn columns(total_bytes: usize, rows: &usize) -> usize {
    (total_bytes / rows) - 1
}

pub fn play(input: &String) -> Result<(), FileError> {
    show_welcome();

    let mut map = match read_file(input) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };
    validate_file(&map)?;
    mostrar_mapa(&map);

    let rows = rows(map.as_bytes());
    let columns = columns(input.len(), &rows);
    let mut sokoban = Sokoban::new(map, rows, columns);
    println!("{:?}", sokoban);
/*
    loop {
        let input = match get_user_input() {
            Ok(i) => i,
            Err(_) => return Err(FileError::ReadError(String::from("Error while getting user input.\n"))),
        };
        if input == QUIT {
            show_goodbye();
            break;
        }
        let movement: Move = move_player(&input);
        //process_move(&map, &mut boxes_coords, &mut player_coords, movement);
        //print_map(&map, &boxes_coords, &boxes_targets, &player_coords);

        if true {//victory(&boxes_coords, &boxes_targets) {
            show_victory();
            break;
        }
    }
*/
    show_goodbye();
    Ok(())
}




