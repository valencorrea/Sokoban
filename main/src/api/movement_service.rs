use crate::api::sokoban_service::{Coord, Move, Sokoban};
use crate::api::utils::{DOWN, LEFT, UP};

pub fn process_input(input: &str) -> Move {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else {
        Move::Right
    };
}

pub fn get_deltas(movement: Move) -> (i8, i8) {
    let mut delta_x: i8 = 0;
    let mut delta_y: i8 = 0;

    match movement {
        Move::Up => delta_y = -1,
        Move::Left => delta_x = -1,
        Move::Down => delta_y = 1,
        Move::Right => delta_x = 1,
    };
    (delta_x, delta_y)
}

pub fn get_next_valid_coord(
    user_coords: &Coord,
    delta_x: i8,
    delta_y: i8,
    rows: &usize,
    columns: &usize,
) -> Coord {
    let mut new_coord_x = user_coords.x as i8 + delta_x;
    let mut new_coord_y = user_coords.y as i8 + delta_y;

    // si se va del mapa no lo dejamos hacer el movimiento
    if (new_coord_x >= *columns as i8) || (new_coord_x < 0) {
        new_coord_x = user_coords.x as i8;
    };
    if (new_coord_y >= *rows as i8) || (new_coord_y < 0) {
        new_coord_y = user_coords.y as i8;
    };
    Coord {
        x: new_coord_x as u8,
        y: new_coord_y as u8,
    }
}

pub fn get_previous_valid_coord(user_coords: &Coord, delta_x: i8, delta_y: i8) -> Coord {
    let past_coord_x = (user_coords.x as i8 + delta_x * 2) as u8;
    let past_coord_y = (user_coords.y as i8 + delta_y * 2) as u8;
    Coord {
        x: past_coord_x,
        y: past_coord_y,
    }
}

pub fn process_move(sokoban: &mut Sokoban, movement: Move) {
    // todo mencionar que se puede explicitar el tipo

    let (delta_x, delta_y) = get_deltas(movement);
    let coord_in_direction: Coord = get_next_valid_coord(
        &sokoban.user_coords,
        delta_x,
        delta_y,
        &sokoban.rows,
        &sokoban.columns,
    );
    let coord_in_past_direction: Coord =
        get_previous_valid_coord(&sokoban.user_coords, delta_x, delta_y);

    if is_wall(&coord_in_direction, &sokoban.map) {
        return;
    } else if is_box(&coord_in_direction, &sokoban.boxes_coords) {
        if is_wall(&coord_in_past_direction, &sokoban.map) {
            return;
        } else if is_box(&coord_in_past_direction, &sokoban.boxes_coords) {
            return;
        } else {
            move_player(&mut sokoban.user_coords, &coord_in_direction);
            move_box(
                &mut sokoban.boxes_coords,
                &coord_in_direction,
                &coord_in_past_direction,
            );
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
