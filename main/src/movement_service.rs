pub use crate::command_service::{DOWN, LEFT, UP};
use crate::sokoban_service::{Coord, Move, Sokoban, SokobanError};

pub fn move_player(input: &String) -> Move {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else {
        Move::Right
    }
}
/*
pub fn process_move(sokoban: &Sokoban, movement: Move){
    let mut delta_x: i8 = 0;
    let mut delta_y: i8 = 0;

    match movement {
        Move::Up => delta_y = -1,
        Move::Left => delta_x = -1,
        Move::Down => delta_y = 1,
        Move::Right => delta_x = 1,
    }

    let coord_in_direction: Coord = Coord {
        x: (sokoban.user_coords.x as i8 + delta_x) as u8,
        y: (sokoban.user_coords.y as i8 + delta_y) as u8,
    };

    let coord_in_past_direction: Coord = Coord {
        x: (sokoban.user_coords.x as i8 + delta_x * 2) as u8,
        y: (sokoban.user_coords.y as i8 + delta_y * 2) as u8,
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

}*/