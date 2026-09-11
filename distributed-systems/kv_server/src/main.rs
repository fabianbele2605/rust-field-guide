use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpStream, TcpListener};
use std::io::{BufRead, BufReader, Write};
use std::thread;

type Store = Arc<Mutex<HashMap<String, String>>>;

fn handler_client(stream: TcpStream, store: Store) {
    let mut write = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        
        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
        let response = match parts.as_slice() {
            ["SET", key, value] => {
                let mut s = store.lock().unwrap();
                s.insert(key.to_string(), value.to_string());
                "OK\n".to_string()
            }
            ["GET", key] => {
                let s = store.lock().unwrap();
                match s.get(*key) {
                    Some(v) => format!("{}\n", v),
                    None => "NOT FOUND\n".to_string(),
                }
            }
            ["DEL", key] => {
                let mut s = store.lock().unwrap();
                s.remove(*key);
                "OK\n.to_string()".to_string()
            }
            _ => "ERROR: comando desconocido\n".to_string(),
        };

        if write.write_all(response.as_bytes()).is_err() {
            break;
        }
    }
}

fn main() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    println!("KV Server escuchando en 127.0.0.1:6380");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || handler_client(stream, store));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}