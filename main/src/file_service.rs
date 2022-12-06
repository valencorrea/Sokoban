use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::io;
use crate::sokoban_service::SokobanError;
use crate::utils::{BOX_U8, ENTER_U8, TARGET_U8, WALL_U8};

// deprecado
#[derive(Debug)]
pub enum FileError {
    ReadError(String),
    WriteError(String),
    FormatError(String),
}

pub fn handle_file(input: &String) -> Result<&String, SokobanError> {
    let mut map = match read_file(input) {
        Ok(result) => result,
        Err(error) => return Err(SokobanError::FileError("err".to_string())),
    };
    return match validate_file(&map) {
        Ok(f) => Ok(f),
        Err(err) => Err(SokobanError::FileError("err".to_string())),
    };
}

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
        if (*char != BOX_U8) && (*char != WALL_U8) && (*char != TARGET_U8) && (*char != ENTER_U8) {
            return Err(SokobanError::FileError(String::from("Error en el formato del archivo.")))
        }
    }
    Ok(file)
}