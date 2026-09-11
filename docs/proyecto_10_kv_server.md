# 📚 Proyecto 10: Key/Value Server — Documentación Educativa

**Campo:** ☁️ Distributed Systems / Cloud | **Nivel:** 🟢 Básico | **Estado:** ✅ COMPLETADO
**Carpeta:** `distributed-systems/kv_server/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un servidor tipo Redis simplificado: guarda pares clave-valor en memoria y responde a un protocolo de texto propio.

```
Cliente → "SET name Juan"  → Servidor guarda en HashMap → "OK"
Cliente → "GET name"       → Servidor busca en HashMap  → "Juan"
Cliente → "DEL name"       → Servidor borra del HashMap → "OK"
Cliente → "GET name"       → No existe                  → "NOT FOUND"
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `Arc<Mutex<T>>` — el patrón de concurrencia más común en Rust

```rust
type Store = Arc<Mutex<HashMap<String, String>>>;
let store: Store = Arc::new(Mutex::new(HashMap::new()));
```

**Descomponiendo la capa:**
```
HashMap<String, String>   → los datos reales
Mutex<HashMap<...>>       → protege el acceso (solo un thread a la vez puede tocar el HashMap)
Arc<Mutex<...>>           → permite que VARIOS threads posean una referencia al mismo Mutex
```

**¿Por qué las dos capas?**
- `Mutex` por sí solo no se puede compartir entre threads que empiezan como closures distintos — cada `thread::spawn` necesita SU PROPIA copia de la variable
- `Arc` ("Atomic Reference Counted") permite tener múltiples "copias" que en realidad apuntan al mismo dato en memoria — clonar un `Arc` solo incrementa un contador, no copia el `HashMap`

### 2. `Arc::clone` dentro del loop, antes de mover al thread

```rust
for stream in listener.incoming() {
    let store = Arc::clone(&store);       // nueva referencia
    thread::spawn(move || handle_client(stream, store));  // el thread es DUEÑO de esa referencia
}
```

El `move` fuerza al closure a tomar posesión de `store` (la copia del `Arc`, no el original) y de `stream`. Cada thread tiene su propio `Arc` apuntando al mismo `Mutex`.

### 3. `match` sobre un slice de `&str` — parsear un protocolo de texto

```rust
let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
match parts.as_slice() {
    ["SET", key, value] => { ... }
    ["GET", key] => { ... }
    ["DEL", key] => { ... }
    _ => { ... }
}
```

**¿Qué hace `splitn(3, ' ')`?**
- Divide el string por espacios, pero como MÁXIMO en 3 partes
- Así, `"SET name Juan Pérez"` da `["SET", "name", "Juan Pérez"]` (el valor conserva sus espacios internos), en vez de partirse en 4 pedazos

**¿Qué hace el `match` con patrones de array?**
- `["SET", key, value]` hace pattern matching sobre la FORMA del array: "si tiene exactamente 3 elementos y el primero es literalmente `"SET"`, dame el 2do y 3ro como `key`/`value`"
- Es mucho más legible que una cadena de `if`/`else` comparando índices manualmente

### 4. `try_clone()` en un `TcpStream`

```rust
let mut writer = stream.try_clone().unwrap();
let reader = BufReader::new(stream);
```

Necesitamos LEER (con `BufReader`) y ESCRIBIR (`writer`) sobre la misma conexión al mismo tiempo. `try_clone()` da un segundo "handle" al mismo socket subyacente — ambos apuntan a la misma conexión real, permitiendo separar la parte de lectura de la de escritura.

---

## 🖥️ Conceptos de Sistemas Distribuidos

### 1. ¿Por qué "Distributed Systems" empieza con un servidor NO distribuido?

Este proyecto corre en una sola máquina, un solo proceso. Es la base necesaria antes de distribuir: primero entender cómo almacenar y servir datos correctamente CON concurrencia (múltiples clientes a la vez), luego (Proyecto 11) replicar ese mismo storage entre varias máquinas.

### 2. Protocolo de texto simple vs binario

Redis real usa un protocolo binario (RESP) por eficiencia. Aquí usamos texto plano línea por línea — más fácil de debuggear (puedes hablarle con `nc`/`telnet` a mano), a costa de ser menos eficiente. Es la misma decisión de diseño que HTTP (texto) vs protocolos binarios como gRPC.

---

## 📝 Código Final

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

type Store = Arc<Mutex<HashMap<String, String>>>;

fn handle_client(stream: TcpStream, store: Store) {
    let mut writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
        let response = match parts.as_slice() {
            ["SET", key, value] => {
                let mut s = store.lock().unwrap();
                s.insert(key.to_string(), value.to_string());
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
                "OK\n".to_string()
            }
            _ => "ERROR: comando desconocido\n".to_string(),
        };

        if writer.write_all(response.as_bytes()).is_err() {
            break;
        }
    }
}

fn main() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    println!("KV Server escuchando en 127.0.0.1:6380");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || handle_client(stream, store));
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
| `Arc<Mutex<T>>` | Referencia compartida + exclusión mutua | Múltiples threads, un solo dato protegido |
| `Arc::clone` | Copia barata (solo contador) | Cada thread necesita su propia referencia |
| `match` sobre slice | Pattern matching por forma del array | Parsear protocolos de texto de forma legible |
| `try_clone()` en socket | Segundo handle a la misma conexión | Leer y escribir simultáneamente |

---

## 🚀 Próximos Pasos

- **Proyecto 11:** Distributed Key/Value Store (replicar este storage entre varios nodos)
- Persistencia a disco (ahora mismo, todo se pierde al reiniciar)
- TTL (expiración automática de claves)

---

**🎉 Proyecto 10: Completado. Primer proyecto del campo Distributed Systems.**
