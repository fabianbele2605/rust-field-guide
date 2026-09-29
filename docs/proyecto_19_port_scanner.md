# 📚 Proyecto 19: Port Scanner — Documentación Educativa

**Campo:** 🔐 Cybersecurity | **Nivel:** 🟢 Básico | **Estado:** ✅ COMPLETADO
**Carpeta:** `cybersecurity/port_scanner/`

> ⚠️ **Uso responsable:** esta herramienta se probó ÚNICAMENTE contra `127.0.0.1` (la propia máquina). Escanear puertos de hosts de terceros sin autorización explícita puede ser ilegal — úsala solo contra tus propios sistemas o entornos de laboratorio/CTF autorizados.

---

## 🎯 Resumen: ¿Qué Construimos?

Un escáner de puertos concurrente: dado un host, revisa un rango de puertos y reporta cuáles están abiertos, con el nombre del servicio conocido si aplica.

```
port_scanner 127.0.0.1
  ↓
10 threads escanean en paralelo, cada uno un rango de 100 puertos
  ↓
127.0.0.1:631 -> OPEN (CUPS (impresión))
Escaneo completo.
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `TcpStream::connect_timeout` vs `connect`

```rust
let socket_addr: SocketAddr = addr.parse().unwrap();
TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
```

**Diferencia clave con el Proyecto 7:** `connect` (usado antes) puede quedarse esperando indefinidamente si el host no responde nada (puerto filtrado por firewall, por ejemplo). `connect_timeout` recibe un `Duration` máximo — pasado ese tiempo, falla con error en vez de colgarse para siempre. Fundamental para un scanner: sin timeout, un solo puerto silencioso podría trabar TODO el programa.

**Por qué necesita `SocketAddr` en vez de un string:** `connect_timeout` requiere la dirección ya "parseada" en su forma estructurada (IP + puerto como tipos, no texto) — de ahí el `.parse::<SocketAddr>()`.

### 2. Dividir trabajo entre threads: chunks de un rango

```rust
let chunk_size = total_ports / num_threads;
for i in 0..num_threads {
    let start = i * chunk_size + 1;
    let end = if i == num_threads - 1 { total_ports } else { (i + 1) * chunk_size };
    ...
}
```

Patrón de paralelización simple: partir un rango grande en N pedazos iguales, y asignar un pedazo a cada thread. El último pedazo absorbe cualquier resto de la división entera (`i == num_threads - 1`), para no perder puertos por redondeo.

### 3. Recolectar resultados de múltiples threads con `join()`

```rust
let handle = thread::spawn(move || {
    let mut open_ports = Vec::new();
    // ... escanea su rango, llena open_ports
    open_ports  // el thread "retorna" este Vec
});
handles.push(handle);

// más adelante:
for handle in handles {
    let open_ports = handle.join().unwrap();  // recupera lo que retornó ese thread
    all_open.extend(open_ports);
}
```

A diferencia de proyectos anteriores (donde los threads solo escribían a un estado compartido con `Mutex`), aquí cada thread simplemente **retorna su propio resultado** al terminar. `handle.join()` bloquea hasta que ESE thread específico termine, y devuelve lo que retornó su closure — un patrón más simple que compartir estado cuando cada trabajador puede resolver su parte de forma completamente independiente.

---

## 🖥️ Conceptos de Redes / Seguridad

### 1. Qué significa realmente "puerto abierto"

Un puerto está "abierto" cuando hay un proceso escuchando ahí, dispuesto a aceptar conexiones TCP (`TcpListener::bind`, como en los Proyectos 8-12). Un `connect()` exitoso confirma exactamente eso — no dice nada sobre QUÉ corre ahí (eso requeriría inspeccionar la respuesta, "banner grabbing", fuera de este proyecto).

### 2. Por qué escanear en paralelo importa tanto aquí

Escanear 1000 puertos secuencialmente, con 100ms de timeout cada uno, tardaría en el PEOR caso 100 segundos (si todos estuvieran filtrados/silenciosos). Con 10 threads en paralelo, ese peor caso baja a ~10 segundos — una mejora lineal directa con el número de threads, porque cada intento de conexión es independiente entre sí (no hay dependencia de datos entre puertos distintos).

### 3. Ética y legalidad del escaneo de puertos

Escanear tus propios sistemas (o sistemas para los que tienes autorización explícita, como en un CTF o laboratorio de pentesting) es una práctica de seguridad completamente legítima. Escanear sistemas de terceros SIN autorización puede constituir un delito informático en muchas jurisdicciones (acceso no autorizado a sistemas, aunque sea solo "tocar la puerta"). La regla práctica: si no eres dueño del sistema y no tienes permiso por escrito, no lo escanees.

---

## 📝 Código Final

```rust
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
        21 => "FTP", 22 => "SSH", 23 => "Telnet", 25 => "SMTP", 53 => "DNS",
        80 => "HTTP", 443 => "HTTPS", 631 => "CUPS (impresión)",
        3306 => "MySQL", 5432 => "PostgreSQL", 6379 => "Redis", 8080 => "HTTP-alt",
        _ => "desconocido",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let host = if args.len() > 1 { args[1].clone() } else { "127.0.0.1".to_string() };
    let host = host.as_str();

    let timeout = Duration::from_millis(100);
    let total_ports = 1000u16;
    let num_threads = 10;
    let chunk_size = total_ports / num_threads;

    let mut handles = Vec::new();
    for i in 0..num_threads {
        let start = i * chunk_size + 1;
        let end = if i == num_threads - 1 { total_ports } else { (i + 1) * chunk_size };
        let host = host.to_string();
        handles.push(thread::spawn(move || {
            let mut open_ports = Vec::new();
            for port in start..=end {
                if scan_port(&host, port, timeout) {
                    open_ports.push(port);
                }
            }
            open_ports
        }));
    }

    let mut all_open: Vec<u16> = Vec::new();
    for handle in handles {
        all_open.extend(handle.join().unwrap());
    }
    all_open.sort();

    for port in &all_open {
        println!("{}:{} -> OPEN ({})", host, port, service_name(*port));
    }
    println!("Escaneo completo.");
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `connect_timeout` | Conexión con límite de tiempo | Evita que un puerto silencioso cuelgue todo el scanner |
| Chunking de un rango | Dividir el trabajo en partes iguales | Paralelizar sin coordinación compleja entre threads |
| `handle.join()` | Recuperar el valor retornado por un thread | Cada thread resuelve su parte de forma independiente |
| `service_name` | Tabla puerto → nombre conocido | Contexto útil sobre qué podría correr ahí |

---

## 🚀 Próximos Pasos

- **Proyecto 20:** Packet Analyzer
- Banner grabbing (leer la respuesta inicial de un servicio para identificarlo con más certeza)
- Escaneo UDP (más complejo: sin respuesta no siempre significa "cerrado")
- Progreso en tiempo real (mostrar puertos abiertos a medida que se encuentran, no solo al final)

---

**🎉 Proyecto 19: Completado. Primer proyecto del campo Cybersecurity.**
