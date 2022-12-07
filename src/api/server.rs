/*use crate::api::file_service::read_file;
use crate::api::movement_service::{process_input, process_move};
use crate::api::sokoban_service::{Move, Sokoban, SokobanError};*/
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::io;
use crate::api::constants::MAP_01;

struct Coord {
    x: u8,
    y: u8,
}

enum Move {
    Up,
    Left,
    Down,
    Right,
}

fn victory(boxes_coords: &[Coord; 7], boxes_targets: &[Coord; 7]) -> bool {
    for box_coords in boxes_coords.iter() {
        let mut placed: bool = false;
        for box_target in boxes_targets.iter() {
            if box_coords.x == box_target.x && box_coords.y == box_target.y {
                placed = true;
                break;
            }
        }
        if !placed {
            return false;
        }
    }
    return true;
}

fn print_map(
    map: &[[u8; 8]; 9],
    boxes_coords: &[Coord; 7],
    boxes_targets: &[Coord; 7],
    player_coords: &Coord,
) {
    for j in 0..map.len() {
        let row: &[u8; 8] = &map[j];
        for i in 0..row.len() {
            let cell: u8 = row[i];
            let coord: Coord = Coord {
                x: i as u8,
                y: j as u8,
            };
            if cell == 1 {
                print!("#");
            } else if is_player(&coord, &player_coords) {
                print!("P");
            } else if is_box(&coord, &boxes_coords) {
                if is_target(&coord, &boxes_targets) {
                    print!("*");
                } else {
                    print!("=");
                }
            } else if is_target(&coord, &boxes_targets) {
                print!("+");
            } else {
                print!(" ");
            }
            print!(" ");
        }
        println!("");
    }
}

fn get_user_input() -> String {
    let mut input: String = String::new();
    loop {
        input.clear();
        println!("Escribe tu movimiento (WASD) o Q para cerrar el juego:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_len: usize = input.trim_end().len();
        input.truncate(trimmed_len);

        if is_valid_input(&input) {
            return input;
        }
    }
}

fn is_valid_input(input: &String) -> bool {
    return input == "W" || input == "A" || input == "S" || input == "D" || input == "Q";
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

fn process_move(
    map: &[[u8; 8]; 9],
    boxes_coords: &mut [Coord; 7],
    player_coords: &mut Coord,
    movement: Move,
) {
    let mut delta_x: i8 = 0;
    let mut delta_y: i8 = 0;
    match movement {
        Move::Up => delta_y = -1,
        Move::Left => delta_x = -1,
        Move::Down => delta_y = 1,
        Move::Right => delta_x = 1,
    }

    let coord_in_direction: Coord = Coord {
        x: (player_coords.x as i8 + delta_x) as u8,
        y: (player_coords.y as i8 + delta_y) as u8,
    };

    let coord_in_past_direction: Coord = Coord {
        x: (player_coords.x as i8 + delta_x * 2) as u8,
        y: (player_coords.y as i8 + delta_y * 2) as u8,
    };

    if is_wall(&coord_in_direction, &map) {
        return;
    } else if is_box(&coord_in_direction, &boxes_coords) {
        if is_wall(&coord_in_past_direction, &map) {
            return;
        } else if is_box(&coord_in_past_direction, &boxes_coords) {
            return;
        } else {
            move_player(player_coords, &coord_in_direction);
            move_box(boxes_coords, &coord_in_direction, &coord_in_past_direction);
        }
    } else {
        move_player(player_coords, &coord_in_direction);
        return;
    }
}

fn is_wall(coords: &Coord, map: &[[u8; 8]; 9]) -> bool {
    return map[coords.y as usize][coords.x as usize] == 1;
}

fn is_player(coord: &Coord, player_coords: &Coord) -> bool {
    if coord.x == player_coords.x && coord.y == player_coords.y {
        return true;
    } else {
        return false;
    }
}

fn is_box(coord: &Coord, boxes_coords: &[Coord; 7]) -> bool {
    for box_coords in boxes_coords.iter() {
        if coord.x == box_coords.x && coord.y == box_coords.y {
            return true;
        }
    }
    return false;
}

fn is_target(coord: &Coord, boxes_targets: &[Coord; 7]) -> bool {
    for box_target in boxes_targets.iter() {
        if coord.x == box_target.x && coord.y == box_target.y {
            return true;
        }
    }
    return false;
}

fn move_player(player_coords: &mut Coord, new_cord: &Coord) {
    player_coords.x = new_cord.x;
    player_coords.y = new_cord.y;
}

fn move_box(boxes_coords: &mut [Coord; 7], box_coords: &Coord, box_new_coords: &Coord) {
    for boxx in boxes_coords.iter_mut() {
        if boxx.x == box_coords.x && boxx.y == box_coords.y {
            boxx.x = box_new_coords.x;
            boxx.y = box_new_coords.y;
            return;
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    println!("[SERVER] - Listening for connections on port 7878");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream);
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
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

    let map = MAP_01;

    let mut player_coords: Coord = Coord { x: 2, y: 2 };

    let mut boxes_coords: [Coord; 7] = [
        Coord { x: 3, y: 2 },
        Coord { x: 4, y: 3 },
        Coord { x: 4, y: 4 },
        Coord { x: 1, y: 6 },
        Coord { x: 3, y: 6 },
        Coord { x: 4, y: 6 },
        Coord { x: 5, y: 6 },
    ];

    let boxes_targets: [Coord; 7] = [
        Coord { x: 1, y: 2 },
        Coord { x: 5, y: 3 },
        Coord { x: 1, y: 4 },
        Coord { x: 4, y: 5 },
        Coord { x: 3, y: 6 },
        Coord { x: 6, y: 6 },
        Coord { x: 4, y: 7 },
    ];

    print_map(&map, &boxes_coords, &boxes_targets, &player_coords);

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
                process_move(&map, &mut boxes_coords, &mut player_coords, movement);
                print_map(&map, &boxes_coords, &boxes_targets, &player_coords);
                if victory(&boxes_coords, &boxes_targets) {
                    let response = String::from(
                        "Felicitaciones! Has vencido el juego. Gracias por jugar.\n",
                    );
                    stream_clone.write_all(response.as_bytes());
                    break;
                }
                let response = String::from("OK\n");
                stream_clone.write_all(response.as_bytes());
            }
        }
    }
    println!("READY TO JOIN");
}

/*fn process_request(&mut self, client_request: String) -> String {
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
}*/
