use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use super::utils::{invalid_command, show_commands, show_welcome};

// TODO OK
fn is_valid_input(input: String) -> bool {
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
    show_welcome();

    let stream: TcpStream = TcpStream::connect("127.0.0.1:7878")?;

    let mut stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("[SERVER-CONNECTION] Unsuccesful creation of TCP connection");
            return Ok(());
        }
    };

    let buf_reader = BufReader::new(stream);
    let mut lines = buf_reader.lines();

    let mut input: String = String::new();
    loop {
        input.clear();
        show_commands();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if !is_valid_input(input.trim_end().to_owned()) {
            invalid_command();
            continue;
        }

        stream_clone.write_all(input.as_bytes())?;
        if let Some(l) = lines.next() {
            let line = match l {
                Ok(p) => p,
                Err(_) => {
                    println!("[CLIENT] - Connection closed");
                    return Ok(());
                }
            };
            println!("[SERVER]: {} ", line);
            if line.contains("CLOSING") {
                println!("[CLIENT] - Connection closed"); // TODO refactor
                return Ok(());
            }
        }
    }
}
