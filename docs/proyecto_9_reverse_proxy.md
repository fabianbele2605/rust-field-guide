# 📚 Proyecto 9: Reverse Proxy — Documentación Educativa

**Campo:** 🌐 Networking | **Nivel:** 🔴 Avanzado | **Estado:** ✅ COMPLETADO
**Carpeta:** `networking/reverse_proxy/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un proxy que recibe peticiones de clientes y las reparte entre varios servidores backend (load balancing), devolviendo la respuesta como si él mismo la hubiera generado.

```
Cliente → Proxy (8000) → Backend 1 (9001) o Backend 2 (9002), alternando
                              ↓
                       trae la respuesta
                              ↓
Cliente ← Proxy ← respuesta
```

Combina TODO lo anterior:
- Es **servidor** (Proyecto 8: `TcpListener`, `incoming()`)
- Es **cliente** (Proyecto 7: `TcpStream::connect`)
- Reenvía bytes crudos en ambas direcciones

---

## 🦀 Conceptos Rust Aprendidos

### 1. `AtomicUsize` — contador seguro entre threads sin `Mutex`

```rust
static NEXT_BACKEND: AtomicUsize = AtomicUsize::new(0);

fn pick_backend() -> &'static str {
    let index = NEXT_BACKEND.fetch_add(1, Ordering::SeqCst) % BACKENDS.len();
    BACKENDS[index]
}
```

**¿Por qué no `Mutex<usize>`?**
- `Mutex` bloquea un hilo mientras otro tiene el lock — correcto para estructuras complejas
- Para un simple contador, el CPU tiene instrucciones atómicas nativas (incrementar sin que dos threads pisen el mismo valor a la vez) — `AtomicUsize` usa esas instrucciones directamente, sin bloquear nada
- `fetch_add(1, ...)` = "suma 1 y dime cuál era el valor ANTES de sumar", de forma atómica (ninguna otra petición puede colarse en medio)

### 2. `match` para reemplazar `.unwrap()` y evitar panics

```rust
let mut backend = match TcpStream::connect(backend_addr) {
    Ok(b) => b,
    Err(e) => {
        // responder 502 y salir de la función con `return`
        return;
    }
};
```

**Lección clave de esta sesión:** `.unwrap()` en un servidor de producción es peligroso — si `TcpStream::connect` falla (backend caído), `.unwrap()` hace `panic!`, y si no hay un thread por conexión, **ese panic mata el proceso completo**, tumbando el proxy para TODOS los clientes, no solo el que pidió al backend caído.

`match` + `return` temprano soluciona esto: si algo falla, esa función simplemente termina ahí, sin matar nada más.

### 3. `is_err()` como atajo cuando no necesitas el valor de error

```rust
if backend.write_all(&request[..n]).is_err() {
    return;
}
```

Cuando no te importa el mensaje de error, solo si hubo error o no, `.is_err()` es más corto que un `match` completo.

---

## 🖥️ Conceptos de Networking / Arquitectura

### 1. Load Balancing Round-Robin

La estrategia más simple: repartir peticiones en orden fijo (1, 2, 1, 2, ...). Ventajas: trivial de implementar, reparte carga de forma pareja SI todos los backends son igual de rápidos. Desventaja: no tiene en cuenta si un backend está más cargado que otro en ese momento (para eso existen estrategias más avanzadas: least-connections, weighted round-robin, etc.)

### 2. Por qué un proxy necesita ser resiliente a fallos

Un backend real puede caerse en cualquier momento (crash, reinicio, deploy). El proxy es el único punto de contacto del cliente — si el proxy muere cuando UN backend falla, el problema se propaga a TODOS los clientes, incluso los que iban a backends sanos. Por eso manejar el error de conexión (502 Bad Gateway) en vez de crashear es la diferencia entre "degradado" y "caído".

### 3. 502 Bad Gateway

Código HTTP específico para "soy un proxy/gateway y el servidor detrás de mí no respondió correctamente" — distinto de un 500 (error del propio servidor) o 404 (recurso no encontrado).

---

## 🐛 Bug de esta sesión: panic tumbaba el proxy completo

**Síntoma:** primera petición funcionaba, pero apenas el round-robin apuntaba a un backend caído, el proxy entero moría (`thread 'main' panicked...`), y CUALQUIER petición posterior (incluso a backends sanos) fallaba con "Couldn't connect to server".

**Causa:** `TcpStream::connect(backend_addr).unwrap()` — sin threads de por medio, el panic de `main` mata el proceso completo.

**Corrección:** reemplazar cada `.unwrap()` de la ruta caliente por `match`/`is_err()` con `return` temprano, y responder 502 al cliente en vez de explotar.

**Lección general:** en cualquier servidor, `.unwrap()` en un valor que depende de la red (conexión, lectura, escritura) es una bomba de tiempo — el mundo exterior SIEMPRE puede fallar.

---

## 📝 Código Final

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
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
            return;
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
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `AtomicUsize` | Contador atómico sin bloqueo | Round-robin seguro entre threads |
| `.unwrap()` en red | Panic si falla | ⚠️ Peligroso — nunca en la ruta caliente de un servidor |
| `match`/`is_err()` + `return` | Manejo de error sin panic | El proceso sigue vivo aunque un backend caiga |
| 502 Bad Gateway | Código HTTP específico | Comunica "el backend falló", no "yo fallé" |

---

## 🚀 Próximos Pasos (fuera de este proyecto, campo completo)

Con esto se cierra el campo **Networking** (Proyectos 7, 8, 9). Mejoras posibles si se retoma:
- Health checks (dejar de enrutar a un backend caído en vez de solo devolver 502 una vez)
- Connection pooling (reutilizar conexiones a backends en vez de abrir una nueva por petición)
- Migrar a `tokio` + `axum` para async real

**Siguiente campo sugerido:** Backend (APIs REST), Distributed Systems, o volver a profundizar Kernel.

---

**🎉 Proyecto 9: Completado. Campo Networking (7, 8, 9) cerrado.**
