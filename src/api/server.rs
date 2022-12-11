use std::{net::{TcpListener, TcpStream}, thread, io::{BufReader, BufRead, Write}, sync::{Arc, Mutex}};

use crate::api::movement::Move;

use super::{sokoban::Sokoban, coord::Coord};

#[derive(Debug)]
pub struct Server {
    sokoban: Arc<Mutex<Sokoban>>,
}

impl Server {
    pub fn create_from_map(sokoban: Sokoban) -> Server {
        Server {
            sokoban: Arc::new(Mutex::new(sokoban)),
        }
    }

    pub fn run(self) -> std::io::Result<()> {
        let s = Arc::new(self);

        {
            let ss = s.clone();

            let sok = Arc::new(ss.sokoban.lock().unwrap()); 

            Sokoban::print(sok.clone());
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
                        let sok = Arc::new(server.sokoban.lock().unwrap()); 

                        Sokoban::print(sok.clone());

                        if Sokoban::victory(sok.clone()){
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
        let sok = Arc::new(server.sokoban.lock().unwrap());

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

        if Sokoban::is_wall(sok.clone(), &coord_in_direction) { return; }
        else if Sokoban::is_box(sok.clone(), &coord_in_direction) {
            if Sokoban::is_wall(sok.clone(), &coord_in_past_direction) { return; } 
            else if Sokoban::is_box(sok.clone(), &coord_in_past_direction) { return; } 
            else {
                Sokoban::move_player(server.sokoban.lock().unwrap(), &coord_in_direction);
                Sokoban::move_box(server.sokoban.lock().unwrap(), &coord_in_direction, &coord_in_past_direction);
            }
        } else {
            Sokoban::move_player(server.sokoban.lock().unwrap(), &coord_in_direction);
            return;
        }
        
    }

}


// fn print_map(
//     map: &[[u8; 8]; 9],
//     boxes_coords: &[Coord; 7],
//     boxes_targets: &[Coord; 7],
//     player_coords: &Coord,
// ) {
//     for j in 0..map.len() {
//         let row: &[u8; 8] = &map[j];
//         for i in 0..row.len() {
//             let cell: u8 = row[i];
//             let coord: Coord = Coord {
//                 x: i as u8,
//                 y: j as u8,
//             };
//             if cell == 1 {
//                 print!("#");
//             } else if is_player(&coord, &player_coords) {
//                 print!("P");
//             } else if is_box(&coord, &boxes_coords) {
//                 if is_target(&coord, &boxes_targets) {
//                     print!("*");
//                 } else {
//                     print!("=");
//                 }
//             } else if is_target(&coord, &boxes_targets) {
//                 print!("+");
//             } else {
//                 print!(" ");
//             }
//             print!(" ");
//         }
//         println!("");
//     }
// }

// fn get_user_input() -> String {
//     let mut input: String = String::new();
//     loop {
//         input.clear();
//         println!("Escribe tu movimiento (WASD) o Q para cerrar el juego:");
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let trimmed_len: usize = input.trim_end().len();
//         input.truncate(trimmed_len);

//         if is_valid_input(&input) {
//             return input;
//         }
//     }
// }

// fn is_valid_input(input: &String) -> bool {
//     return input == "W" || input == "A" || input == "S" || input == "D" || input == "Q";
// }

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


// fn is_player(coord: &Coord, player_coords: &Coord) -> bool {
//     if coord.x == player_coords.x && coord.y == player_coords.y {
//         return true;
//     } else {
//         return false;
//     }
// }

// fn is_target(coord: &Coord, boxes_targets: &[Coord; 7]) -> bool {
//     for box_target in boxes_targets.iter() {
//         if coord.x == box_target.x && coord.y == box_target.y {
//             return true;
//         }
//     }
//     return false;
// }



// /*fn process_request(&mut self, client_request: String) -> String {
//     let request: Vec<&str> = client_request.split(" ").collect();
//     if request[0] == "QUIT" {
//         return String::from("QUIT\n");
//     } else if request[0] == "MOVE" {
//         let dir = request[1];
//         let movement: Move = process_input(dir);
//         process_move(&mut self.sokoban, movement);
//         return String::from("MOVING!\n");
//     } else {
//         return String::from("Bad request\n");
//     }
// }*/
