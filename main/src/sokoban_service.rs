use crate::file_service::{FileError, read_file, validate_file};
use crate::user_welcome;


pub fn play(map: &String) -> Result<(), FileError> {
    user_welcome();

    let input_file = match read_file(map) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    validate_file(input_file)?;
    Ok(())
}