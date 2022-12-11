use std::{net::{TcpListener, TcpStream}, thread, io::{BufReader, BufRead, Write}, sync::{Arc, Mutex, RwLock}};

use crate::api::movement::Move;

use super::{sokoban::Sokoban, coord::Coord};

#[derive(Debug)]
pub struct Server {
    sokoban: Mutex<Sokoban>,
}

impl Server {
    pub fn create_from_map(sokoban: Sokoban) -> Server {
        Server {
            sokoban: Mutex::new(sokoban),
        }
    }

    pub fn run(self) -> std::io::Result<()> {
        let s = Arc::new(self);

        {
            let ss = s.clone();
            
            ss.sokoban.lock().unwrap().print();
        }

        let listener = TcpListener::bind("0.0.0.0:7878")?;
    
        println!("[SERVER] - Listening for connections on port 7878");
    
        for stream in listener.incoming() {
            let stream = stream?;
            let ss = s.clone();
            thread::spawn(move || {
                Server::handle_client(ss, stream);
            });
            
        }
        Ok(())
    }

    fn handle_client(server: Arc<Server>, stream: TcpStream) {
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
    
                let request: Vec<&str> = line.split(" ").collect();
                if request[0] == "QUIT" {
                    let response = String::from("CLOSING");
                    stream_clone.write_all(response.as_bytes());
                    break;
                } else if request[0] == "MOVE" {
                    let input = request[1];
                    let movement: Move = process_input(&input);
                    Server::process_move(server.clone(), movement);

                    {
                        let s = server.clone();

                        let sok = s.sokoban.lock().unwrap();

                        sok.print();

                        if sok.victory(){
                            let response = String::from(
                                "Felicitaciones! Has vencido el juego. Gracias por jugar.\n",
                            );
                            stream_clone.write_all(response.as_bytes());
                            break;
                        }

                    }
            
                    let response = String::from("OK\n");
                    stream_clone.write_all(response.as_bytes());
                }
            }
        }

        println!("READY TO JOIN");
        
    }

    fn process_move(
        server: Arc<Server>,
        movement: Move,
    ) {
        let mut sok = server.sokoban.lock().unwrap();

        let mut delta_x: i8 = 0;
        let mut delta_y: i8 = 0;
        match movement {
            Move::Up => delta_y = -1,
            Move::Left => delta_x = -1,
            Move::Down => delta_y = 1,
            Move::Right => delta_x = 1,
        }
    
        let coord_in_direction: Coord = Coord {
            x: (sok.user_coords.x as i8 + delta_x as i8) as usize,
            y: (sok.user_coords.y as i8 + delta_y as i8) as usize,
        };
    
        let coord_in_past_direction: Coord = Coord {
            x: (sok.user_coords.x as i8 + delta_x * 2 as i8) as usize,
            y: (sok.user_coords.y as i8 + delta_y * 2 as i8) as usize,
        };

        if sok.is_wall(&coord_in_direction) { return; }
        else if sok.is_box(&coord_in_direction) {
            if sok.is_wall(&coord_in_past_direction) { return; } 
            else if sok.is_box(&coord_in_past_direction) { return; } 
            else {
                sok.move_player(&coord_in_direction);
                sok.move_box(&coord_in_direction, &coord_in_past_direction);
            }
        } else {
            sok.move_player(&coord_in_direction);
            return;
        }
        
    }

}

fn process_input(input: &str) -> Move {
    if input == "W" {
        return Move::Up;
    } else if input == "A" {
        return Move::Left;
    } else if input == "S" {
        return Move::Down;
    } else {
        return Move::Right;
    }
}