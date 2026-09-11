use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

const BACKENDS: [&str; 2] = ["127.0.0.1:9001", "127.0.0.1:9002"];
static NEXT_BACKEND: AtomicUsize = AtomicUsize::new(0);

fn pick_backend() -> &'static str {
    let index = NEXT_BACKEND.fetch_add(1, Ordering::SeqCst) % BACKENDS.len();
    BACKENDS[index]
}

fn handle_client(mut client: TcpStream) {
    let backend_addr = pick_backend();
    println!("Reenviando a: {}", backend_addr);

    let mut backend = match TcpStream::connect(backend_addr) {
        Ok(b) => b,
        Err(e) => {
            println!("Backend {} no disponible: {}", backend_addr, e);
            let body = "<h1>502 Bad Gateway</h1>";
            let response = format!(
                "HTTP/1.1 502 Bad Gateway\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = client.write_all(response.as_bytes());
            return; // sale de esta funcion, pero el proceso sigue vivo
        }
    };

    let mut request = [0u8; 1024];
    let n = match client.read(&mut request) {
        Ok(n) => n,
        Err(_) => return,
    };

    if backend.write_all(&request[..n]).is_err() {
        return;
    }

    let mut response = Vec::new();
    if backend.read_to_end(&mut response).is_err() {
        return;
    }

    let _ = client.write_all(&response);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Proxy escuchando en http://127.0.0.1:8000 → {:?}", BACKENDS);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error: {}", e),
        }
    }
}