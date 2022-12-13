use crate::api::ux::get_object;
use std::str;

use super::{
    constants::{
        AIR_U8, BOX_ON_TARGET_STR, BOX_ON_TARGET_U8, BOX_STR, BOX_U8, PLAYER_STR, PLAYER_U8,
        TAB_STR, TARGET_STR, TARGET_U8, WALL_U8,
    },
    coord_service::{equals_to, get_deltas, get_next_coord, Coord},
    file_service::{read_file, validate_file},
    sokoban_service::{create_map, get_dimentions},
    utils::{delete_enters, Move},
};

#[derive(Debug, PartialEq, Eq)]
pub enum SokobanError {
    File(String),
    Command(String),
    Connection(String),
}

#[derive(Debug)]
pub struct Sokoban {
    pub map: Vec<Vec<u8>>,
    pub user_coords: Coord,
    pub target_coords: Vec<Coord>,
    pub boxes_coords: Vec<Coord>,
    pub rows: usize,
    pub columns: usize,
}

impl Sokoban {
    pub fn create_from_path(argv: &String) -> Result<Self, SokobanError> {
        let map = read_file(argv)?;
        validate_file(&map)?;

        let mut input = map;

        let (rows, columns) = get_dimentions(&input);
        let input = delete_enters(&mut input);
        let map = create_map(input.clone(), rows, columns);

        let mut target_coords = get_coords(input.clone(), TARGET_STR, rows, columns)?;
        let mut boxes_on_target_coords = get_coords(input.clone(), BOX_ON_TARGET_STR, rows, columns)?;
        target_coords.append(&mut boxes_on_target_coords.clone());

        let boxes_coords = get_coords(input.clone(), BOX_STR, rows, columns)?;
        target_coords.append(&mut boxes_on_target_coords);

        let mut vec_user_coords = get_coords(input, PLAYER_STR, rows, columns)?;

        Ok(Sokoban {
            map,
            user_coords: vec_user_coords.remove(0),
            target_coords,
            boxes_coords,
            rows,
            columns,
        })
    }

    pub fn to_str(&self) -> String {
        let mut str_map = String::new();
        for row in 0..self.rows {
            for column in 0..self.columns {
                let object = get_object(self.map[row][column]);
                str_map.push(object.parse().unwrap());
            }
            str_map.push_str(TAB_STR);
        }

        str_map
    }

    pub fn _print(&self) {
        println!("{}", self.to_str());
    }

    fn refresh_map(&mut self, coords_from: &Coord, coords_to: &Coord, object: u8) {
        self.map[coords_to.y][coords_to.x] = object;

        if self.target_coords.contains(coords_from) {
            self.map[coords_from.y][coords_from.x] = TARGET_U8;
        } else {
            self.map[coords_from.y][coords_from.x] = AIR_U8;
        }
    }

    fn move_player(&mut self, coords_to: &Coord) {
        self.refresh_map(&self.user_coords.to_owned(), coords_to, PLAYER_U8);
        self.user_coords = coords_to.to_owned();
    }

    fn move_box(&mut self, coords_from: &mut Coord, coords_to: &mut Coord) {
        let move_to_target = self.is_object(coords_to, TARGET_U8);
        let move_from_target = self.is_object(coords_from, BOX_ON_TARGET_U8);

        match self
            .boxes_coords
            .iter()
            .position(|b| equals_to(b, coords_from))
        {
            None => {}
            Some(index_to_remove) => {
                self.boxes_coords.remove(index_to_remove);
                self.boxes_coords.push(Coord {
                    x: coords_to.x,
                    y: coords_to.y,
                });
            }
        }
        self.refresh_map(coords_from, coords_to, BOX_U8);
        if move_from_target {
            self.map[coords_to.y][coords_to.x] = BOX_U8;
        }
        if move_to_target {
            self.map[coords_to.y][coords_to.x] = BOX_ON_TARGET_U8;
        }
    }

    pub fn process_move(&mut self, movement: Move) -> String {
        let (delta_x, delta_y) = get_deltas(movement);
        let mut next_coord: Coord = get_next_coord(&self.user_coords, delta_x, delta_y);
        let mut next_next_coord = get_next_coord(&next_coord, delta_x, delta_y);

        if self.is_object(&next_coord, WALL_U8) {
            return self.to_str();
        }

        if self.is_object(&next_coord, BOX_U8) || self.is_object(&next_coord, BOX_ON_TARGET_U8) {
            if !(self.is_object(&next_next_coord, AIR_U8)
                || self.is_object(&next_next_coord, TARGET_U8))
            {
                return self.to_str();
            }
            self.move_box(&mut next_coord, &mut next_next_coord);
        }
        self.move_player(&next_coord);

        self.to_str()
    }

    pub fn is_object(&self, next_coord: &Coord, object_to_compare: u8) -> bool {
        self.map[next_coord.y as usize][next_coord.x as usize] == object_to_compare
    }

    pub fn victory(&self) -> bool {
        for box_coords in self.boxes_coords.iter() {
            let mut found = false;
            for target in &self.target_coords {
                if equals_to(box_coords, target) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true
    }
}

pub fn get_coords(
    mut map_string: String,
    object: &str,
    rows: usize,
    columns: usize,
) -> Result<Vec<Coord>, SokobanError> {
    let mut row = 0;
    let mut column = 0;
    let mut coord_vec = Vec::new();

    while row < rows && !map_string.is_empty() {
        if map_string.remove(0).to_string() == *object {
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
    Ok(coord_vec)
}
