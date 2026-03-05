use rusty_kv::{Command, KvStore, parse_command};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("Server Listening");

    let mut store = KvStore::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connection");
                handle_client(stream, &mut store);
            }
            Err(e) => {
                eprintln!("Failed to establish conncetion {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, store: &mut KvStore) {
    let s = stream.try_clone().unwrap();

    let mut reader = BufReader::new(&s);
    let mut line = String::new();

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line).unwrap();

        if bytes_read == 0 {
            println!("Client Disconnected");
            break;
        }

        let response = match parse_command(&line) {
            Ok(command) => match command {
                Command::Set { key, value } => {
                    store.set(key, value);
                    "OK\n".to_string()
                }
                Command::Get { key } => match store.get(key) {
                    Some(val) => format!("{}\n", val),
                    None => "(nil)\n".to_string(),
                },
                Command::Remove { key } => match store.remove(key) {
                    Some(_) => "OK\n".to_string(),
                    None => "(nil)\n".to_string(),
                },
            },
            Err(e) => format!("ERROR: {:?}\n", e),
        };

        // 2. Write response back to client
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to stream: {}", e);
            break;
        }
    }
}
