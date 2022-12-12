use core::time;
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{io, thread};

use crate::api::constants::{ENTER_STR2, TAB_STR};
use crate::api::utils::show_goodbye;

use super::sokoban::{Sokoban, SokobanError};
use super::utils::{ask_for_command, invalid_command, show_commands, show_victory, show_welcome};

fn is_valid_input(input: String) -> bool {
    if input.len() == 0 {
        return true;
    }

    let s: Vec<&str> = input.split(" ").collect();

    if s[0] == "QUIT" {
        return true;
    }

    if s[0] != "MOVE" {
        return false;
    }

    return s[1] == "W" || s[1] == "A" || s[1] == "S" || s[1] == "D";
}

pub fn run() -> std::io::Result<()> {
    let mut end_game = false;

    show_welcome();

    let stream: TcpStream = TcpStream::connect("127.0.0.1:7878")?;

    let mut stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("[SERVER-CONNECTION] Unsuccesful creation of TCP connection");
            return Ok(());
        }
    };

    let t = thread::spawn(move || {
        let buf_reader = BufReader::new(stream.try_clone().unwrap());
        let mut lines = buf_reader.lines();
        while !end_game {
            if let Some(Ok(l)) = lines.next() {
                let line = l.replace(TAB_STR, ENTER_STR2);
                println!("[SERVER]:{}{}", ENTER_STR2, line);
                if line.contains("CLOSING") {
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    end_game = true;
                } else if line.contains("VICTORY") {
                    show_victory();
                    end_game = true;
                }
            }
        }
    });

    let mut input: String = String::new();
    while !end_game {
        input.clear();
        show_commands();
        ask_for_command();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if !end_game && !is_valid_input(input.trim_end().to_owned()) {
            invalid_command();
            continue;
        }

        match stream_clone.write_all(input.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                end_game = true;
            }
        };

        if input.trim_end().to_owned() == "QUIT" {
            end_game = true;
        }
    }

    t.join().unwrap();

    show_goodbye();

    Ok(())
}

fn tcp_destroy(stream: TcpStream) -> Result<(), SokobanError> {
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("[CLIENT] Cleaned up TCP connection");
        }
        Err(_) => {
            return Err(SokobanError::ConnectionError(
                "Internal server error".to_string(),
            ))
        }
    }
    Ok(())
}
