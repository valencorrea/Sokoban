// TODO Traer Sokoban
// TODO traer get_deltas
// TODO Traer get_next_coord
// TODO Traer is_object

// TODO OK
#[derive(Debug)]
pub struct Server {
    sokoban: Mutex<Sokoban>,
}

impl Server {
    // TODO OK
    pub fn create_from_map(sokoban: Sokoban) -> Server {
        Server {
            sokoban: Mutex::new(sokoban),
        }
    }

    // TODO OK
    pub fn run(self) -> std::io::Result<()> {
        let s = Arc::new(self);

        let listener = TcpListener::bind("0.0.0.0:7878")?;

        println!("[SERVER] - Listening for connections on port 7878");

        for stream in listener.incoming() {
            let stream = stream?;
            let ss = s.clone();
            thread::spawn(move || {
                Server::handle_client(ss, stream);
            });
        }
        Ok(())
    }

    fn handle_client(server: Arc<Server>, stream: TcpStream) {
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

                    {
                        let s = server.clone();

                        let sok = s.sokoban.lock().unwrap();

                        sok.process_move(movement);
                        sok.print();

                        if sok.victory() {
                            let response = String::from(
                                "Felicitaciones! Has vencido el juego. Gracias por jugar.\n",
                            );
                            stream_clone.write_all(response.as_bytes());
                            break;
                        }
                    }

                    let response = String::from("OK\n");
                    stream_clone.write_all(response.as_bytes());
                }
            }
        }

        println!("READY TO JOIN");
    }    
}

// TODO OK
pub fn process_input(input: &str) -> Result<Move, SokobanError> {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else if input == RIGHT {
        Move::Right
    } else {
        SokobanError
    }
}