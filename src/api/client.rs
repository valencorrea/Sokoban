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

// TODO OK
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

// TODO OK
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
                println!("[SERVER]: {} {} ", ENTER_STR2, line);
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

pub fn run_from_gui(rx: Receiver<String>, tx: Sender<String>) -> Result<(), SokobanError> {
    show_welcome();

    let stream: TcpStream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("[SERVER-CONNECTION] Unsuccesful creation of TCP connection");
            return Ok(());
        }
    };

    let thread = listen_from_gui(tx, stream_clone).unwrap();

    let mut stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("[SERVER-CONNECTION] Unsuccesful creation of TCP connection");
            return Ok(());
        }
    };

    while let Ok(mssg) = rx.recv_timeout(time::Duration::from_secs(10)) {
        if let Err(e) = stream_clone.write_all(mssg.as_bytes()) {
            println!("Can't send message to server: {:?}", e);
            break;
        }
    }

    thread.join();
    Ok(())
}

fn listen_from_gui(tx: Sender<String>, stream: TcpStream) -> Result<JoinHandle<()>, SokobanError> {
    let s = match stream.try_clone() {
        Ok(v) => v,
        Err(_) => {
            tcp_destroy(stream)?;
            return Err(SokobanError::ConnectionError(
                "Can't create enough TCP streams to work properly".to_string(),
            ));
        }
    };

    let t = thread::spawn(move || {
        let _ = stream.set_nonblocking(true);
        let mut lines = BufReader::new(stream).lines();
        loop {
            if let Some(line) = lines.next() {
                match line {
                    Ok(p) => {
                        if let Err(e) = tx.send(p) {
                            println!("[CLIENT-MESSAGE SENDER] {:?}", e);
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(_) => {
                        println!("Disconnected from the server. Terminating [E1]");
                        break;
                    }
                }
            } else {
                println!("Disconnected from the server. Terminating [E2]");
                break;
            }
        }
    });
    Ok(t)
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
