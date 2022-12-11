use crate::api::sokoban_service::{Move, Sokoban};
use crate::api::constants::{AIR_U8, BOX_ON_TARGET_U8, BOX_U8, DOWN, LEFT, PLAYER_U8, TARGET_U8, UP, WALL_U8};
use crate::api::coord_service::{Coord, equals_to, get_deltas, get_next_coord, update_coords};
use crate::api::map_service::refresh_map;
use crate::api::ux::print_map;

// TODO MANDAR AL SERVER
pub fn process_input(input: &str) -> Move {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else {
        Move::Right
    };
}

