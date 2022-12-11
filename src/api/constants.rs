pub const AIR_U8: u8 = 32; // whitespace
pub const WALL_U8: u8 = 35; // #
pub const BOX_U8: u8 = 61; // =
pub const TARGET_U8: u8 = 43; // +
pub const BOX_ON_TARGET_U8: u8 = 42; // *
pub const PLAYER_U8: u8 = 80; // P
pub const ENTER_U8: u8 = 10;

pub const AIR_STR: &str = " ";
pub const BOX_STR: &str = "=";
pub const TARGET_STR: &str = "+";
pub const BOX_ON_TARGET_STR: &str = "*";
pub const WALL_STR: &str = "#";
pub const PLAYER_STR: &str = "P";
pub const ENTER_STR: &str = "\r";
pub const ENTER_STR2: &str = "\n";

pub const UP: &str = "W";
pub const DOWN: &str = "S";
pub const LEFT: &str = "A";
pub const RIGHT: &str = "D";
pub const QUIT: &str = "Q";

pub const ERR_GETTING_INPUT: &str = "Error while getting user input.\n";
pub const ERR_FILE_FORMAT: &str = "File format Error.\n";

pub const MAP_01: [[u8; 8]; 9] = [
    [0, 0, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 1, 0],
    [1, 1, 1, 0, 0, 0, 1, 0],
    [1, 0, 1, 1, 0, 0, 1, 0],
    [1, 0, 1, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];