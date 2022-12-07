use std::io;

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

fn sokoban2() {
    println!("Bienvenido al Sokoban!");
    print_instructions();

    // construir mapa
    let map: [[u8; 8]; 9] = [
        [0, 0, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 0, 0, 0, 1, 0],
        [1, 0, 0, 0, 0, 0, 1, 0],
        [1, 1, 1, 0, 0, 0, 1, 0],
        [1, 0, 1, 1, 0, 0, 1, 0],
        [1, 0, 1, 0, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

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
        let input: String = get_user_input();
        if input == "Q" {
            println!("Gracias por jugar! Nos vemos!");
            break;
        }

        let movement: Move = process_input(&input);
        process_move(&map, &mut boxes_coords, &mut player_coords, movement);
        print_map(&map, &boxes_coords, &boxes_targets, &player_coords);

        if victory(&boxes_coords, &boxes_targets) {
            println!("Felicitaciones!\nHas vencido el juego. Gracias por jugar.");
            break;
        }
    }
}

fn print_instructions() {
    println!();
    println!("El objetivo del juego es empujar cada caja a un objetivo. ¡Suerte!");
    println!("Usaremos las siguientes notaciones:");
    println!("\t# - PARED");
    println!("\tP - PERSONAJE");
    println!("\t= - CAJA");
    println!("\t+ - OBJETIVO");
    println!("\t* - CAJA EN OBJETIVO");
    println!();
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

fn process_input(input: &String) -> Move {
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
