use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::io;
use crate::utils::{BOX_U8, ENTER_U8, TARGET_U8, WALL_U8};

#[derive(Debug)]
pub enum FileError {
    ReadError(String),
    WriteError(String),
    FormatError(String),
}

pub fn read_file(path: &String) -> Result<String, FileError> {
    let f = File::open(path);

    let mut f = match f {
        Ok(archivo) => archivo,
        Err(error) => return Err(FileError::ReadError(error.to_string())),
    };

    let mut read_file = String::new();
    match f.read_to_string(&mut read_file) {
        Ok(_) => {
            read_file = read_file.add("\n");
            Ok(read_file)
        }
        Err(error) => Err(FileError::WriteError(error.to_string())),
    }
}

pub fn validate_file(file: String) -> Result<String, FileError> {
    for char in file.as_bytes() {
        if (*char != BOX_U8) && (*char != WALL_U8) && (*char != TARGET_U8) && (*char != ENTER_U8) {
            return Err(FileError::FormatError(String::from("Error en el formato del archivo.")))
        }
    }
    Ok(file)
}

pub fn get_command() -> Result<String, FileError> {
    let mut command: String = String::new();
    match io::stdin()
        .read_line(&mut command){
        Ok(c) => c,
        Err(error) => return Err(FileError::ReadError(error.to_string())),
    };

    Ok(command)
}