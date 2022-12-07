use crate::api::command_service::get_user_input;
use crate::api::file_service::{read_file, validate_file};
use crate::api::map_service::{create_map, get_dimentions};
use crate::api::movement_service::{process_input, process_move};
use crate::api::utils::delete_enters;
use crate::api::utils::{BOX_STR, PLAYER_STR, QUIT, TARGET_STR, WALL_U8};
use crate::api::ux::{print_map, show_goodbye, show_victory, show_welcome};
use std::fmt::Debug;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub enum SokobanError {
    CoordError(String),
    FileError(String),
    GTKError(String),
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
pub fn get_coords(
    mut coords: String,
    object: &str,
    rows: usize,
    columns: usize,
) -> Result<Vec<Coord>, SokobanError> {
    let mut row = 0;
    let mut column = 0;
    let mut coord_vec = vec![Coord { x: 0, y: 0 }]; // todo al final eliminar el primero

    /*if object == PLAYER_STR {
        return Ok(vec![Coord { x: 2, y: 2 }]);
    } else*/
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

    coords = delete_enters(coords); //Agregue esto
    while row < rows && !coords.is_empty() {
        if coords.remove(0).to_string() == object.to_string() {
            let new_coord = Coord {
                x: column as u8, // inverti x e y
                y: row as u8,
            };
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

impl Sokoban {
    pub fn new(map_bytes: &String) -> Result<Self, SokobanError> {
        let (rows, columns) = get_dimentions(map_bytes);
        let map = create_map(map_bytes.clone(), rows, columns); // todo mencionar desventajas

        let boxes_coords = match get_coords(map_bytes.clone(), BOX_STR, rows, columns) {
            Ok(b) => b,
            Err(err) => return Err(err),
        };
        let target_coords = match get_coords(map_bytes.clone(), TARGET_STR, rows, columns) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };
        let player_coords = match get_coords(map_bytes.clone(), PLAYER_STR, rows, columns) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };

        //sokoban = initialize_coords(sokoban, &boxes_coords, BOX_U8);
        //sokoban = initialize_coords(sokoban, &target_coords, TARGET_U8);

        Ok(Sokoban {
            map,
            user_coords: Coord {
                x: player_coords[0].x,
                y: player_coords[0].y,
            },
            boxes_coords,
            target_coords,
            rows,
            columns,
        })
    }

    pub fn print_map(&self) {
        println!("{:?}", self.user_coords);
        for j in 0..self.rows {
            /*pub map: Vec<Vec<u8>>,
            pub user_coords: Coord,
            pub boxes_coords: Vec<Coord>,
            pub target_coords: Vec<Coord>,*/
            let row: &Vec<u8> = &self.map[j];
            for i in 0..self.columns {
                let cell: u8 = row[i];
                let coord: Coord = Coord {
                    x: i as u8,
                    y: j as u8,
                };
                if cell == WALL_U8 {
                    print!("#");
                } else if is_player(&coord, &self.user_coords) {
                    print!("P");
                } else if is_box(&coord, &self.boxes_coords) {
                    if is_target(&coord, &self.target_coords) {
                        print!("*");
                    } else {
                        print!("=");
                    }
                } else if is_target(&coord, &self.target_coords) {
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

// todo refactorizar
pub fn play(input: &String) -> Result<(), SokobanError> {
    show_welcome();
    let mut map_bytes = match read_file(input) {
        Ok(result) => result,
        Err(error) => return Err(SokobanError::FileError("err".to_string())),
    };
    validate_file(&map_bytes)?;
    print_map(&map_bytes.clone()); // todo refactor

    let mut sokoban = match Sokoban::new(&map_bytes) {
        Ok(s) => s,
        Err(err) => return Err(err),
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
        if true {
            //victory(&boxes_coords, &boxes_targets) {
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
