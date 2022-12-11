use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn is_valid_input(input: &String) -> bool {
    return input == "W" || input == "A" || input == "S" || input == "D" || input == "Q";
}

pub fn run() -> std::io::Result<()> {
    let stream: TcpStream = TcpStream::connect("127.0.0.1:7878")?;

    let mut stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("[SERVER-CONNECTION] Unsuccesful creation of TCP connection");
            return Ok(());
        }
    };

    let buf_reader = BufReader::new(stream);
    let mut lines = buf_reader.lines();

    let mut input: String = String::new();
    loop {
        input.clear();
        println!("Escribe tu movimiento (WASD) o QUIT para cerrar el juego:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        stream_clone.write_all(input.as_bytes())?;
        if let Some(l) = lines.next() {
            let line = match l {
                Ok(p) => p,
                Err(_) => {
                    println!("[CLIENT] - Connection closed");
                    return Ok(())
                }
            };
            println!("[SERVER]: {} ", line);
            if line.contains("CLOSING") {
                println!("[CLIENT] - Connection closed"); // TODO refactor
                return Ok(())
            }
        }
    }

}
