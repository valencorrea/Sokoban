use std::fmt::Debug;
use std::slice::SliceIndex;
use crate::api::command_service::get_user_input;
use crate::api::file_service::{read_file, validate_file};
use crate::api::map_service::{create_map, get_dimentions};
use crate::api::movement_service::{process_input, process_move};
use crate::api::utils::{BOX_STR, BOX_U8, delete_enters, PLAYER_STR, QUIT, TARGET_STR, TARGET_U8};
use crate::api::ux::{print_map, show_goodbye, show_victory, show_welcome};

#[derive(Debug)]
pub enum SokobanError {
    CoordError(String),
    FileError(String),
    GTKError(String),
}

#[derive(Debug, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
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
    pub target_coords: Vec<Coord>, // usar para ver si gano
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
            let new_coord = Coord{x: row, y: column};
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

pub fn initialize_coords(sokoban: &mut Vec<Vec<u8>>, coords: &Vec<Coord>, object: u8) {
    for coord in coords.iter() {
        sokoban[coord.y][coord.x] = object;
    }
}

// todo mencionar como ventaja el que pueden ser estaticos o mutables
// todo otra ventaja lifetimes? usarlo en algun lado
impl Sokoban {
    pub fn new(input: &mut String) -> Result<Self, SokobanError> {
        let (rows, columns) =  get_dimentions(input);
        let input = delete_enters(input);
        let mut map = create_map(input.clone(), rows, columns); // todo mencionar desventajas

        let target_coords = match get_coords(input.clone(), TARGET_STR, rows, columns){
            Ok(t) => t,
            Err(err) => return Err(err)
        };

        let mut vec_user_coords = match get_coords(input.clone(), PLAYER_STR, rows, columns){
            Ok(t) => t,
            Err(err) => return Err(err)
        };

        Ok(Sokoban {
            map,
            user_coords: vec_user_coords.remove(0),// devuelve la primera posicion
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
        Err(error) => return Err(error),
    };
    validate_file(&map)?;
    //print_map(&map.clone()); // todo refactor
    let mut sokoban = match Sokoban::new(&mut map){
        Ok(s) => s,
        Err(error) => return Err(error)
    };

    loop {
        print_map(&mut sokoban);
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

        print_map(&mut sokoban);
        //print_map(&map, &boxes_coords, &boxes_targets, &player_coords);

        if victory(&sokoban) {
            show_victory();
            break;
        }
    }

    show_goodbye();
    Ok(())
}

fn victory(sokoban: &Sokoban) -> bool {
    let mut cant_targets = 0;
    let mut target_index = 0;

    while target_index < sokoban.target_coords.len() {
        let target = match sokoban.target_coords.get(target_index){
            Some(t) => t,
            None => &Coord{ x: 0, y: 0 }, // no deberia entrar nunca aca
        };

        if sokoban.map[target.y][target.x] == BOX_U8 {
            cant_targets += 1;
        }
    }
    return if cant_targets == sokoban.target_coords.len() {
        true
    } else { false }
}


