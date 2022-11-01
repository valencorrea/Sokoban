pub fn user_welcome() {
    println!("\nBienvenidos al Sokoban!\n");
    show_instructions();
}

pub fn user_goodbye() {
    println!("Gracias por jugar! Nos vemos!");
    //show()_results();
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
