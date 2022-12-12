use crate::api::command_service::get_user_input;
use crate::api::file_service::{read_file, validate_file};
use crate::api::map_service::{create_map, get_dimentions, refresh_map};
use crate::api::_movement_service::{process_input};
use crate::api::constants::{BOX_STR, WALL_U8, TARGET_STR, PLAYER_STR, QUIT, BOX_U8, BOX_ON_TARGET_STR, ERR_GETTING_INPUT, BOX_ON_TARGET_U8, TARGET_U8, PLAYER_U8, AIR_U8};
use crate::api::utils::{delete_enters, is_object};
use crate::api::ux::{print_map, show_goodbye, show_victory, show_welcome};
use std::fmt::Debug;
use crate::api::coord_service::{Coord, equals_to, get_deltas, get_next_coord, update_coords};
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
    pub rows: usize,
    pub columns: usize,
}

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
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
        // todo refactor
        if map_string.remove(0).to_string() == object.to_string() {
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

impl Sokoban {
    pub fn new(input: &mut String) -> Result<Self, SokobanError> {
        let (rows, columns) = get_dimentions(input);
        let input = delete_enters(input);
        let mut map = create_map(input.clone(), rows, columns);

        let mut target_coords = get_coords(input.clone(), TARGET_STR, rows, columns)?;
        let mut boxes_on_target_coords = get_coords(input.clone(), BOX_ON_TARGET_STR, rows, columns)?;
        target_coords.append(&mut boxes_on_target_coords.clone());

        let mut boxes_coords = get_coords(input.clone(), BOX_STR, rows, columns)?;
        boxes_coords.append(&mut boxes_on_target_coords);

        let mut vec_user_coords = get_coords(input.clone(), PLAYER_STR, rows, columns)?;

        Ok(Sokoban {
            map,
            user_coords: vec_user_coords.remove(0), // todo refactor
            target_coords,
            boxes_coords,
            rows,
            columns,
        })
    }
}

fn end_game(input: &String) -> bool {
    input == QUIT
}

pub fn won_game(sokoban: &mut Sokoban) -> bool {
    for box_coords in &sokoban.boxes_coords {
        let mut found = false;
        for target in &sokoban.target_coords {
            if equals_to(box_coords, target) {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    return true;
}

pub fn process_move(sokoban: &mut Sokoban, movement: Move) {
    let (delta_x, delta_y) = get_deltas(movement);
    let mut next_coord: Coord = get_next_coord(&sokoban.user_coords, delta_x, delta_y);
    let mut next_next_coord = get_next_coord(&next_coord, delta_x, delta_y);

    if is_object(&next_coord, WALL_U8, &sokoban.map) {
        return;
    }

    if is_object(&next_coord, BOX_U8, &sokoban.map)
        || is_object(&next_coord, BOX_ON_TARGET_U8, &sokoban.map) {

        if !(is_object(&next_next_coord, AIR_U8, &sokoban.map)
            || is_object(&next_next_coord, TARGET_U8, &sokoban.map)){
            return;
        }
        move_box(&mut sokoban.map, &mut next_coord, &mut next_next_coord, &sokoban.target_coords, &mut sokoban.boxes_coords);
    }
    move_player(&mut sokoban.map, &mut sokoban.user_coords, &next_coord, &sokoban.target_coords);
}

fn move_player(map: &mut Vec<Vec<u8>>, coords_from: &mut Coord, coords_to: &Coord, target_coords: &Vec<Coord>) {
    refresh_map(map, coords_from, coords_to, target_coords, PLAYER_U8);
    update_coords(coords_from, coords_to);
}

fn move_box(
    map: &mut Vec<Vec<u8>>, coords_from: &mut Coord, coords_to: &mut Coord, target_coords: &Vec<Coord>,
    boxes_coords: &mut Vec<Coord>) {
    let move_to_target = is_object(&coords_to, TARGET_U8, map);
    let move_from_target = is_object(&coords_from, BOX_ON_TARGET_U8, map);

    match boxes_coords.iter().position(|b| equals_to(b, coords_from)){
        None => {}
        Some(index_to_remove) => {
            boxes_coords.remove(index_to_remove);
            boxes_coords.push(Coord{x:coords_to.x, y:coords_to.y});
        }
    }
    refresh_map(map, coords_from, coords_to, target_coords, BOX_U8);
    if move_from_target{
        map[coords_to.y][coords_to.x] = BOX_U8;
    }
    if move_to_target{
        map[coords_to.y][coords_to.x] = BOX_ON_TARGET_U8;
    }
}

pub fn play(input: &String) -> Result<(), SokobanError> {
    show_welcome();
    let mut map = read_file(input)?;
    validate_file(&map)?;

    let mut sokoban = match Sokoban::new(&mut map) {
        Ok(s) => s,
        Err(error) => return Err(error),
    };

    print_map(&mut sokoban);
    while !won_game(&mut sokoban) {
        let input = match get_user_input() {
            Ok(i) => i,
            Err(_) => return Err(CommandError(ERR_GETTING_INPUT.to_string())),
        };
        if end_game(&input){
            break;
        }
        let movement: Move = process_input(&input);
        process_move(&mut sokoban, movement);
        print_map(&mut sokoban);
    }
    show_goodbye();
    Ok(())
}
