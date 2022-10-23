pub fn user_welcome() {
    println!("\nBienvenidos al Sokoban!\n");
    show_instructions();
}

fn show_instructions(){
    println!("El objetivo del juego es empujar cada caja a un objetivo. ¡Suerte!\n");
    println!("Usaremos las siguientes notaciones:");
    println!("\t# - PARED");
    println!("\tP - PERSONAJE");
    println!("\t= - CAJA");
    println!("\t+ - OBJETIVO");
    println!("\t* - CAJA EN OBJETIVO\n");
}