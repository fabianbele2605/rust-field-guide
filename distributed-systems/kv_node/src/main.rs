use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpStream, TcpListener};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::env;

type Store = Arc<Mutex<HashMap<String, String>>>;

fn replicate(peer_addr: &str, command: &str) {
    if let Ok(mut stream) = TcpStream::connect(peer_addr) {
        let msg = format!("REPLICATE {}\n", command);
        let _ = stream.write_all(msg.as_bytes());
    } else {
        println!("No se pudo replicar a {} (¿está caído?)", peer_addr);
    }
}

fn handler_client(stream: TcpStream, store: Store, peer_addr: Option<String>) {
    let mut write = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim();

        // ¿Es un comando de replicación entrante desde el peer?
        let (is_replicated, real_command) = if let Some(rest) = line.strip_prefix("REPLICATE ") {
            (true, rest.to_string())
        } else {
            (false, line.to_string())
        };
        
        let parts: Vec<&str> = real_command.splitn(3, ' ').collect();
        let response = match parts.as_slice() {
            ["SET", key, value] => {
                let mut s = store.lock().unwrap();
                s.insert(key.to_string(), value.to_string());
                drop(s);

                // Si NO vino de replicación, reenviarla al peer
                if !is_replicated {
                    if let Some(peer) = &peer_addr {
                        replicate(peer, &real_command);
                    }
                }
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
                drop(s);
                if !is_replicated {
                    if let Some(peer) = &peer_addr {
                        replicate(peer, &real_command);
                    }
                }
                "OK\n".to_string()
            }
            _ => "ERROR: comando desconocido\n".to_string(),
        };

        if is_replicated {
            continue;  // no responder nada al peer, solo aplicar el cambio
        }
        if write.write_all(response.as_bytes()).is_err() {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: lv_node <puerto> [peer_host:puerto]");
        return;
    }
    let port = &args[1];
    let peer_addr: Option<String> = args.get(2).cloned();

    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Node escuchando en {} (peer: {:?})", addr, peer_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                let peer_addr = peer_addr.clone();
                thread::spawn(move || handler_client(stream, store, peer_addr));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}