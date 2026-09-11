# 📚 Proyecto 8: HTTP Server — Documentación Educativa

**Campo:** 🌐 Networking | **Nivel:** 🟡 Intermedio | **Estado:** ✅ COMPLETADO
**Carpeta:** `networking/http_server/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un servidor HTTP desde cero (sin frameworks) que:
1. Escucha conexiones en un puerto
2. Lee qué ruta pidió el cliente
3. Responde distinto según la ruta (routing)
4. Atiende múltiples clientes en paralelo (threads)

```
Cliente → TcpListener::bind → accept() → leer petición → routing → responder
                                              ↑
                                    (todo dentro de un thread por cliente)
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `TcpListener` — el espejo de `TcpStream`

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
```

En el Proyecto 7 fuiste **cliente** (`TcpStream::connect`). Aquí eres **servidor**: abres un puerto y esperas que otros se conecten a ti.

### 2. `listener.incoming()` — iterador infinito de conexiones

```rust
for stream in listener.incoming() {
    match stream { ... }
}
```

A diferencia de `accept()` (una sola conexión), `.incoming()` es un iterador que nunca termina — cada vuelta del loop bloquea hasta que llega un cliente nuevo.

### 3. `BufReader` + `.lines()` para leer la petición

```rust
let mut reader = BufReader::new(&stream);
let mut request_line = String::new();
reader.read_line(&mut request_line).unwrap();
```

`BufReader` envuelve el stream y agrega un buffer interno, permitiendo leer línea por línea eficientemente (en vez de leer byte a byte).

### 4. `thread::spawn` — concurrencia simple

```rust
thread::spawn(|| handle_client(stream));
```

**¿Qué hace?**
- Crea un hilo del sistema operativo
- El closure `|| handle_client(stream)` se ejecuta en ESE hilo, no en el principal
- El loop principal sigue aceptando nuevas conexiones inmediatamente, sin esperar a que termine la anterior

**¿Por qué es necesario?**
Sin threads, si un cliente tarda 5 segundos en algo, TODOS los demás clientes esperan esos 5 segundos antes de ser atendidos (procesamiento serial). Con un thread por cliente, se atienden en paralelo.

---

## 🖥️ Conceptos de Networking / Servidores

### 1. Anatomía de una respuesta HTTP

```
HTTP/1.1 200 OK\r\n
Content-Length: 25\r\n
\r\n
<h1>Página principal</h1>
```

- Línea de estado: versión + código + texto (`200 OK`, `404 NOT FOUND`)
- Headers: metadatos (aquí solo `Content-Length`, el tamaño del cuerpo en bytes)
- Línea vacía: separa headers del cuerpo
- Cuerpo: el contenido real (HTML en este caso)

### 2. Routing: decidir qué responder según la ruta pedida

```rust
if request_line.starts_with("GET / ") { ... }
else if request_line.starts_with("GET /users ") { ... }
else { /* 404 */ }
```

Todo framework web (Axum, Express, Flask...) hace esencialmente esto por debajo: mirar el método + ruta de la petición entrante y decidir qué código ejecutar. Aquí lo hicimos a mano con `if/else`.

### 3. Multithreading vs Async (mencionado, no implementado aún)

Usamos **threads del sistema operativo** (`std::thread::spawn`) — simple, pero cada thread tiene cierto costo de memoria (stack propio) y el sistema operativo hace el cambio de contexto.

La alternativa moderna en Rust es **async** (con `tokio`), donde muchas conexiones comparten un pool pequeño de threads reales, y el runtime hace el "cambio de tarea" de forma mucho más liviana. Es el siguiente nivel — no lo tocamos aquí, pero es la razón por la que la guía menciona "Tokio" y "Axum" como el paso avanzado de este campo.

---

## 📝 Código Final

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    println!("Petición: {}", request_line.trim());

    let (status, body) = if request_line.starts_with("GET / ") {
        ("200 OK", "<h1>Página principal</h1>")
    } else if request_line.starts_with("GET /users ") {
        ("200 OK", "<h1>Lista de usuarios</h1>")
    } else {
        ("404 NOT FOUND", "<h1>404 - No encontrado</h1>")
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
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Servidor escuchando en http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `TcpListener::bind` | Abre un puerto | Punto de entrada del servidor |
| `.incoming()` | Iterador infinito de conexiones | Servidor persistente, no de un solo uso |
| `BufReader` | Buffer de lectura | Leer línea por línea eficientemente |
| Routing manual (`if/else`) | Decidir respuesta según ruta | Base de todo framework web |
| `thread::spawn` | Hilo del SO | Atender clientes en paralelo |

---

## 🚀 Próximos Pasos

- **Proyecto 9:** Reverse Proxy (enrutar a múltiples servidores backend)
- Parsear headers de la petición (no solo la primera línea)
- Servir archivos estáticos reales
- Migrar a `tokio` + `axum` para comparar async vs threads

---

**🎉 Proyecto 8: Completado. Servidor HTTP funcional desde cero, con routing y concurrencia.**
