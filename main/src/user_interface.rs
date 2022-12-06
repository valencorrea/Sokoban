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
pub fn show_commands(){
    println!("Comandos validos:");
    println!("\t# - PARED");
    println!("\tP - PERSONAJE");
    println!("\t= - CAJA");
    println!("\t+ - OBJETIVO");
    println!("\t* - CAJA EN OBJETIVO\n");
}

pub fn show_victory(){
    println!("\nFelicitaciones!\nHas vencido el juego. Gracias por jugar.\n");
}

pub fn ask_for_command(){
    // todo: si no te los acordas preisona h
    println!("Escribe tu movimiento o Q para cerrar el juego.")
}

pub fn mostrar_mapa(mapa: &String) {
    println!("Mostrando mapa actual...\n\n{}\n", mapa);
}