use crate::api::file_service::{read_file, validate_file};
use crate::api::constants::{BOX_STR, WALL_U8, TARGET_STR, PLAYER_STR, QUIT, BOX_U8, BOX_ON_TARGET_STR, ENTER_STR2};
use crate::api::utils::delete_enters;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::{MutexGuard, Arc};
use crate::api::coord::Coord;

use super::constants::{WALL_STR, PLAYER_U8, TARGET_U8, BOX_ON_TARGET_U8, AIR_STR, ENTER_U8};

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

impl Sokoban {
    pub fn create_from_path(argv: &String) -> Result<Self, SokobanError> {
        let mut map = read_file(argv)?;
        validate_file(&map)?;

        let mut input = map.to_owned();

        let (rows, columns) = get_dimentions(&input);
        let input = delete_enters(&mut input);
        let mut map = create_map(input.clone(), rows, columns);

        let mut target_coords = get_coords(input.clone(), TARGET_STR, rows, columns)?;
        let boxes_on_target_coords = get_coords(input.clone(), BOX_ON_TARGET_STR, rows, columns)?;
        append_targets(&mut target_coords, &boxes_on_target_coords);

        let boxes_coords = get_coords(input.clone(), BOX_STR, rows, columns)?;
        let mut vec_user_coords = get_coords(input.clone(), PLAYER_STR, rows, columns)?;

        Ok(Sokoban {
            map: map,
            user_coords: vec_user_coords.remove(0),
            target_coords: target_coords,
            boxes_coords: boxes_coords,
            boxes_on_target_coords: boxes_on_target_coords,
            rows: rows,
            columns: columns,
        })

    }

    pub fn print(&self) {
        let mut str_map = String::new();
        for row in 0..self.rows {
            for column in 0..self.columns {
                let object = get_object(self.map[row][column]);
                str_map.push(object.parse().unwrap());
            }
            str_map.push_str(ENTER_STR2);
        }
    
        println!("{}", str_map);    
    }
  
    pub fn is_wall(&self, coords: &Coord) -> bool {
        return self.map[coords.y as usize][coords.x as usize] == 1;
    }

    pub fn is_box(&self, coord: &Coord) -> bool {
        for box_coords in self.boxes_coords.iter() {
            if coord.x == box_coords.x && coord.y == box_coords.y {
                return true;
            }
        }
        return false;
    }
    
    pub fn move_player(&mut self, new_cord: &Coord) {
        self.user_coords.x = new_cord.x;
        self.user_coords.y = new_cord.y;
    }
    
    pub fn move_box(&mut self, box_coords: &Coord, box_new_coords: &Coord) {
        self.boxes_coords.iter_mut().for_each(|boxx| {
            if boxx.x == box_coords.x && boxx.y == box_coords.y {
                boxx.x = box_new_coords.x;
                boxx.y = box_new_coords.y;
                return;
            }
        });
    }

    pub fn victory(&self) -> bool {
        for box_coords in self.boxes_coords.iter() {
            let mut placed: bool = false;
            for box_target in self.boxes_on_target_coords.iter() {
                if box_coords.x == box_target.x && box_coords.y == box_target.y {
                    placed = true;
                    break;
                }
            }
            if !placed {
                return false;
            }
        }
        return true;
    }

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

    coords = delete_enters(&mut coords);

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

fn append_targets(targets: &mut Vec<Coord>, boxes_on_targets: &Vec<Coord>){
    for box_on_target in boxes_on_targets {
        let coord = Coord{x:box_on_target.x, y:box_on_target.y};
        targets.push(coord);
    }
}

pub fn get_object(map_object: u8) -> &'static str {
    return if map_object == WALL_U8 {
        WALL_STR
    } else if map_object == PLAYER_U8 {
        PLAYER_STR
    } else if map_object == BOX_U8 {
        BOX_STR
    } else if map_object == TARGET_U8 {
        TARGET_STR
    } else if map_object == BOX_ON_TARGET_U8 {
        BOX_ON_TARGET_STR
    } else {
        AIR_STR
    };
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
    let mut map_str = vec![vec![' '; columns]; rows]; // todo para que lo usamos?
    let mut row = 0;
    let mut column = 0;

    while row < rows && !input.is_empty() {
        let cell = input.remove(0); // todo mencionar casteos
        map[row][column] = cell as u8;
        map_str[row][column] = cell;
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    map
}