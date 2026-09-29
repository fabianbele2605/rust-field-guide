use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::thread;
use std::env;

fn scan_port(host: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{}:{}", host, port);
    match addr.parse::<SocketAddr>() {
        Ok(socket_addr) => TcpStream::connect_timeout(&socket_addr, timeout).is_ok(),
        Err(_) => false,
    }
}

fn service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        443 => "HTTPS",
        631 => "CUPS (impresión)",
        3306 => "MySQL",
        5432 => "PostgreSQL",
        6379 => "Redis",
        8080 => "HTTP-alt",
        _ => "desconocido",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let host = if args.len() > 1 { args[1].clone() } else { "127.0.0.1".to_string() };
    let host = host.as_str();
    
    let timeout = Duration::from_millis(100);
    let start_port = 9980u16;
    let end_port = 10000u16;
    let num_threads = 10u16;
    let chunk_size = (end_port - start_port) / num_threads;

    println!("Escaneando {} (puertos {}-{}) con {} threads...", host, start_port, end_port, num_threads);

    let mut handles = Vec::new();

    for i in 0..num_threads {
        let start = start_port + i * chunk_size;
        let end = if i == num_threads - 1 { end_port } else { start_port + (i + 1) * chunk_size };
        let host = host.to_string();

        let handle = thread::spawn(move || {
            let mut open_ports = Vec::new();
            for port in start..=end {
                if scan_port(&host, port, timeout) {
                    open_ports.push(port);
                }
            }
            open_ports
        });

        handles.push(handle);
    }

    let mut all_open: Vec<u16> = Vec::new();
    for handle in handles {
        let open_ports = handle.join().unwrap();
        all_open.extend(open_ports);
    }

    all_open.sort();
    for port in &all_open {
        println!("{}:{} -> OPEN ({})", host, port, service_name(*port));
    }
    println!("Escaneo completo.")
}