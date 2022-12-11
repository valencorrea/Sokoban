use crate::api::constants::{BOX_ON_TARGET_U8, BOX_U8, EMPTY_PLACE_U8, ENTER_U8, TARGET_U8};
use crate::api::coord_service::Coord;

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

pub fn refresh_map(
    map: &mut Vec<Vec<u8>>,
    coords_from: &mut Coord,
    coords_to: &Coord,
    target_coords: &Vec<Coord>,
    object: u8,
    boxes_on_target_coords: &Vec<Coord>,
    boxes_coords: &Vec<Coord>
) {
    map[coords_to.y][coords_to.x] = object;

    if target_coords.contains(coords_from) {
        map[coords_from.y][coords_from.x] = TARGET_U8;
    } else if boxes_coords.contains(coords_from) {
        map[coords_from.y][coords_from.x] = BOX_U8;
    } else if boxes_on_target_coords.contains(coords_from) {
        map[coords_from.y][coords_from.x] = BOX_ON_TARGET_U8;
    } else {
        map[coords_from.y][coords_from.x] = EMPTY_PLACE_U8;
    }
}

#[cfg(test)]
mod dimentions_tests {
    use super::*;

    #[test]
    fn map_nxm_cant_rows_returns_n() {
        assert_eq!(rows(&[42, 42, 42, ENTER_U8, 42, 42, 42, ENTER_U8]), 2);
    }

    #[test]
    fn map_nxm_cant_columns_returns_m() {
        assert_eq!(columns(8, &(2 as usize)), 3);
    }
}