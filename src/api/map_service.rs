use crate::api::constants::{ENTER_U8};

pub fn rows(bytes: &[u8]) -> usize {
    let mut rows = 0;

    for row in bytes {
        if *row == ENTER_U8 {
            rows += 1;
        }
    }
    rows
}

pub fn columns(total_bytes: usize, rows: &usize) -> usize {
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
