use crate::api::utils::{delete_enters, ENTER_U8};

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
    let mut row = 0;
    let mut column = 0;

    input = delete_enters(input);

    while row < rows && !input.is_empty() {
        map[row][column] = input.remove(0) as u8; // todo mencionar casteos
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    map
}