# 📚 Proyecto 1: File Explorer CLI — Documentación Educativa

**Nivel:** 🟢 BÁSICO | **Duración:** 2-4 días | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un programa CLI que **explora directorios y muestra archivos** de forma legible.

**Entrada:**
```bash
$ cargo run -- .
```

**Salida esperada:**
```
Contenido de: .

📁 src/
📄 Cargo.lock (157 B)
📄 Cargo.toml (84 B)
📁 target/
📄 .gitignore (8 B)
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. **std::fs — Acceso al Filesystem**

```rust
let entries = fs::read_dir(".")?;
```

**¿Qué es?**
- `fs` = módulo de la librería estándar para operaciones de archivos
- `read_dir()` = función que lee contenido de un directorio
- Devuelve un **iterador** sobre las entradas

**Por qué es importante:**
- Rust proporciona abstracciones seguras para acceder al SO
- No hay peligro de buffer overflows o memory leaks
- El compilador verifica manejo de errores

---

### 2. **Result y Option — Manejo de Errores**

```rust
match fs::read_dir(ruta) {
    Ok(entries) => { /* éxito */ }
    Err(e) => { /* error */ }
}
```

**¿Qué es?**
- `Result<T, E>` = un enum que dice: "esto PODRÍA fallar"
- `Ok(value)` = caso exitoso
- `Err(error)` = caso de error
- `match` = analizar ambos casos

**Por qué es importante:**
- En muchos lenguajes, los errores se ignoran silenciosamente
- En Rust, DEBES manejar errores explícitamente
- El compilador te obliga a ser seguro

---

### 3. **Ownership y Borrowing**

```rust
let name = path.file_name().unwrap().to_string_lossy();
if let Some(nombre) = obtener_nombre_proceso(&name) {
    // usar nombre
}
```

**¿Qué es?**
- `&name` = "préstamo" de la variable (borrowing)
- La función `obtener_nombre_proceso()` NO consume `name`
- Después de la función, `name` sigue siendo válido

**Por qué es importante:**
- Rust evita que múltiples partes del código modifiquen lo mismo
- No hay garbage collection, pero tampoco memory leaks
- El compilador verifica seguridad de memoria en tiempo de compilación

---

### 4. **Iteradores**

```rust
for entry in entries {
    if let Ok(entry) = entry {
        // procesar cada entrada
    }
}
```

**¿Qué es?**
- `entries` es un **iterador** (genera items uno a uno)
- `for` consume el iterador automáticamente
- Cada item es un `Result` (puede ser OK o Error)

**Por qué es importante:**
- Los iteradores son lazy (no generan todos los items upfront)
- Eficientes en memoria incluso con millones de archivos
- Expresivo y funcional

---

### 5. **Structs y Enums**

```rust
let metadata = entry.metadata().unwrap();
if metadata.is_dir() { /* carpeta */ }
else { /* archivo */ }
```

**¿Qué es?**
- `metadata` es un **struct** que contiene info del archivo
- Tiene campos: `is_dir()`, `len()`, etc.
- Rust struct = datos organizados juntos

**Por qué es importante:**
- Organiza datos relacionados
- Type safety: no confundes un archivo con un string

---

### 6. **Funciones y Composición**

```rust
fn formato_tamaño(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
```

**¿Qué es?**
- Función que transforma bytes en un string legible
- Usa `format!` macro para construir strings
- Devuelve un `String`

**Por qué es importante:**
- Reutilizable: llamamos en múltiples lugares
- Responsabilidad única: solo formatea tamaños
- Fácil de probar y mantener

---

### 7. **String Formatting**

```rust
println!("{:<6} {:<20} {:.1} MB", name, nombre, memoria_mb);
```

**Especificadores:**
- `{:<6}` = alinear a izquierda, ancho 6 caracteres
- `{:<20}` = alinear a izquierda, ancho 20 caracteres
- `{:.1}` = mostrar 1 decimal

**Por qué es importante:**
- Hace salida legible en columnas
- Professional output
- Control fino sobre formato

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. **Filesystem Hierarchy**

Tu programa accede a:
```
/home/usuario/proyecto/
├── src/
├── Cargo.toml
├── Cargo.lock
└── target/
```

**¿Qué es?**
- Los archivos/carpetas están organizados en árbol
- `/` es la raíz (root)
- Cada archivo tiene un path único

**Por qué importa en Rust:**
- `Path` y `PathBuf` son tipos Rust para rutas seguras
- Evitan problemas de path traversal
- Funciona en Windows y Linux

---

### 2. **File Metadata**

Cuando accedes a un archivo, el SO guarda:
- **Tipo:** ¿es archivo o carpeta? (`is_dir()`)
- **Tamaño:** en bytes (`len()`)
- **Permisos:** quién puede leer/escribir
- **Timestamps:** cuándo fue modificado

**¿Por qué es importante?**
- Necesitas esta info para mostrar en el explorer
- El kernel la mantiene, tu programa la lee
- Rust lo hace seguro con type safety

---

### 3. **Abstracción de la API del SO**

```rust
fs::read_dir()  // Rust abstrae syscall opendir() del kernel
```

**Debajo hay:**
- Llamada al kernel (syscall)
- El kernel busca en el disco
- Devuelve información
- Rust la convierte en Rust types

**Por qué importa:**
- Rust abstrae detalles del SO
- Código portátil (funciona en Linux, Windows, Mac)
- Seguro: no puedes cometer errores comunes

---

## 📝 Código Final (Completo)

```rust
use std::fs;

fn formato_tamaño(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let ruta = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
    
    match fs::read_dir(ruta) {
        Ok(entries) => {
            println!("Contenido de: {}\n", ruta);
            
            // Ordenar alfabéticamente
            let mut items: Vec<_> = entries.collect();
            items.sort_by_key(|e| {
                let Ok(entry) = e else { return String::new() };
                entry.file_name().to_string_lossy().to_string()
            });
            
            for entry in items {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().unwrap();
                    let name = path.file_name().unwrap().to_string_lossy();
                    let tamaño = metadata.len();
                    
                    if metadata.is_dir() {
                        println!("📁 {}/", name);
                    } else {
                        let tamaño_formateado = formato_tamaño(tamaño);
                        println!("📄 {} ({})", name, tamaño_formateado);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error al leer la carpeta: {}", e);
        }
    }
}
```

---

## 🎓 Lecciones Clave

### Lección 1: Errores No Son Excepciones
**Lo que podrías pensar:** Los errores "se lanzan" y el programa se cuelga.

**La realidad de Rust:** Los errores son valores (`Result`). Debes decidir qué hacer.

```rust
match fs::read_dir(ruta) {
    Ok(entries) => { /* manejar éxito */ }
    Err(e) => { /* manejar error */ }
}
```

---

### Lección 2: Ownership Evita Memory Leaks
**Lo que podrías pensar:** Necesito un garbage collector.

**La realidad de Rust:** El compilador rastrrea quién posee cada valor. Sin GC, sin leaks.

```rust
let name = path.file_name().unwrap().to_string_lossy();
// 'name' existe aquí
// Si salimos del scope, se libera automáticamente
```

---

### Lección 3: Iteradores Son Eficientes
**Lo que podrías pensar:** Leer un millón de archivos es lento.

**La realidad de Rust:** Los iteradores son lazy. Procesa uno por uno, sin almacenar todo.

```rust
for entry in entries {  // lazy: genera cada 'entry' bajo demanda
    // procesar
}
```

---

### Lección 4: Type Safety Previene Bugs
**Lo que podrías pensar:** Es solo un número (bytes, kilobytes, etc).

**La realidad de Rust:** El compilador distingue tipos. No confundes bytes con kilobytes.

```rust
let bytes: u64 = 1024;  // u64 = unsigned 64-bit integer
let kb: f64 = bytes as f64 / 1024.0;  // conversión explícita
```

---

## ⚠️ Errores Comunes y Cómo Evitarlos

### Error 1: Olvidar Manejar el Error
❌ **Incorrecto:**
```rust
let entries = fs::read_dir(ruta);  // ignora Err
for entry in entries {  // ¡crash si ruta no existe!
```

✅ **Correcto:**
```rust
match fs::read_dir(ruta) {
    Ok(entries) => { /* procesar */ }
    Err(e) => { println!("Error: {}", e); }
}
```

---

### Error 2: Usar `unwrap()` Sin Cuidado
❌ **Incorrecto:**
```rust
let name = path.file_name().unwrap();  // crash si no existe
```

✅ **Correcto:**
```rust
if let Some(name) = path.file_name() {
    // usar name seguramente
}
```

---

### Error 3: No Entender Ownership
❌ **Incorrecto:**
```rust
let name = "archivo";
println!("{}", name);
println!("{}", name);  // ¿name se movió? No, strings son Copy
```

✅ **Correcto:**
- Strings pequeños se copian automáticamente
- Para grandes datos, usa `&` (borrowing)

---

## 🚀 Lo que Este Proyecto Te Preparó Para

**Proyectos Siguientes:**
- Proyecto 2: Process Manager — Lee `/proc`, parsing más complejo
- Proyecto 3: Mini Shell — Ejecuta procesos

**Conceptos Avanzados:**
- Concurrencia: múltiples threads leyendo directorios
- Async/await: lectura no-bloqueante de archivos
- File I/O: leer/escribir contenidos de archivos

---

## 📊 Resumen de Conceptos

| Concepto | Aprendiste | Aplicación |
|----------|-----------|-----------|
| **std::fs** | Acceso a filesystem | Leer/escribir archivos |
| **Result/Option** | Manejo de errores | Todo que pueda fallar |
| **Ownership** | Memoria segura | Prevent leaks automáticamente |
| **Iterators** | Procesamiento lazy | Datos grandes eficientemente |
| **Structs** | Organizar datos | Agrupar campos relacionados |
| **Formatting** | Output legible | Tablas, reportes |

---

## ✅ Checklist de Comprensión

Después de este proyecto, deberías entender:

- [ ] Qué es `std::fs` y cómo usarlo
- [ ] Diferencia entre `Ok()` y `Err()`
- [ ] Cómo funciona borrowing (`&`)
- [ ] Por qué Rust es seguro incluso sin GC
- [ ] Cómo los iteradores evitan copiar datos
- [ ] Cómo formatear strings con especificadores
- [ ] Cómo el compilador te fuerza a ser seguro

---

## 🎓 Conclusión

**Proyecto 1 te enseñó:**
- Rust interactúa con el SO de forma segura
- El compilador verifica errores en tiempo de compilación
- Type safety previene muchos bugs
- Ownership evita memory leaks

**Siguiente paso:** Proyecto 2 — Process Manager (más complejo, acceso a `/proc`)

---

**Creado:** 2026-08-25 | **Versión:** 1.0 | **Status:** ✅ Completo
