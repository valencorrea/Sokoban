use crate::api::constants::{ENTER_STR, ENTER_STR2};
use crate::api::coord_service::Coord;

pub fn delete_enters(input: &mut String) -> String {
    let mut output: String = String::new();
    for i in input.chars() {
        if i.to_string() != ENTER_STR && i.to_string() != ENTER_STR2 {
            output.push_str(&*i.to_string());
        }
    }
    output
}

pub fn is_object(next_coord: &Coord, object_to_compare: u8, map: &Vec<Vec<u8>>) -> bool {
    return map[next_coord.y as usize][next_coord.x as usize] == object_to_compare
}