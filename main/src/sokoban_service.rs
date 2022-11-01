use crate::command_service::{get_user_input, is_valid_input, QUIT};
use crate::file_service::{FileError, read_file, validate_file};
use crate::user_interface::user_goodbye;
use crate::user_welcome;

#[derive(Debug)]
struct Coord {
    x: u8,
    y: u8,
}

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

pub fn play(map: &String) -> Result<(), FileError> {
    user_welcome();

    let input_file = match read_file(map) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    validate_file(input_file)?;

    loop {
        let input: String = get_user_input();
        if input == QUIT {
            user_goodbye();
            break;
        }
    }

    Ok(())
}



