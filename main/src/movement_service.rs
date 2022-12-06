pub use crate::command_service::{DOWN, LEFT, UP};
use crate::sokoban_service::{Coord, Move, Sokoban, SokobanError};

pub fn process_input(input: &String) -> Move {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else {
        Move::Right
    }
}

pub fn process_move(sokoban: &mut Sokoban, movement: Move){
    let mut delta_x: i8 = 0;
    let mut delta_y: i8 = 0;

    match movement {
        Move::Up => delta_y = -1,
        Move::Left => delta_x = -1,
        Move::Down => delta_y = 1,
        Move::Right => delta_x = 1,
    }

    let coord_in_direction: Coord = Coord {
        x: (sokoban.user_coords.x as i8 + delta_x) as u8,
        y: (sokoban.user_coords.y as i8 + delta_y) as u8,
    };

    let coord_in_past_direction: Coord = Coord {
        x: (sokoban.user_coords.x as i8 + delta_x * 2) as u8,
        y: (sokoban.user_coords.y as i8 + delta_y * 2) as u8,
    };

    if is_wall(&coord_in_direction, &sokoban.map) {
        return;
    } else if is_box(&coord_in_direction, &sokoban.boxes_coords) {
        if is_wall(&coord_in_past_direction, &sokoban.map) {
            return;
        } else if is_box(&coord_in_past_direction, &sokoban.boxes_coords) {
            return;
        } else {
            move_player(&mut sokoban.user_coords, &coord_in_direction);
            move_box(&mut sokoban.boxes_coords, &coord_in_direction, &coord_in_past_direction);
        }
    } else {
        move_player(&mut sokoban.user_coords, &coord_in_direction);
        return;
    }
}

fn is_wall(coord: &Coord, map: &Vec<Vec<u8>>) -> bool {
    return map[coord.y as usize][coord.x as usize] == 1;
}

fn is_box(coord: &Coord, boxes_coords: &Vec<Coord>) -> bool {
    for box_coords in boxes_coords.iter() {
        if coord.x == box_coords.x && coord.y == box_coords.y {
            return true;
        }
    }
    return false;
}

fn move_player(player_coords: &mut Coord, new_cord: &Coord) {
    player_coords.x = new_cord.x;
    player_coords.y = new_cord.y;
}

fn move_box(boxes_coords: &mut Vec<Coord>, box_coords: &Coord, box_new_coords: &Coord) {
    for boxx in boxes_coords.iter_mut() {
        if boxx.x == box_coords.x && boxx.y == box_coords.y {
            boxx.x = box_new_coords.x;
            boxx.y = box_new_coords.y;
            return;
        }
    }
}