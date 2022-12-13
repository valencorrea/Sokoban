use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use super::{
    constants::{DOWN, LEFT, UP},
    sokoban::Sokoban,
    utils::Move,
};

#[derive(Debug)]
pub struct Server {
    sokoban: Mutex<Sokoban>,
    thr_clients: Mutex<Vec<JoinHandle<()>>>,
    tcp_clients: Mutex<Vec<TcpStream>>,
    responses: (Mutex<VecDeque<String>>, Condvar),
}

impl Server {
    pub fn create_from_map(sokoban: Sokoban) -> Server {
        Server {
            sokoban: Mutex::new(sokoban),
            thr_clients: Mutex::new(Vec::new()),
            tcp_clients: Mutex::new(Vec::new()),
            responses: (Mutex::new(VecDeque::<String>::new()), Condvar::new()),
        }
    }

    pub fn run(self) -> std::io::Result<()> {
        let s = Arc::new(self);

        let ss = s.clone();

        let listener = TcpListener::bind("0.0.0.0:7878")?;

        let closing = Arc::new(Mutex::new(false));

        listener.set_nonblocking(true).unwrap();

        println!("[SERVER] - Listening for connections on port 7878");

        let cc = closing.clone();
        let cc2 = closing.clone();

        let responses_thread = thread::spawn(move || loop {
            let (responses, cv) = &ss.responses;
            let mut responses = responses.lock().unwrap();
            while responses.is_empty() {
                responses = cv.wait(responses).unwrap();
            }

            let response = responses.pop_front().unwrap();
            for tcp in ss.tcp_clients.lock().unwrap().iter() {
                let mut stream_clone = tcp.clone();

                stream_clone.write_all(response.as_bytes()).unwrap();
            }

            let mut ccc = cc.lock().unwrap();

            if response.contains("VICTORY") || response.contains("CLOSING") {
                *ccc = true;

                let mut to_close = ss.tcp_clients.lock().unwrap();

                while let Some(pop) = to_close.pop() {
                    pop.shutdown(std::net::Shutdown::Both).unwrap();
                }

                println!("[SERVER] - Finished closing TCPs");

                let mut to_join = ss.thr_clients.lock().unwrap();

                while let Some(pop) = to_join.pop() {
                    pop.join().unwrap();
                }

                println!("[SERVER] - Finished joining threads");
            }

            if *ccc {
                break;
            }
        });

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(v) => v,
                Err(e) => {
                    if *cc2.lock().unwrap() {
                        break;
                    }
                    continue;
                }
            };

            if *closing.lock().unwrap() {
                break;
            }

            let stream_clone = stream.try_clone().unwrap();
            let ss = s.clone();
            let t = thread::spawn(move || {
                Server::handle_client(ss, stream);
            });

            {
                let mut c_thr = s.thr_clients.lock().unwrap();

                let mut c_tcp = s.tcp_clients.lock().unwrap();

                c_thr.push(t);

                c_tcp.push(stream_clone);
            }

            thread::sleep(Duration::from_secs(1));
        }

        responses_thread.join().unwrap();

        Ok(())
    }

    fn handle_client(server: Arc<Server>, stream: TcpStream) {
        {
            let mut map = server.sokoban.lock().unwrap().to_str();
            map.push('\n');

            let mut st = stream.try_clone().unwrap();

            st.write_all(map.as_bytes()).unwrap();
        }

        let client_addr = match stream.peer_addr() {
            Ok(sa) => sa.to_string(),
            Err(_) => "Unknown".to_owned(),
        };

        println!("[SERVER] - New Connection: {}", client_addr);

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
                    let response = String::from("CLOSING\n");
                    {
                        let s = server.clone();

                        let (q, cv) = &s.responses;

                        let mut q = q.lock().unwrap();

                        q.push_back(response);

                        cv.notify_one();
                    }
                    break;
                } else if request[0] == "MOVE" {
                    let input = request[1];
                    let movement: Move = process_input(&input);
                    {
                        let s = server.clone();

                        let mut sok = s.sokoban.lock().unwrap();

                        let (q, cv) = &s.responses;

                        let mut q = q.lock().unwrap();

                        let mut response = sok.process_move(movement);
                        response.push('\n');

                        if sok.victory() {
                            let response = String::from("VICTORY");
                            q.push_back(response);
                            cv.notify_one();
                            break;
                        }

                        q.push_back(response);

                        cv.notify_one();
                    }
                }
            } else {
                break;
            }
        }

        println!("[SERVER]: READY TO JOIN");
    }
}

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
