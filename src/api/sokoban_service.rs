use crate::api::constants::ENTER_U8;

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
    let mut row = 0;
    let mut column = 0;

    while row < rows && !input.is_empty() {
        let cell = input.remove(0);
        map[row][column] = cell as u8;
        if column == columns - 1 {
            column = 0;
            row += 1;
        } else {
            column += 1;
        }
    }
    map
}

#[cfg(test)]
mod dimentions_tests {
    use crate::api::constants::ENTER_U8;

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
