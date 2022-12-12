use crate::api::command_service::valid_map_object;
use crate::api::constants::{
    AIR_U8, BOX_ON_TARGET_U8, BOX_U8, ENTER_U8, ERR_FILE_FORMAT, PLAYER_U8, TARGET_U8, WALL_U8,
};
use std::fs::File;
use std::io::Read;
use std::ops::Add;

use super::sokoban::SokobanError;

pub fn read_file(path: &String) -> Result<String, SokobanError> {
    let f = File::open(path);

    let mut f = match f {
        Ok(archivo) => archivo,
        Err(error) => return Err(SokobanError::FileError(error.to_string())),
    };

    let mut read_file = String::new();
    match f.read_to_string(&mut read_file) {
        Ok(_) => {
            read_file = read_file.add("\n");
            Ok(read_file)
        }
        Err(error) => Err(SokobanError::FileError(error.to_string())),
    }
}

pub fn validate_file(file: &String) -> Result<&String, SokobanError> {
    for char in file.as_bytes() {
        if !valid_map_object(*char) {
            return Err(SokobanError::FileError(ERR_FILE_FORMAT.to_string()));
        }
    }
    Ok(file)
}
