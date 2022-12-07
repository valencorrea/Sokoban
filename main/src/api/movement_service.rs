use crate::api::sokoban_service::{Coord, Move, Sokoban};
use crate::api::utils::{BOX_U8, DOWN, LEFT, PLAYER_U8, TARGET_U8, UP, WALL_U8};

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

pub fn get_deltas(movement: Move) -> (i8, i8){
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

pub fn get_next_valid_coord(user_coords: &Coord, delta_x: i8, delta_y: i8, rows: &usize, columns: &usize) -> Coord {
    let mut new_coord_x =  user_coords.x as i8 + delta_x;
    let mut new_coord_y =  user_coords.y as i8 + delta_y;

    // si se va del mapa no lo dejamos hacer el movimiento
    if (new_coord_x >= *columns as i8) || (new_coord_x < 0){
        new_coord_x = user_coords.x as i8;
    };
    if (new_coord_y >= *rows as i8) || (new_coord_y < 0){
        new_coord_y = user_coords.y as i8;
    };
    Coord{ x: new_coord_x as usize, y: new_coord_y as usize }
}

pub fn is_object(next_coord: &Coord, object_to_compare: u8, map: &Vec<Vec<u8>>) -> bool {
    return if map[next_coord.x as usize][next_coord.y as usize] == object_to_compare {
        true
    } else {
        false
    }
}

pub fn process_move(sokoban: &mut Sokoban, movement: Move){
    // todo mencionar que se puede explicitar el tipo

    let (delta_x, delta_y) = get_deltas(movement);
    let mut next_coord: Coord = get_next_valid_coord(&sokoban.user_coords, delta_x, delta_y, &sokoban.rows, &sokoban.columns);

    if is_object(&next_coord, WALL_U8, &sokoban.map) {
        return;
    }

    if is_object(&next_coord, BOX_U8, &sokoban.map) {
        let next_next_coord = get_next_valid_coord(&next_coord, delta_x, delta_y, &sokoban.rows, &sokoban.columns);

        if is_object(&next_next_coord, WALL_U8, &sokoban.map) ||
            is_object(&next_next_coord, BOX_U8, &sokoban.map) {
            return;
        } else {
            move_box(&mut sokoban.map, &mut next_coord, &next_next_coord, &sokoban.target_coords);
        }
    }
    move_player(&mut sokoban.map, &mut sokoban.user_coords, &next_coord, &sokoban.target_coords);
}

fn move_object(map: &mut Vec<Vec<u8>>, coords_from: &mut Coord, coords_to: &Coord, target_coords: &Vec<Coord>, object: u8){
    map[coords_to.y][coords_to.x] = object;

    if target_coords.contains(coords_from){
        map[coords_from.y][coords_from.x] = TARGET_U8;
    }
}

fn move_player(map: &mut Vec<Vec<u8>>, coords_from: &mut Coord, coords_to: &Coord, target_coords: &Vec<Coord>){
    move_object(map, coords_from, coords_to,target_coords, PLAYER_U8);
}

// todo volar el vector de boxes coords
fn move_box(map: &mut Vec<Vec<u8>>, coords_from: &mut Coord, coords_to: &Coord, target_coords: &Vec<Coord>){
    move_object(map, coords_from, coords_to, target_coords, BOX_U8);
}