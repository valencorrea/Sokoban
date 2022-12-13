use crate::api::constants::{
    AIR_STR, BOX_ON_TARGET_STR, BOX_ON_TARGET_U8, BOX_STR, BOX_U8, PLAYER_STR, PLAYER_U8,
    TARGET_STR, TARGET_U8, WALL_STR, WALL_U8,
};

pub fn get_object(map_object: u8) -> &'static str {
    if map_object == WALL_U8 {
        WALL_STR
    } else if map_object == PLAYER_U8 {
        PLAYER_STR
    } else if map_object == BOX_U8 {
        BOX_STR
    } else if map_object == TARGET_U8 {
        TARGET_STR
    } else if map_object == BOX_ON_TARGET_U8 {
        BOX_ON_TARGET_STR
    } else {
        AIR_STR
    }
}
