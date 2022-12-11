use crate::api::movement::Move;

#[derive(Debug, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn update_coords(coords_from: &mut Coord, coords_to: &Coord){
    coords_from.x = coords_to.x;
    coords_from.y = coords_to.y;
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

    if (new_coord_x >= *columns as i8) || (new_coord_x < 0) {
        new_coord_x = user_coords.x as i8;
    };
    if (new_coord_y >= *rows as i8) || (new_coord_y < 0) {
        new_coord_y = user_coords.y as i8;
    };
    Coord {
        x: new_coord_x as usize,
        y: new_coord_y as usize,
    }
}

pub fn equals_to(coord_1: &Coord, coord_2: &Coord) -> bool{
    return if (coord_1.x == coord_2.x)
        && (coord_1.y == coord_2.y) {
        true
    } else { false }
}