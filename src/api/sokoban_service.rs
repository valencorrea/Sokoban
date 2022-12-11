use crate::api::command_service::get_user_input;
use crate::api::file_service::{read_file, validate_file};
use crate::api::map_service::{create_map, get_dimentions};
use crate::api::movement_service::{Move, process_input, process_move};
use crate::api::constants::{BOX_STR, WALL_U8, TARGET_STR, PLAYER_STR, QUIT, BOX_U8, BOX_ON_TARGET_STR, ERR_GETTING_INPUT};
use crate::api::utils::delete_enters;
use crate::api::ux::{print_map, show_goodbye, show_victory, show_welcome};
use std::fmt::Debug;
use crate::api::coord_service::Coord;
use crate::SokobanError::{CommandError, FileError};

#[derive(Debug)]
pub enum SokobanError {
    CoordError(String),
    FileError(String),
    GTKError(String),
    CommandError(String),
}

#[derive(Debug)]
pub struct Sokoban {
    pub map: Vec<Vec<u8>>,
    pub user_coords: Coord,
    pub target_coords: Vec<Coord>,
    pub boxes_coords: Vec<Coord>,
    pub boxes_on_target_coords: Vec<Coord>,
    pub rows: usize,
    pub columns: usize,
}

pub fn get_coords(
    mut coords: String,
    object: &str,
    rows: usize,
    columns: usize,
) -> Result<Vec<Coord>, SokobanError> {
    let mut row = 0;
    let mut column = 0;
    let mut coord_vec = vec![Coord { x: 0, y: 0 }];

    // todo no va a entrar porque no se llama a la funcion con box_str
    if object == BOX_STR {
        return Ok(vec![
            Coord { x: 3, y: 2 },
            Coord { x: 4, y: 3 },
            Coord { x: 4, y: 4 },
            Coord { x: 1, y: 6 },
            Coord { x: 3, y: 6 },
            Coord { x: 4, y: 6 },
            Coord { x: 5, y: 6 },
        ]);
    } /* else if object == TARGET_STR {
          return Ok(vec![
              Coord { x: 1, y: 2 },
              Coord { x: 5, y: 3 },
              Coord { x: 1, y: 4 },
              Coord { x: 4, y: 5 },
              Coord { x: 3, y: 6 },
              Coord { x: 6, y: 6 },
              Coord { x: 4, y: 7 },
          ]);
      }*/

    coords = delete_enters(&mut coords); //Agregue esto
    // ->  cuando entra aca ya no deberia haber enters, se llama a delete enters antes de este llamado
    while row < rows && !coords.is_empty() {
        if coords.remove(0).to_string() == object.to_string() {
            let new_coord = Coord { x: column, y: row };
            coord_vec.push(new_coord);
        }
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    coord_vec.remove(0);
    Ok(coord_vec)
}

// ==== BORRAR DESDE ACA =====

fn is_wall(coords: &Coord, map: &Vec<Vec<u8>>) -> bool {
    return map[coords.y as usize][coords.x as usize] == WALL_U8;
}

fn is_player(coord: &Coord, player_coords: &Coord) -> bool {
    if coord.x == player_coords.x && coord.y == player_coords.y {
        return true;
    } else {
        return false;
    }
}

fn is_box(coord: &Coord, boxes_coords: &Vec<Coord>) -> bool {
    for box_coords in boxes_coords.iter() {
        if coord.x == box_coords.x && coord.y == box_coords.y {
            return true;
        }
    }
    return false;
}

fn is_target(coord: &Coord, boxes_targets: &Vec<Coord>) -> bool {
    for box_target in boxes_targets.iter() {
        if coord.x == box_target.x && coord.y == box_target.y {
            return true;
        }
    }
    return false;
}

// ======== HASTA ACA ====== al terminar el front

fn append_targets(targets: &mut Vec<Coord>, boxes_on_targets: &Vec<Coord>){
    for box_on_target in boxes_on_targets {
        let coord = Coord{x:box_on_target.x, y:box_on_target.y};
        targets.push(coord);
    }
}

impl Sokoban {
    pub fn new(input: &mut String) -> Result<Self, SokobanError> {
        let (rows, columns) = get_dimentions(input);
        let input = delete_enters(input);
        let mut map = create_map(input.clone(), rows, columns);

        let mut target_coords = get_coords(input.clone(), TARGET_STR, rows, columns)?;
        let boxes_on_target_coords = get_coords(input.clone(), BOX_ON_TARGET_STR, rows, columns)?;
        append_targets(&mut target_coords, &boxes_on_target_coords);

        let boxes_coords = get_coords(input.clone(), BOX_STR, rows, columns)?;
        let mut vec_user_coords = get_coords(input.clone(), PLAYER_STR, rows, columns)?;

        Ok(Sokoban {
            map,
            user_coords: vec_user_coords.remove(0),
            target_coords,
            boxes_coords,
            boxes_on_target_coords,
            rows,
            columns,
        })
    }

    pub fn print_map(&self) {
        for j in 0..self.rows {
            /*pub map: Vec<Vec<u8>>,
            pub user_coords: Coord,
            pub boxes_coords: Vec<Coord>,
            pub target_coords: Vec<Coord>,*/
            let row: &Vec<u8> = &self.map[j];
            for i in 0..self.columns {
                let cell: u8 = row[i];
                let coord: Coord = Coord { x: i, y: j };
                if cell == WALL_U8 {
                    print!("#");
                } else if is_player(&coord, &self.user_coords) {
                    print!("P");
                }
                /*else if is_box(&coord, &self.boxes_coords) {
                    if is_target(&coord, &self.target_coords) {
                        print!("*");
                    } else {
                        print!("=");
                    }
                }*/
                else if is_target(&coord, &self.target_coords) {
                    print!("+");
                } else {
                    print!(" ");
                }
                print!(" ");
            }
            println!("");
        }
    }
}

fn end_game(input: &String) -> bool {
    return if input == QUIT {
        show_goodbye();
        true
    } else { false }
}

pub fn won_game(sokoban: &mut Sokoban) -> bool {
    return if sokoban.boxes_on_target_coords.len() == sokoban.target_coords.len() {
        print_map(sokoban);
        true
    } else { false }
}

pub fn play(input: &String) -> Result<(), SokobanError> {
    show_welcome();
    let mut map = read_file(input)?;
    validate_file(&map)?;

    let mut sokoban = match Sokoban::new(&mut map) {
        Ok(s) => s,
        Err(error) => return Err(error),
    };

    while !won_game(&mut sokoban) {
        print_map(&mut sokoban);
        let input = match get_user_input() {
            Ok(i) => i,
            Err(_) => return Err(CommandError(ERR_GETTING_INPUT.to_string())),
        };
        if end_game(&input){
            break;
        }
        let movement: Move = process_input(&input);
        process_move(&mut sokoban, movement);
    }
    Ok(())
}
