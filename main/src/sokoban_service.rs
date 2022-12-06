use std::fmt::Debug;
use std::slice::SliceIndex;
use crate::command_service::{get_user_input, QUIT};
use crate::file_service::{FileError, read_file, validate_file};
use crate::map_service::{create_map, get_dimentions};
use crate::movement_service::{process_input, process_move};
use crate::ux::{print_map, show_goodbye, show_victory};
use crate::show_welcome;
use crate::utils::{BOX_STR, BOX_U8, delete_enters, ENTER_STR, ENTER_U8, TARGET_STR, TARGET_U8};

#[derive(Debug)]
pub enum SokobanError {
    CoordError(String),
    FileError(String),
}

#[derive(Debug)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
pub struct Sokoban {
    pub map: Vec<Vec<u8>>,
    pub user_coords: Coord,
    pub boxes_coords: Vec<Coord>,
    pub target_coords: Vec<Coord>,
    pub rows: usize,
    pub columns: usize,
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
    pub fn new(input: &String) -> Result<Self, SokobanError> {
        let (rows, columns) =  get_dimentions(input);
        let map = create_map(input.clone(), rows, columns); // todo mencionar desventajas

        let boxes_coords = match get_coords(input.clone(), BOX_STR, rows, columns){
            Ok(b) => b,
            Err(err) => return Err(err)
        };
        let target_coords = match get_coords(input.clone(), TARGET_STR, rows, columns){
            Ok(t) => t,
            Err(err) => return Err(err)
        };

        //sokoban = initialize_coords(sokoban, &boxes_coords, BOX_U8);
        //sokoban = initialize_coords(sokoban, &target_coords, TARGET_U8);

         Ok(Sokoban {
            map,
            user_coords: Coord { x: 0, y: 0},
            boxes_coords,
            target_coords,
            rows,
            columns
         })
    }
}

// todo refactorizar
pub fn play(input: &String) -> Result<(), SokobanError> {
    show_welcome();
    let mut map = match read_file(input) {
        Ok(result) => result,
        Err(error) => return Err(SokobanError::FileError("err".to_string())),
    };
    validate_file(&map)?;
    print_map(&map.clone()); // todo refactor

    let mut sokoban = match Sokoban::new(&map){
        Ok(s) => s,
        Err(err) => return Err(err)
    };

    loop {
        let input = match get_user_input() {
            Ok(i) => i,
            Err(err) => return Err(SokobanError::FileError("err".to_string())),
        };
        if input == QUIT {
            show_goodbye();
            break;
        }
        let movement: Move = process_input(&input);
        process_move(&mut sokoban, movement);

        //print_map(&map, &boxes_coords, &boxes_targets, &player_coords);

//        if victory(&sokoban) {
        if true {//victory(&boxes_coords, &boxes_targets) {
            show_victory();
            break;
        }
    }

    show_goodbye();
    Ok(())
}

/*fn victory(sokoban: &Sokoban) -> bool {
    for box_coord in sokoban.boxes_coords {
        let mut placed: bool = false;
        for target_coord in sokoban.target_coords {
            if box_coord.x == target_coord.x && box_coord.y == target_coord.y {
                placed = true;
                break;
            }
        }
        if !placed {
            return false;
        }
    }
    return true;
}*/


