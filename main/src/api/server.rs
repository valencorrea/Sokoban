use crate::api::file_service::read_file;
use crate::api::movement_service::{process_input, process_move};
use crate::api::sokoban_service::{Move, Sokoban, SokobanError};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    sokoban: Sokoban,
}

impl Server {
    pub fn create() -> Result<Server, SokobanError> {
        let mut map = match read_file(&"src/api/maps/level_1.txt".to_owned()) {
            Ok(result) => result,
            Err(error) => return Err(SokobanError::FileError("err".to_string())),
        };
        Ok(Server {
            sokoban: Sokoban::new(&mut map).unwrap(),
        })
    }

    pub fn run(mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:7878")?;

        println!("[SERVER] - Listening for connections on port 7878");

        self.sokoban.print_map();
        for stream in listener.incoming() {
            let stream = stream?;
            self.handle_client(stream);
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream) {
        stream.set_nonblocking(false); // FIXME No sé de que sirve esto
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

                let response = self.process_request(line);
                stream_clone.write_all(response.as_bytes());
            }
        }
        println!("READY TO JOIN");
    }

    fn process_request(&mut self, client_request: String) -> String {
        let request: Vec<&str> = client_request.split(" ").collect();
        if request[0] == "QUIT" {
            return String::from("QUIT\n");
        } else if request[0] == "MOVE" {
            let dir = request[1];
            let movement: Move = process_input(dir);
            process_move(&mut self.sokoban, movement);
            return String::from("MOVING!\n");
        } else {
            return String::from("Bad request\n");
        }
    }
}
