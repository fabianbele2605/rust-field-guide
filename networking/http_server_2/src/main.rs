use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    println!("Petición: {}", request_line.trim());

    let (status, body) = if request_line.starts_with("GET / ") {
        ("200 OK", "<h1>Página principal Backend 2</h1>")
    } else if request_line.starts_with("GET /users ") {
        ("200 OK", "<h1>Lista de usuarios<h1>")
    } else {
        ("404 NOT FOUND", "<h1>404 - No encontrado<h1>")
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9002").unwrap();
    println!("Servidor escuchando en http://127.0.0.1:9002");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}