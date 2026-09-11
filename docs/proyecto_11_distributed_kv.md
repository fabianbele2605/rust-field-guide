# 📚 Proyecto 11: Distributed Key/Value Store — Documentación Educativa

**Campo:** ☁️ Distributed Systems / Cloud | **Nivel:** 🟡 Intermedio | **Estado:** ✅ COMPLETADO
**Carpeta:** `distributed-systems/kv_node/`

---

## 🎯 Resumen: ¿Qué Construimos?

Dos nodos del Proyecto 10 (Key/Value Server) que se replican entre sí: cuando un cliente hace `SET` en Node A, ese cambio se propaga automáticamente a Node B.

```
Cliente → Node A (6380): SET name Noah
              ↓ guarda localmente
              ↓ abre conexión a Node B, envía "REPLICATE SET name Noah"
Node B (6381): recibe, detecta que es replicación, guarda SIN reenviar de vuelta
              ↓
Cliente → Node B: GET name → "Noah" (sin haberlo escrito ahí)
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Un nodo es servidor Y cliente a la vez

```rust
fn replicate(peer_addr: &str, command: &str) {
    if let Ok(mut stream) = TcpStream::connect(peer_addr) {
        let msg = format!("REPLICATE {}\n", command);
        let _ = stream.write_all(msg.as_bytes());
    }
}
```

Dentro del mismo `handle_client` (que corre porque el nodo es SERVIDOR, recibiendo clientes), cuando hace falta replicar, el propio nodo actúa como CLIENTE hacia su peer (`TcpStream::connect`, exactamente como el Proyecto 7). Es el mismo patrón que ya usaste en el Reverse Proxy (Proyecto 9): "servidor de un lado, cliente del otro".

### 2. `strip_prefix` — distinguir dos tipos de mensajes en el mismo protocolo

```rust
let (is_replicated, real_command) = if let Some(rest) = line.strip_prefix("REPLICATE ") {
    (true, rest.to_string())
} else {
    (false, line.to_string())
};
```

**¿Qué hace `strip_prefix`?**
- Si el string EMPIEZA con ese prefijo exacto, devuelve `Some(resto_después_del_prefijo)`
- Si no, devuelve `None`

Así diferenciamos: ¿este `SET name Noah` vino de un cliente humano, o es una réplica que llegó de otro nodo? La respuesta cambia el comportamiento (reenviar o no reenviar).

### 3. `Option<String>` para "puede o no tener peer"

```rust
let peer_addr: Option<String> = args.get(2).cloned();
```

`args.get(2)` devuelve `Option<&String>` (puede no existir un tercer argumento). `.cloned()` convierte eso en `Option<String>` (dueño de su propio dato, no una referencia prestada) — necesario porque este valor debe viajar a otro thread más adelante.

---

## 🐛 Bug de esta sesión: falta un espacio, y el fallo fue silencioso

**Código con el bug:**
```rust
line.strip_prefix("REPLICATE")   // ❌ sin espacio
```

**Síntoma:** `SET` en Node A funcionaba (respondía "OK"), pero Node B nunca recibía el dato — y NO había ningún error visible en consola.

**Causa exacta:**
1. Node A envía: `"REPLICATE SET name Noah"`
2. `strip_prefix("REPLICATE")` (sin espacio) quita solo `"REPLICATE"`, dejando `" SET name Noah"` — **con un espacio sobrante al inicio**
3. `" SET name Noah".splitn(3, ' ')` produce `["", "SET", "name Noah"]` — el primer elemento es un string VACÍO, no `"SET"`
4. El `match ["SET", key, value]` nunca coincide (el primer elemento no es literalmente `"SET"`)
5. Cae en `_ => "ERROR: comando desconocido"` — pero como es un comando REPLICADO, esa respuesta ni se envía a ningún lado (`if is_replicated { continue; }` la descarta)

**Por qué fue tan difícil de ver:** el error ocurría, pero el propio diseño del protocolo (no responder nada a comandos replicados) hacía que el error se descartara en silencio, sin ningún log. 

**Lección:** cuando un flujo "no responde nada" por diseño, agregar un `println!` de depuración temporal en la rama de error ayuda muchísimo — aquí lo diagnosticamos leyendo el código con cuidado, comparando byte a byte el prefijo esperado vs el realmente enviado.

**Corrección:** `strip_prefix("REPLICATE ")` — con el espacio incluido en el prefijo a remover.

---

## 🖥️ Conceptos de Sistemas Distribuidos

### 1. Replicación simple (sin consenso)

Lo que construimos es la forma MÁS básica de replicación: "cuando cambio algo, se lo cuento a mi peer". No hay:
- **Consenso** (¿qué pasa si dos clientes escriben al mismo tiempo en nodos distintos con el mismo key?) → posible inconsistencia
- **Detección de fallos** (si Node B está caído, `replicate()` simplemente falla silenciosamente esa vez, y los datos quedan desincronizados para siempre)
- **Reintentos** (si la réplica falla, no se reintenta)

Esto es intencional: es la base antes de introducir algoritmos de consenso reales (Raft, Paxos) en el Proyecto 12.

### 2. El problema de "split-brain" (para pensar, no resuelto aquí)

Si Node A y Node B pierden la conexión entre sí pero ambos siguen aceptando escrituras de clientes distintos, terminan con datos diferentes y no hay forma automática de saber cuál es la versión "correcta" al reconectar. Es uno de los problemas centrales que resuelven los algoritmos de consenso.

---

## 📝 Código Final

Ver `distributed-systems/kv_node/src/main.rs` — estructura principal:

```rust
fn replicate(peer_addr: &str, command: &str) {
    if let Ok(mut stream) = TcpStream::connect(peer_addr) {
        let msg = format!("REPLICATE {}\n", command);
        let _ = stream.write_all(msg.as_bytes());
    } else {
        println!("No se pudo replicar a {} (¿está caído?)", peer_addr);
    }
}

// Dentro de handle_client:
let (is_replicated, real_command) = if let Some(rest) = line.strip_prefix("REPLICATE ") {
    (true, rest.to_string())
} else {
    (false, line.to_string())
};

// En SET/DEL: solo replicar si NO vino ya replicado (evita loop infinito A→B→A→B...)
if !is_replicated {
    if let Some(peer) = &peer_addr {
        replicate(peer, &real_command);
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| Nodo servidor+cliente | Mismo proceso hace ambos roles | Replicar requiere conectarse como cliente al peer |
| `strip_prefix` | Detectar y quitar un prefijo exacto | Distinguir comando de cliente vs réplica |
| Flag `is_replicated` | Evita loop infinito de replicación | Sin él, A replica a B, B replicaría de vuelta a A... |
| Bug de espacio faltante | Fallo silencioso en parsing | Los protocolos de texto son frágiles a espacios exactos |

---

## 🚀 Próximos Pasos

- **Proyecto 12:** Mini Distributed Database (leader/follower real, recuperación ante fallos)
- Reintentos si `replicate()` falla
- Extender a 3+ nodos
- Algoritmo de consenso simple (mayoría de votos)

---

**🎉 Proyecto 11: Completado. Primera replicación real entre dos nodos.**
