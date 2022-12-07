use crate::api::sokoban_service::Sokoban;
use crate::api::utils::{
    AIR_STR, BOX_STR, BOX_U8, ENTER_STR, PLAYER_STR, PLAYER_U8, TARGET_STR, TARGET_U8, WALL_STR,
    WALL_U8,
};

pub fn show_welcome() {
    println!("\nBienvenidos al Sokoban!\n");
    println!("El objetivo del juego es empujar cada caja a un objetivo. ¡Suerte!\n");
    show_commands();
}

pub fn show_goodbye() {
    println!("Gracias por jugar! Nos vemos!");
    //show()_results();
}

// todo agregar h de ayuda y que muestre de nuevo los comandos
pub fn show_commands() {
    println!("Comandos validos:");
    println!("\tA - LEFT");
    println!("\tW - UP");
    println!("\tD - RIGHT");
    println!("\tS - DOWN");
}

pub fn show_victory() {
    println!("\nFelicitaciones!\nHas vencido el juego. Gracias por jugar.\n");
}

pub fn ask_for_command() {
    // todo: si no te los acordas preisona h
    println!("Escribe tu movimiento o Q para cerrar el juego.")
}

pub fn get_object(map_object: u8) -> &'static str {
    return if map_object == WALL_U8 {
        WALL_STR
    } else if map_object == PLAYER_U8 {
        PLAYER_STR
    } else if map_object == BOX_U8 {
        BOX_STR
    } else if map_object == TARGET_U8 {
        TARGET_STR
    } else {
        AIR_STR
    };
}

//todo move no debe estar actualizando el struct sokoban
pub fn print_map(sokoban: &mut Sokoban) {
    let mut str_map = String::new();
    for row in 0..sokoban.rows {
        for column in 0..sokoban.columns {
            let object = get_object(sokoban.map[row][column]);
            str_map.push(object.parse().unwrap());
        }
        str_map.push_str(ENTER_STR);
    }
    println!("{}", str_map);
}
