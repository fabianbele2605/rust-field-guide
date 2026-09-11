# 📚 Proyecto 2: Process Manager — Documentación Educativa

**Nivel:** 🟡 INTERMEDIO | **Duración:** 4-7 días | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un programa CLI que **lista procesos del sistema** con información de PID, nombre, memoria y CPU.

**Entrada:**
```bash
$ cargo run
```

**Salida esperada:**
```
Listando procesos del sistema...

PID    NAME              MEMORY     CPU
1      systemd           15.2 MB    3.9%
1204   firefox           120.5 MB   23.5%
1321   code              240.9 MB   45.2%
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. **Acceso a Filesystems Especiales: `/proc`**

```rust
let proc_path = Path::new("/proc");
fs::read_dir(proc_path)
```

**¿Qué es `/proc`?**
- En Linux, `/proc` es un **filesystem virtual** (no es un disco real)
- El kernel crea archivos "virtuales" con información del sistema
- Ejemplo: `/proc/1204/status` contiene info del proceso PID 1204

**Por qué es importante:**
- Rust accede a información del kernel de forma segura
- Type safety: no confundes un int con un path
- Portabilidad: Rust abstrae diferencias entre SO

---

### 2. **Parsing de Archivos de Texto**

```rust
fn obtener_nombre_proceso(pid: &str) -> Option<String> {
    let status_path = format!("/proc/{}/status", pid);
    let contenido = fs::read_to_string(&status_path).ok()?;
    
    for linea in contenido.lines() {
        if linea.starts_with("Name:") {
            let nombre = linea.strip_prefix("Name:")?.trim();
            return Some(nombre.to_string());
        }
    }
    None
}
```

**¿Qué hace?**
1. Construye ruta: `/proc/1204/status`
2. Lee todo el archivo como string
3. Itera línea por línea
4. Busca la línea que comienza con "Name:"
5. Extrae el nombre

**Por qué es importante:**
- Parsear datos no estructurados
- Buscar información en archivos
- Manejo robusto de errores

---

### 3. **Option y el Operador `?` (Try Operator)**

```rust
let contenido = fs::read_to_string(&status_path).ok()?;
//                                                  ^^
//                    Si hay error, retorna None inmediatamente
```

**¿Qué es `?`?**
- Operador "try" que simplifica manejo de errores
- Si resultado es `Err`, retorna ese error
- Si es `Ok`, extrae el valor

**Sin `?`:**
```rust
let contenido = match fs::read_to_string(&status_path) {
    Ok(c) => c,
    Err(e) => return None,
};
```

**Con `?`:**
```rust
let contenido = fs::read_to_string(&status_path).ok()?;
```

---

### 4. **Type Conversions**

```rust
let kilobytes: u64 = valor_str.parse().ok()?;
let memoria_mb = memoria as f64 / (1024.0 * 1024.0);
```

**¿Qué es `as`?**
- Conversión de tipos explícita
- `u64 as f64` convierte entero a decimal
- Rust requiere conversión explícita (no automática)

**¿Qué es `.parse()`?**
- Convierte string a cualquier tipo
- `"1024".parse::<u64>()` → `Ok(1024)`
- Puede fallar si string no es válido

---

### 5. **Vectores y Ordenamiento**

```rust
let mut items: Vec<_> = entries.collect();
items.sort_by_key(|e| {
    let Ok(entry) = e else { return String::new() };
    entry.file_name().to_string_lossy().to_string()
});
```

**¿Qué es `Vec`?**
- Vector = lista dinámica, puede crecer
- `Vec<_>` = Rust deduce el tipo
- `mut` = mutable (puede cambiar)

**¿Por qué `collect()`?**
- Iterador → Vector (necesario para ordenar)
- Iteradores son lazy, vectores son eager

**¿Por qué `sort_by_key()`?**
- Ordena por una clave (aquí, nombre del archivo)
- Elegante y funcional

---

### 6. **Closures (Funciones Anónimas)**

```rust
items.sort_by_key(|e| {
    // ^ closure: función sin nombre
    let Ok(entry) = e else { return String::new() };
    entry.file_name().to_string_lossy().to_string()
})
```

**¿Qué es un closure?**
- Función definida en el lugar
- Puede capturar variables del scope
- Se pasa como argumento a `sort_by_key()`

**¿Por qué es útil?**
- Código compacto cerca de donde se usa
- Acceso a variables locales
- Funcional y elegante

---

### 7. **String Methods**

```rust
let valor_str = linea.strip_prefix("Name:")?.trim();
// strip_prefix: remueve "Name:" del inicio
// trim: remueve espacios en blanco
```

**Métodos comunes de String:**
- `.starts_with()` — ¿empieza con esto?
- `.strip_prefix()` — remueve prefijo
- `.trim()` — remueve espacios
- `.split_whitespace()` — divide por espacios
- `.lines()` — divide por newlines

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. **Procesos y PIDs**

```
PID = Process ID (identificador único de proceso)

Cada proceso en Linux tiene:
- PID (número)
- Name (nombre del ejecutable)
- Memory (RAM usada)
- CPU time (cuánto CPU usó)
```

**¿Por qué es importante?**
- Todo lo que corre en el SO es un proceso
- Necesitas identificarlos por PID
- Monitor de recursos = ver procesos activos

---

### 2. **Información en `/proc`**

**Estructura:**
```
/proc/
├── 1/               (PID 1 = systemd)
│   ├── status       (metadatos del proceso)
│   ├── stat         (estadísticas de CPU)
│   ├── cmdline      (línea de comandos)
│   └── maps         (memoria mapeada)
├── 2/               (PID 2)
├── 1204/            (PID 1204 = firefox)
│   ├── status
│   └── stat
```

**¿Qué información tiene `status`?**
```
Name:    firefox
State:   S (sleeping)
VmPeak:  2000000 kB (pico de memoria)
VmRSS:   120000 kB (memoria actual)
```

**¿Qué información tiene `stat`?**
```
1204 (firefox) S 1 1204 1204 0 ...
     ^         ^
     name    state
```

---

### 3. **Memoria de Procesos**

```
VmRSS = Resident Set Size (memoria física usada)
VmPeak = Peak memory (máxima memoria usada)
```

**¿Por qué es importante?**
- VmRSS es lo que realmente consume RAM
- Útil para detectar memory leaks
- Monitor de recursos usa esto

---

### 4. **CPU Time**

```
utime = user CPU time (tiempo en user space)
stime = system CPU time (tiempo en kernel space)
total_ticks = utime + stime
```

**¿Por qué es importante?**
- Saber cuánto CPU consume un proceso
- Detectar procesos que usan 100% CPU
- `top` y `ps` usan esto

---

## 📝 Código Final (Completo)

```rust
use std::fs;

fn obtener_nombre_proceso(pid: &str) -> Option<String> {
    let status_path = format!("/proc/{}/status", pid);
    let contenido = fs::read_to_string(&status_path).ok()?;
    
    for linea in contenido.lines() {
        if linea.starts_with("Name:") {
            let nombre = linea.strip_prefix("Name:")?.trim();
            return Some(nombre.to_string());
        }
    }
    None
}

fn obtener_memoria_proceso(pid: &str) -> Option<u64> {
    let status_path = format!("/proc/{}/status", pid);
    let contenido = fs::read_to_string(&status_path).ok()?;
    
    for linea in contenido.lines() {
        if linea.starts_with("VmRSS:") {
            let valor_str = linea.strip_prefix("VmRSS:")?.trim();
            let valor_str = valor_str.split_whitespace().next()?;
            let kilobytes: u64 = valor_str.parse().ok()?;
            return Some(kilobytes * 1024);
        }
    }
    None
}

fn obtener_cpu_proceso(pid: &str) -> Option<f64> {
    let stat_path = format!("/proc/{}/stat", pid);
    let contenido = fs::read_to_string(&stat_path).ok()?;
    
    let campos: Vec<&str> = contenido.split_whitespace().collect();
    
    if campos.len() < 15 {
        return None;
    }
    
    let utime: u64 = campos[13].parse().ok()?;
    let stime: u64 = campos[14].parse().ok()?;
    
    let total_ticks = utime + stime;
    
    Some(total_ticks as f64 / 100.0)
}

fn main() {
    println!("Listando procesos del sistema...\n");
    
    let proc_path = std::path::Path::new("/proc");
    
    match fs::read_dir(proc_path) {
        Ok(entries) => {
            // Recolectar y ordenar
            let mut items: Vec<_> = entries.collect();
            items.sort_by_key(|e| {
                let Ok(entry) = e else { return String::new() };
                entry.file_name().to_string_lossy().to_string()
            });
            
            // Procesar cada entrada
            for entry in items {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy().to_string();
                    
                    // Solo procesar PIDs (nombres que son números)
                    if name.chars().all(|c| c.is_digit(10)) {
                        if let Some(nombre) = obtener_nombre_proceso(&name) {
                            let memoria = obtener_memoria_proceso(&name)
                                .unwrap_or(0);
                            let memoria_mb = memoria as f64 / (1024.0 * 1024.0);
                            
                            let cpu = obtener_cpu_proceso(&name)
                                .unwrap_or(0.0);
                            
                            println!("{:<6} {:<20} {:<10} {:.1}%", 
                                name, nombre, 
                                format!("{:.1} MB", memoria_mb),
                                cpu);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error leyendo /proc: {}", e);
        }
    }
}
```

---

## 🎓 Lecciones Clave

### Lección 1: Acceso Seguro a Información del Kernel
**Lo que podrías pensar:** Necesito usar `ioctl()` y syscalls directas.

**La realidad de Rust:** Rust abstrae `/proc` como filesystem normal. Type safe.

```rust
let contenido = fs::read_to_string("/proc/1204/status")?;
// Rust verifica: ese archivo existe? tiene permisos? etc
```

---

### Lección 2: Parsing es Frágil, Manéjalo con Cuidado
**Lo que podrías pensar:** Busco "Name:" y ya está.

**La realidad de Rust:** Formato puede cambiar, línea puede no existir.

```rust
for linea in contenido.lines() {
    if linea.starts_with("Name:") {  // verifica antes
        // ... extrae valor
    }
}
// Si "Name:" no existe, devuelve None, no crash
```

---

### Lección 3: Conversiones de Tipos Son Explícitas
**Lo que podrías pensar:** Un número es un número.

**La realidad de Rust:** u64 ≠ f64. Conversión explícita.

```rust
let kilobytes: u64 = 1024;
let megabytes: f64 = kilobytes as f64 / 1024.0;
// ^ conversión explícita
```

---

### Lección 4: Iteradores Lazy Son Eficientes
**Lo que podrías pensar:** Leer 1000 procesos en memoria.

**La realidad de Rust:** `read_dir()` genera archivos bajo demanda.

```rust
for entry in fs::read_dir("/proc") {
    // Procesa UNO a la vez, no carga todos en memoria
}
```

---

## ⚠️ Errores Comunes

### Error 1: Asumir Que Archivos Siempre Existen
❌ **Incorrecto:**
```rust
let contenido = fs::read_to_string("/proc/1204/status").unwrap();
// crash si archivo no existe
```

✅ **Correcto:**
```rust
let contenido = fs::read_to_string("/proc/1204/status").ok()?;
// devuelve None si no existe, manejo graceful
```

---

### Error 2: No Verificar Índices en Arrays
❌ **Incorrecto:**
```rust
let campos: Vec<&str> = contenido.split_whitespace().collect();
let utime: u64 = campos[13].parse().ok()?;
// crash si campos.len() < 14
```

✅ **Correcto:**
```rust
if campos.len() < 15 {
    return None;
}
let utime: u64 = campos[13].parse().ok()?;
```

---

### Error 3: Olvidar Manejo de Encoding
❌ **Incorrecto:**
```rust
let nombre = linea.strip_prefix("Name:")?.to_string();
// ¿qué si tiene caracteres especiales?
```

✅ **Correcto:**
```rust
let nombre = linea.strip_prefix("Name:")?.trim().to_string();
// Rust String es UTF-8, seguro por defecto
```

---

## 🚀 Lo que Este Proyecto Te Preparó Para

**Conceptos Avanzados:**
- **Async I/O:** Leer múltiples `/proc/[pid]/status` concurrentemente
- **Benchmarking:** Comparar performance de diferentes formas de parsear
- **Real-time Monitoring:** Polling periódico para ver cambios
- **Data Structures:** HashMap para cachear info de procesos

**Proyectos Siguientes:**
- Proyecto 3: Mini Shell — Ejecutar procesos, control de recursos
- Proyecto 4-6: Kernels — Entender scheduling, interrupts

---

## 📊 Resumen de Conceptos

| Concepto | Aprendiste | Aplicación |
|----------|-----------|-----------|
| **/proc filesystem** | Acceso a info del kernel | Monitoring de SO |
| **String parsing** | Buscar en archivos | Parsear logs, configs |
| **Option/?** | Manejo elegante de errores | Código robusto |
| **Type conversion** | u64 → f64, string → int | Transformación de datos |
| **Vectors** | Colecciones dinámicas | Almacenar múltiples items |
| **Sorting** | Ordenar por clave | Presentación legible |
| **Closures** | Funciones anónimas | Callbacks, funcional |

---

## ✅ Checklist de Comprensión

Después de este proyecto, deberías entender:

- [ ] Qué es `/proc` y cómo contiene info del SO
- [ ] Cómo parsear archivos de texto línea por línea
- [ ] Diferencia entre VmRSS y VmPeak
- [ ] Cómo convertir kilobytes a megabytes
- [ ] Qué es un closure y cuándo usarlo
- [ ] Cómo el operador `?` simplifica manejo de errores
- [ ] Por qué Rust es seguro incluso con acceso a sistemas

---

## 🔗 Comparativa: File Explorer vs Process Manager

| Aspecto | Proyecto 1 | Proyecto 2 |
|--------|-----------|-----------|
| **Lectura** | Filesystem normal | Filesystem virtual (`/proc`) |
| **Parsing** | Metadatos (is_dir, len) | Archivos de texto |
| **Complejidad** | Básica | Media |
| **Errores** | Pocos | Muchos posibles |
| **Conceptos** | Ownership, iterators | Parsing, conversiones |

---

## 🎓 Conclusión

**Proyecto 2 te enseñó:**
- Rust accede a información del kernel de forma segura
- Parsing de datos no estructurados
- Manejo robusto de errores con `?`
- Type safety en conversiones
- Closures y ordenamiento funcional

**Siguiente paso:** Proyecto 3 — Mini Shell (ejecutar procesos, lo más complejo)

---

**Creado:** 2026-08-25 | **Versión:** 1.0 | **Status:** ✅ Completo
