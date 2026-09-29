# 📚 Proyecto 21: Mini IDS (Intrusion Detection System) — Documentación Educativa

**Campo:** 🔐 Cybersecurity | **Nivel:** 🔴 Avanzado | **Estado:** ✅ COMPLETADO
**Carpeta:** `cybersecurity/mini_ids/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un sistema que detecta **port scanning** en tiempo real: si una misma IP toca muchos puertos distintos en poco tiempo, dispara una alerta. Lo verificamos corriendo el **Port Scanner del Proyecto 19 contra este mismo IDS**, viendo la detección en vivo.

```
Port Scanner (Proyecto 19)  →  6 honeypots (Mini IDS)  →  🚨 ALERTA en tiempo real
     (atacante simulado)         (defensor)
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `HashSet` para contar "distintos", no "totales"

```rust
let recent_ports: HashSet<u16> = attempts.iter()
    .filter(|a| now.duration_since(a.when) <= self.window)
    .map(|a| a.port)
    .collect();
recent_ports.len() >= self.port_threshold
```

**Por qué `HashSet` y no `Vec`:** un `HashSet` automáticamente elimina duplicados — si la misma IP toca el puerto 80 diez veces, solo cuenta UNA vez en el set. Esto es exactamente lo que necesita la regla: contar puertos ÚNICOS, no intentos totales (eso sería fuerza bruta, una regla distinta).

### 2. `Instant` y ventanas de tiempo deslizantes

```rust
now.duration_since(a.when) <= self.window
```

En vez de borrar eventos viejos activamente, simplemente los **filtramos** cada vez que evaluamos la regla: solo cuentan los que ocurrieron dentro de los últimos `window` segundos. Es una "ventana deslizante" — el conjunto de eventos relevantes cambia constantemente sin necesitar limpieza explícita del historial completo.

### 3. `Arc<Mutex<IdsAnalyzer>>` compartido entre múltiples listeners

```rust
for &port in &ports_to_watch {
    let ids = Arc::clone(&ids);
    thread::spawn(move || {
        let listener = TcpListener::bind(...).unwrap();
        // cada listener usa el MISMO analyzer compartido
    });
}
```

6 `TcpListener`s corriendo en threads distintos, cada uno escuchando SU PROPIO puerto, pero todos escribiendo al MISMO `IdsAnalyzer` — así el sistema puede correlacionar actividad de una IP a través de MÚLTIPLES puertos/servicios, no solo dentro de uno.

---

## 🐛 Bugs de esta sesión

### 1. Falta llamar a `record()` antes de `check_port_scan()`

**Síntoma:** el servidor registraba "Conexión registrada" para 6 conexiones al mismo puerto, pero nunca disparaba alerta, y el compilador avisaba "method `record` is never used" (¡a pesar de estar definido y aparentemente en uso!).

**Causa:** el código llamaba a `analyzer.check_port_scan(&peer)` pero JAMÁS a `analyzer.record(&peer, port)` — el warning del compilador era la pista correcta: si `record` de verdad nunca se invocaba en ningún lado, el historial se quedaba vacío para siempre, y la regla no tenía nada que evaluar.

**Lección:** un warning de "función nunca usada" no siempre significa "código muerto que se puede borrar" — a veces significa "olvidaste llamar esto en el flujo real", que es un bug mucho más serio.

### 2. Fórmula de chunking incorrecta al mover el punto de inicio del scanner

**Síntoma:** al intentar escanear el rango 9980-10000 (en vez de 1-1000), el scanner reportó "Escaneo completo" sin encontrar NINGÚN puerto abierto — ni siquiera los honeypots que sí estaban escuchando.

**Causa:**
```rust
let start = 9980 + i * chunk_size;           // absoluto
let end = (i + 1) * chunk_size;              // relativo a 0, sin sumar 9980
```
Al mezclar un `start` que sí sumaba el offset (9980) con un `end` que NO lo sumaba, casi todos los rangos quedaban invertidos (`start > end`), y un rango invertido en Rust (`for x in start..=end`) simplemente no itera NADA — sin error, sin panic, solo "cero resultados" silenciosamente.

**Corrección:** calcular `chunk_size` sobre el TAMAÑO real del rango (`end_port - start_port`), y sumar `start_port` consistentemente en ambos extremos de cada chunk.

**Lección:** al reparametrizar código existente (mover un rango de "empieza en 1" a "empieza en 9980"), hay que revisar TODAS las fórmulas que dependían del punto de inicio anterior — cambiar una sola línea sin ajustar las relacionadas produce bugs silenciosos, no errores de compilación.

---

## 🖥️ Conceptos de Seguridad / IDS

### 1. Qué es un IDS basado en reglas (rule-based)

Un IDS (Intrusion Detection System) basado en reglas compara el comportamiento observado contra patrones conocidos de ataque ("firmas"). Nuestra única regla ("N puertos distintos en poco tiempo = posible scan") es la versión más simple de esto — sistemas reales (Snort, Suricata) tienen miles de reglas para SQL injection, fuerza bruta, exfiltración de datos, etc.

### 2. Honeypot: un servicio diseñado para ser atacado

Los 6 `TcpListener`s (puertos 9990-9995) no ofrecen ningún servicio real — solo existen para DETECTAR quién se conecta. Esto es la idea central de un **honeypot**: un sistema señuelo que no tiene valor real, pero que registra y alerta sobre cualquiera que interactúe con él, porque nadie legítimo debería estar tocándolo.

### 3. Trade-off entre sensibilidad y falsos positivos

Bajar el `port_threshold` (por ejemplo, a 2) detectaría scans más pequeños, pero también dispararía alertas por usuarios legítimos navegando varios servicios rápido. Subirlo (a 20) evitaría falsos positivos, pero dejaría pasar scans más lentos y cuidadosos ("low and slow", una técnica real de evasión). Ajustar este balance es el trabajo central de cualquier analista de seguridad configurando un IDS real.

### 4. Verificación end-to-end: atacante real vs simulado

La diferencia entre simular eventos a mano (Pasos 2-3) y usar el Port Scanner REAL del Proyecto 19 (Paso 5) importa: confirma que el sistema funciona con tráfico de RED real, con las latencias, condiciones de carrera entre threads y comportamiento real del sistema operativo — no solo con datos de prueba perfectamente controlados.

---

## 📝 Código Final

Ver `cybersecurity/mini_ids/src/main.rs` completo. Estructura:

```rust
struct IdsAnalyzer {
    history: HashMap<String, Vec<ConnectionAttempt>>,
    window: Duration,
    port_threshold: usize,
}

impl IdsAnalyzer {
    fn record(&mut self, ip: &str, port: u16) { /* agrega al historial */ }
    fn check_port_scan(&self, ip: &str) -> bool { /* cuenta puertos únicos recientes */ }
}

// main(): N threads, cada uno con su TcpListener, todos compartiendo Arc<Mutex<IdsAnalyzer>>
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `HashSet` para deduplicar | Cuenta distintos, no totales | La regla es "puertos únicos", no "intentos totales" |
| Ventana de tiempo deslizante | Filtrar por `Instant` reciente | Detecta patrones RECIENTES, no historial completo |
| `Arc<Mutex<T>>` compartido | Un estado, múltiples listeners | Correlacionar actividad across múltiples puertos |
| Warning "nunca usado" como pista | A veces es un bug, no código muerto | Indicó que faltaba llamar `record()` en el flujo real |
| Honeypot | Servicio señuelo sin valor real | Cualquier interacción es sospechosa por definición |

---

## 🚀 Cierre del Campo Cybersecurity

Con este proyecto se cierra **Cybersecurity** (Proyectos 19, 20, 21) — y de forma especial, integrando el Proyecto 19 como "atacante" real contra el Proyecto 21 como "defensor".

**Si se retoma en el futuro:**
- Reglas adicionales (fuerza bruta: muchos intentos al MISMO puerto; escaneo "low and slow" con ventanas más largas)
- Persistir alertas a un log/archivo, no solo `println!`
- Integrar el parser del Proyecto 20 para inspeccionar contenido de paquetes, no solo metadatos de conexión
- Whitelisting de IPs conocidas (para reducir falsos positivos)

---

**🎉 Proyecto 21: Completado. Campo Cybersecurity (19, 20, 21) cerrado — con demo real de ataque/defensa entre proyectos.**
