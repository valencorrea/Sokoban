pub const WALL_U8: u8 = 87;
pub const BOX_U8: u8 = 66;
pub const TARGET_U8: u8 = 84;
pub const PLAYER_U8: u8 = 80;
pub const ENTER_U8: u8 = 10;
pub const EMPTY_PLACE_U8: u8 = 46;

pub const BOX_STR: &str = "B";
pub const TARGET_STR: &str = "T";
pub const WALL_STR: &str = "W";
pub const PLAYER_STR: &str = "P";
pub const ENTER_STR: &str = "\n";
pub const EMPTY_PLACE_STR: &str = ".";

pub const UP: &str = "W";
pub const DOWN: &str = "S";
pub const LEFT: &str = "A";
pub const RIGHT: &str = "D";
pub const QUIT: &str = "Q";

pub fn delete_enters(input: &mut String) -> String {
    let mut output: String = String::new();
    for i in input.chars() {
        if i.to_string() != ENTER_STR {
            output.push_str(&*i.to_string());
        }
    }
    output
}