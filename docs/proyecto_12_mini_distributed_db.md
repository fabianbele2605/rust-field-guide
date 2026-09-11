# 📚 Proyecto 12: Mini Distributed Database — Documentación Educativa

**Campo:** ☁️ Distributed Systems / Cloud | **Nivel:** 🔴 Avanzado (versión simplificada) | **Estado:** ✅ COMPLETADO
**Carpeta:** `distributed-systems/mini_db/`

> **Nota de alcance:** la guía original pide Raft/Paxos y leader election automática — eso es semanas de trabajo incluso para ingenieros experimentados. Esta versión simplifica a: roles fijos (Leader/Follower asignados manualmente), WAL real, y recovery real. Se gana entender los 3 pilares de una DB distribuida con código completo y legible, a cambio de no implementar el algoritmo de consenso en sí.

---

## 🎯 Resumen: ¿Qué Construimos?

Extendimos `kv_node` (Proyecto 11) con tres piezas nuevas:

1. **Roles explícitos** — un nodo es Leader (acepta escrituras de clientes) o Follower (solo acepta réplicas)
2. **Write-Ahead Log (WAL)** — cada cambio se persiste a disco ANTES/junto con aplicarse en memoria
3. **Recovery** — al iniciar, el nodo relee su WAL y reconstruye su `HashMap`

```
Cliente → Leader: SET name Fabian
              ↓ aplica en memoria
              ↓ escribe en wal_6380.log
              ↓ replica a Follower
Follower: recibe réplica
              ↓ aplica en memoria
              ↓ escribe en wal_6381.log

--- proceso muere y reinicia ---

Leader: lee wal_6380.log → reconstruye HashMap → sigue como si nada
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `Copy` en un enum simple — por qué importa en threads

```rust
#[derive(Clone, Copy, PartialEq)]
enum Role {
    Leader,
    Follower,
}
```

**Bug real de esta sesión:** sin `Copy`, cada `thread::spawn(move || ...)` dentro del loop `for stream in listener.incoming()` intenta MOVER `role` hacia el closure. La primera iteración se lo queda; en la segunda iteración, `role` ya no existe (fue movido) → error de compilación: *"value moved... in previous iteration of loop"*.

Con `Copy`, cada iteración obtiene automáticamente su propia copia (barata, es solo un byte) — no hay "movimiento" real, solo duplicación.

### 2. `OpenOptions` con `.append(true)` — escribir sin borrar

```rust
OpenOptions::new().create(true).append(true).open(wal_path)
```

- `.create(true)` — si el archivo no existe, créalo
- `.append(true)` — cada escritura se agrega al FINAL, nunca sobrescribe lo existente

Es la diferencia entre "reemplazar el archivo" y "seguir agregando líneas" — fundamental para un log que debe conservar TODA la historia.

### 3. Reutilizar el mismo formato de texto para 3 propósitos distintos

El comando `"SET name Fabian"` se usa, sin cambiar su forma, para:
- El protocolo cliente→servidor (Proyecto 10)
- El mensaje de replicación, con prefijo `REPLICATE ` (Proyecto 11)
- La línea del WAL en disco (Proyecto 12)

**Por qué es una buena decisión de diseño:** un solo parser (`splitn(3, ' ')` + `match ["SET", key, value]`) sirve para los tres casos, en vez de tener tres formatos distintos que mantener.

### 4. `recover_from_wal` — aplicar comandos sin pasar por la red

```rust
fn recover_from_wal(wal_path: &str, store: &Store) {
    if let Ok(file) = std::fs::File::open(wal_path) {
        let reader = BufReader::new(file);
        let mut s = store.lock().unwrap();
        for line in reader.lines() {
            if let Ok(command) = line {
                let parts: Vec<&str> = command.splitn(3, ' ').collect();
                match parts.as_slice() {
                    ["SET", key, value] => { s.insert(key.to_string(), value.to_string()); }
                    ["DEL", key] => { s.remove(*key); }
                    _ => {}
                }
            }
        }
    }
}
```

Nota que esta función toma el lock UNA sola vez (`store.lock().unwrap()` antes del loop) y aplica todas las líneas dentro de esa única sección crítica — más eficiente que bloquear/desbloquear en cada línea, y correcto porque el recovery ocurre antes de aceptar conexiones (no hay concurrencia real todavía en ese momento).

---

## 🖥️ Conceptos de Bases de Datos Distribuidas

### 1. Write-Ahead Log — la base de la durabilidad

Casi toda base de datos real (PostgreSQL, SQLite, Kafka) usa esta misma idea: antes de que un cambio se considere "confirmado", debe estar escrito en un log en disco. Si el proceso muere justo después, al reiniciar puede "reproducir" el log y llegar exactamente al mismo estado que tenía. La clave es el ORDEN: escribir al log primero, aplicar en memoria (o al revés, pero de forma consistente) — nunca "confirmar" al cliente sin haber persistido.

### 2. Leader/Follower vs Leader Election

Lo que construimos: los roles se **asignan a mano** al arrancar (`leader` o `follower` como argumento). Un sistema real con Raft/Paxos haría esto automáticamente: los nodos "votan" entre ellos, y si el Leader se cae, los Followers detectan la ausencia (timeout) y eligen uno nuevo sin intervención humana. Esa parte — la más difícil — es la que dejamos fuera intencionalmente.

### 3. ¿Qué pasa si el Leader se cae en nuestra versión?

Con roles fijos: nada automático. El Follower se queda sirviendo lecturas (`GET`) de los datos que tenía, pero rechaza escrituras (no hay Leader disponible para promover). Un humano tendría que reiniciar ese nodo como `leader` manualmente. Es la limitación central de esta versión simplificada, y el motivo por el que Raft/Paxos existen.

---

## 📝 Código Final

Ver `distributed-systems/mini_db/src/main.rs` completo. Piezas clave añadidas sobre el Proyecto 11:

```rust
#[derive(Clone, Copy, PartialEq)]
enum Role { Leader, Follower }

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
                    ["SET", key, value] => { s.insert(key.to_string(), value.to_string()); }
                    ["DEL", key] => { s.remove(*key); }
                    _ => {}
                }
            }
        }
    }
}

// En main(): antes de escuchar el puerto
let wal_path_main = format!("wal_{}.log", port);
recover_from_wal(&wal_path_main, &store);

// En handler_client(): un Follower rechaza escrituras directas
if role == Role::Follower && !is_replicated {
    // ... responde error si el comando es SET/DEL
}

// En las ramas SET/DEL: persistir antes de replicar
append_to_wal(&wal_path, &real_command);
if !is_replicated {
    if let Some(peer) = &peer_addr { replicate(peer, &real_command); }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `Copy` en enum | Duplicar en vez de mover | Necesario para usar el valor en cada iteración de un loop con threads |
| `OpenOptions().append(true)` | Escritura que no borra | Preservar la historia completa en el WAL |
| WAL | Persistencia a disco antes de confirmar | Sobrevivir a un crash sin perder datos |
| Recovery | Releer el WAL al iniciar | Reconstruir el estado en memoria tras un reinicio |
| Leader/Follower fijo | Roles sin elección automática | Simplificación — el paso completo sería Raft/Paxos |

---

## 🚀 Cierre del Campo Distributed Systems

Con este proyecto se cierra **Distributed Systems** (Proyectos 10, 11, 12).

**Si se retoma en el futuro:**
- Implementar leader election real (timeout + votación)
- Snapshotting (compactar el WAL cuando crece mucho)
- Reintentos de replicación si el peer está caído temporalmente
- Un capstone real: combinar esto con Networking (Proyecto 9) para un sistema con proxy + réplicas + failover

---

**🎉 Proyecto 12: Completado. Campo Distributed Systems (10, 11, 12) cerrado.**
