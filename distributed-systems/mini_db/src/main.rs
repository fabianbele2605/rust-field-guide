use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpStream, TcpListener};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::env;
use std::fs::OpenOptions;

#[derive(Clone, Copy, PartialEq)]
enum Role {
    Leader,
    Follower,
}

type Store = Arc<Mutex<HashMap<String, String>>>;

fn replicate(peer_addr: &str, command: &str) {
    if let Ok(mut stream) = TcpStream::connect(peer_addr) {
        let msg = format!("REPLICATE {}\n", command);
        let _ = stream.write_all(msg.as_bytes());
    } else {
        println!("No se pudo replicar a {} (¿está caído?)", peer_addr);
    }
}

fn append_to_wal(wal_path: &str, command: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(wal_path) {
        let _ = writeln!(file, "{}", command);
    }
}

fn recover_from_wal(wal_path: &str, store: &Store) {
    if let Ok(file) = std::fs::File::open(wal_path) {
        let reader = BufReader::new(file);
        let mut s = store.lock().unwrap();
        for line in reader.lines() {
            if let Ok(command) = line {
                let parts: Vec<&str> = command.splitn(3, ' ').collect();
                match parts.as_slice() {
                    ["SET", key, value] => {
                        s.insert(key.to_string(), value.to_string());
                    }
                    ["DEL", key] => {
                        s.remove(*key);
                    }
                    _ => {}
                }
            }
        }
        println!("Recuperados {} registros desde {}", s.len(), wal_path);
    } else {
        println!("Sin WAL previo en {} (arranque limpio)", wal_path);
    }
}

fn handler_client(stream: TcpStream, store: Store, peer_addr: Option<String>, role: Role, wal_path: String) {
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

        // Un Follower rechaza escrituras directas de clientes (no replicas)
        if role == Role::Follower && !is_replicated {
            let parts_check: Vec<&str> = real_command.splitn(2, ' ').collect();
            if let Some(cmd) = parts_check.first() {
                if *cmd == "SET" || *cmd == "DEL" {
                    let _ = write.write_all(b"ERROR: soy follower, escribe en el Leader\n");
                    continue;
                }
            }
        }
        
        let parts: Vec<&str> = real_command.splitn(3, ' ').collect();
        let response = match parts.as_slice() {
            ["SET", key, value] => {
                let mut s = store.lock().unwrap();
                s.insert(key.to_string(), value.to_string());
                drop(s);
                append_to_wal(&wal_path, &real_command);
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
                append_to_wal(&wal_path, &real_command);
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
        println!("Uso: mini_db <puerto> <leader|follower> [peer_host:puerto]");
        return;
    }
    let port = &args[1];
    let role = match args[2].as_str() {
        "leader" => Role::Leader,
        "follower" => Role::Follower,
        _ => {
            println!("Rol inválido, usa 'leader' o 'follower'");
            return;
        }
    };
    let peer_addr: Option<String> = args.get(3).cloned();

    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).unwrap();
    let wal_path_main = format!("wal_{}.log", port);
    recover_from_wal(&wal_path_main, &store);
    println!("Node ({:?}) escuchando en {} (peer: {:?})",
        if role == Role::Leader { "Leader" } else { "Follower" }, addr, peer_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                let peer_addr = peer_addr.clone();
                let wal_path = format!("wal_{}.log", port);
                thread::spawn(move || handler_client(stream, store, peer_addr, role, wal_path));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}