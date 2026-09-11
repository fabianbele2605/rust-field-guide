use std::net::TcpStream;
use std::io::{Read, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: tcp_client <host:puerto>");
        return;
    }

    let address = &args[1];

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Conectado a {}!", address);

            let host = address.split(':').next().unwrap_or(address);
            let request = format!(
                "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                host
            );
            stream.write_all(request.as_bytes()).unwrap();
            println!("Peticion enviada.");

            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            println!("--- Respuesta ---");
            println!("{}", response);
        }
        Err(e) => println!("Error al conectar: {}", e),
    }
}