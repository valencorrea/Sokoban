use crate::api::sokoban_service::Sokoban;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Server {
    sokoban: Sokoban,
}

impl Server {
    pub fn create() -> Server {
        Server {
            sokoban: Sokoban::new(&"map_easy".to_owned()).unwrap(),
        }
    }

    pub fn run(self) -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:7878")?;

        println!("[SERVER] - Listening for connections on port 7878");

        for stream in listener.incoming() {
            let stream = stream?;
            thread::spawn(|| handle_client(stream));
        }
        Ok(())
    }
}

fn process_request(client_request: String) -> String {
    let request: Vec<&str> = client_request.split(" ").collect();
    if request[0] == "QUIT" {
        return String::from("QUIT\n");
    } else if request[1] == "MOVE" {
        return String::from("MOVING!\n");
    } else {
        return String::from("Bad request\n");
    }
}

fn handle_client(stream: TcpStream) {
    stream.set_nonblocking(false);
    let client_addr = match stream.peer_addr() {
        Ok(sa) => sa.to_string(),
        Err(_) => "Unknown".to_owned(),
    };

    println!("New Connection: {}", client_addr);

    let mut stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("[SERVER-CONNECTION] Unsuccesful creation of TCP connection");
            return;
        }
    };

    let buf_reader = BufReader::new(stream);
    let mut lines = buf_reader.lines();

    loop {
        if let Some(l) = lines.next() {
            let line = match l {
                Ok(p) => p,
                Err(_) => {
                    break;
                }
            };

            println!("[{}]: {} ", client_addr, line);

            let response = process_request(line);
            stream_clone.write_all(response.as_bytes());
        }
    }
    println!("READY TO JOIN");
}
