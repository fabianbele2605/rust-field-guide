use std::collections::{HashMap, HashSet};
use std::time::{Instant, Duration};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct ConnectionAttempt {
    port: u16,
    when: Instant,
}

struct IdsAnalyzer {
    // por OP: lista intentos recientes
    history: HashMap<String, Vec<ConnectionAttempt>>,
    window: Duration,       // ventana de tiempo a considerar
    port_threshold: usize,  // cuántos puertos distintos disparan alerta
}

impl IdsAnalyzer {
    fn new(window: Duration, port_threshold: usize) -> Self {
        IdsAnalyzer {
            history: HashMap::new(),
            window,
            port_threshold,
        }
    }

    fn record(&mut self, ip: &str, port: u16) {
        let entry = self.history.entry(ip.to_string()).or_insert_with(Vec::new);
        entry.push(ConnectionAttempt { port, when: Instant::now() });
    }

    fn check_port_scan(&self, ip: &str) -> bool {
        let now = Instant::now();
        if let Some(attempts) = self.history.get(ip) {
            let recent_ports: HashSet<u16> = attempts.iter()
                .filter(|a| now.duration_since(a.when) <= self.window)
                .map(|a| a.port)
                .collect();
            recent_ports.len() >= self.port_threshold
        } else {
            false
        }
    }
}

fn main() {
    let ids = Arc::new(Mutex::new(IdsAnalyzer::new(Duration::from_secs(5), 5)));

    let ports_to_watch = [9990, 9991, 9992, 9993, 9994, 9995];
    let mut handles = Vec::new();

    for &port in &ports_to_watch {
        let ids = Arc::clone(&ids);
        let handle = thread::spawn(move || {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
            println!("Honeypot escuchando en 127.0.0.1:{}", port);

            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    let peer = stream.peer_addr().map(|a| a.ip().to_string()).unwrap_or_default();
                    let mut analyzer = ids.lock().unwrap();
                    analyzer.record(&peer, port);

                    if analyzer.check_port_scan(&peer) {
                        println!(" ALERTA: posible port scan desde {} (puerto {})", peer, port);
                    } else {
                        println!("Conexion registrada desde {} (puerto {})", peer, port);
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}