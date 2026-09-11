# 📚 Proyecto 3: Mini Shell — Documentación Educativa

**Nivel:** 🔴 AVANZADO | **Duración:** 1-2 semanas | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un **shell interactivo** que permite ejecutar comandos del sistema, cambiar directorios y acceder a built-ins como `help` y `clear`.

**Ejemplo de uso:**
```bash
$ cargo run

mysh> ls
Cargo.toml  Cargo.lock  src  target

mysh> cd src

mysh> pwd
/home/usuario/proyecto/src

mysh> cat main.rs
[contenido del archivo]

mysh> help
=== Mini Shell - Comandos Disponibles ===
...

mysh> exit
Saliendo...
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. **I/O Interactivo: stdin/stdout**

```rust
use std::io::{self, Write};

print!("mysh> ");
io::stdout().flush().unwrap();

let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

**¿Qué es?**
- `io::stdout()` = stream de salida estándar
- `print!()` = imprime SIN newline
- `.flush()` = obliga a mostrar output inmediatamente
- `io::stdin()` = stream de entrada estándar
- `read_line()` = lee una línea del usuario

**Por qué es importante:**
- Interactividad: programas que responden al usuario
- Control fino: cuándo mostrar output
- Blocking: `read_line()` espera input del usuario

---

### 2. **Procesos Hijo: std::process::Command**

```rust
use std::process::Command;

match Command::new(comando)
    .args(argumentos)
    .spawn()
{
    Ok(mut child) => {
        let _ = child.wait();
    }
    Err(e) => {
        println!("Error: {}", e);
    }
}
```

**¿Qué es?**
- `Command::new()` = crear proceso
- `.args()` = pasar argumentos
- `.spawn()` = ejecutar (devuelve `Result`)
- `child.wait()` = esperar a que termine
- `Err(e)` = manejo de errores (comando no encontrado)

**Por qué es importante:**
- Rust puede ejecutar cualquier programa
- Type safety: errores son valores
- Control de procesos hijo

---

### 3. **Strings y Parsing**

```rust
fn parsear_comando(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
```

**¿Qué es?**
- `split_whitespace()` = divide por espacios
- `.collect()` = reúne en vector

**Transformación:**
```
Input:  "cd src"
Output: ["cd", "src"]

Input:  "ls -la /tmp"
Output: ["ls", "-la", "/tmp"]
```

**Por qué es importante:**
- Parsing = convertir entrada a datos estructurados
- Eficiente: no copia strings, usa referencias
- Funcional: composition de iteradores

---

### 4. **Match Expression (Pattern Matching)**

```rust
match comando {
    "exit" => { std::process::exit(0); }
    "cd" => { env::set_current_dir(argumentos[0])?; }
    "help" => { mostrar_help(); }
    "clear" => { ejecutar_comando("clear", &[]); }
    _ => false, // default case
}
```

**¿Qué es?**
- `match` = analiza todos los casos
- `"exit"`, `"cd"`, etc = patrones
- `_` = "cualquier otro caso"
- Exhaustivo: el compilador verifica que cubres TODOS los casos

**Por qué es importante:**
- Seguro: no olvidas casos
- Expresivo: código legible
- Performante: sin overhead

---

### 5. **Mutable Strings**

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

**¿Qué es?**
- `let mut` = variable que puede cambiar
- `String::new()` = string vacío
- `&mut input` = préstamo mutable (la función puede modificar)

**Por qué es importante:**
- `read_line()` agrega al string existente
- Necesita permisos para modificar
- Ownership: solo una referencia mutable a la vez

---

### 6. **Environment Variables: env Module**

```rust
use std::env;

match env::set_current_dir(argumentos[0]) {
    Ok(_) => println!("OK"),
    Err(e) => println!("Error: {}", e),
}
```

**¿Qué es?**
- `env::set_current_dir()` = cambiar directorio del proceso
- Afecta a TODO lo que el proceso haga después
- Los procesos hijo heredan este directorio

**Por qué es importante:**
- Integración con SO
- Estado del programa (directorio actual)
- Procesos hijo respetan cambios

---

### 7. **Closures y Higher-Order Functions**

```rust
items.sort_by_key(|e| {
    // closure: función anónima
    entry.file_name().to_string_lossy().to_string()
})
```

**¿Qué es?**
- Closure = función sin nombre definida in-place
- `|e| { ... }` = parámetro + cuerpo
- Captura variables del scope

**Por qué es importante:**
- Conciso: código en el lugar donde se usa
- Funcional: pass functions as values
- Elegante: evita funciones temporales

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. **Procesos e IPC (Inter-Process Communication)**

```
Tu Shell (proceso padre)
    ├── spawn() → ls (proceso hijo)
    │   └── ls ejecuta y termina
    ├── spawn() → cat (proceso hijo)
    │   └── cat ejecuta y termina
```

**¿Qué es?**
- Cada comando es un **proceso hijo**
- El shell espera a que termine
- El shell sigue ejecutándose

**Por qué es importante:**
- El shell es un "program launcher"
- Debe coordinar múltiples procesos
- Manejo de ciclo de vida

---

### 2. **Working Directory (Directorio Actual)**

```
Cada proceso tiene un "working directory"

/home/usuario/proyecto/
    ├── src/
    ├── target/
    └── Cargo.toml

Si haces "cd src", el directorio actual CAMBIA
Cualquier comando relativo (./archivo) es relativo a ese dir
```

**¿Qué es?**
- Todo proceso tiene un directorio "actual"
- Rutas relativas son relativas a ese directorio
- `cd` cambia el directorio del proceso

**Por qué es importante:**
- Necesario para navegación
- Los procesos hijo heredan el directorio actual
- Fundamental en shells

---

### 3. **Exit Codes (Códigos de Salida)**

```rust
std::process::exit(0);  // 0 = éxito
                        // 1-255 = error
```

**¿Qué es?**
- Todo proceso devuelve un número al terminar
- 0 = éxito
- Otros números = error (significado depende del programa)

**Por qué es importante:**
- El shell puede verificar si comando falló
- Scripts usan exit codes para tomar decisiones
- Cadenas de comandos (`&&`, `||`)

---

### 4. **Signals y Process Termination**

```
Cuando presionas Ctrl+C:
1. Terminal envía SIGINT al proceso
2. El proceso recibe la señal
3. El proceso termina (o maneja la señal)
```

**¿Qué es?**
- Signals = mensajes del SO al proceso
- SIGINT (Ctrl+C), SIGTERM, SIGKILL
- Permiten terminar procesos gracefully

**Por qué es importante:**
- Control de procesos
- Cleanup de recursos
- Terminación ordenada

---

## 📝 Código Final (Completo)

```rust
use std::io::{self, Write};
use std::process::Command;
use std::env;

fn parsear_comando(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn ejecutar_comando(comando: &str, argumentos: &[&str]) {
    match Command::new(comando)
        .args(argumentos)
        .spawn()
    {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            println!("Error: comando '{}' no encontrado ({})", comando, e);
        }
    }
}

fn mostrar_help() {
    println!("\n=== Mini Shell - Comandos Disponibles ===");
    println!("help          - Mostrar esta ayuda");
    println!("clear         - Limpiar pantalla");
    println!("cd <ruta>     - Cambiar directorio");
    println!("pwd           - Mostrar directorio actual");
    println!("exit          - Salir del shell");
    println!("\nPuedes ejecutar cualquier comando del sistema:");
    println!("ls, cat, echo, mkdir, rm, etc.\n");
}

fn procesar_builtin(comando: &str, argumentos: &[&str]) -> bool {
    match comando {
        "exit" => {
            println!("Saliendo...");
            std::process::exit(0);
        }
        "cd" => {
            if argumentos.is_empty() {
                println!("cd: se requiere una ruta");
                return true;
            }
            match env::set_current_dir(argumentos[0]) {
                Ok(_) => true,
                Err(e) => {
                    println!("Error: no se pudo cambiar directorio ({})", e);
                    true
                }
            }
        }
        "help" => {
            mostrar_help();
            true
        }
        "clear" => {
            ejecutar_comando("clear", &[]);
            true
        }
        _ => false,
    }
}

fn main() {
    loop {
        // Mostrar prompt
        print!("mysh> ");
        io::stdout().flush().unwrap();
        
        // Leer línea del usuario
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Parsear en comando y argumentos
        let partes = parsear_comando(input.trim());
        
        // Si entrada vacía, continuar
        if partes.is_empty() {
            continue;
        }
        
        let comando = partes[0];
        let argumentos = &partes[1..];
        
        // Intentar comando built-in, si no, ejecutar externo
        if !procesar_builtin(comando, argumentos) {
            ejecutar_comando(comando, argumentos);
        }
    }
}
```

---

## 🎓 Lecciones Clave

### Lección 1: Procesos Hijo Son Independientes
**Lo que podrías pensar:** El shell controla todo lo que hace el proceso hijo.

**La realidad de Rust:** El proceso hijo es independiente. El shell solo lo ejecuta y espera.

```rust
Command::new("ls").spawn()  // ls corre en su propio espacio
child.wait()                // shell espera a que termine
```

---

### Lección 2: `.wait()` Es Blocking
**Lo que podrías pensar:** Puedo ejecutar múltiples comandos en paralelo.

**La realidad de Rust:** `.wait()` bloquea hasta que termina.

```rust
child.wait();  // blocking: esperar aquí hasta que ls termine
// solo después continúa
```

Para paralelismo, necesitarías `async` o threads.

---

### Lección 3: El Compilador Verifica Todos los Casos
**Lo que podrías pensar:** Puedo olvidar un caso en el `match`.

**La realidad de Rust:** El compilador OBLIGA a cubrir todos los casos.

```rust
match comando {
    "exit" => { ... }
    "cd" => { ... }
    // Si no incluyes "_" aquí, error de compilación
    _ => false,  // ← default case obligatorio
}
```

---

### Lección 4: Ownership de Argumentos
**Lo que podrías pensar:** Paso argumentos al comando, se copian.

**La realidad de Rust:** Usa referencias (`&`) para no copiar.

```rust
ejecutar_comando(comando, &argumentos)  // & = préstamo
// argumentos sigue siendo válido después de la llamada
```

---

## ⚠️ Errores Comunes

### Error 1: Olvidar `.flush()` en el Prompt
❌ **Incorrecto:**
```rust
print!("mysh> ");  // sin flush
io::stdin().read_line(&mut input).unwrap();
```

**Problema:** El prompt no se muestra hasta que haya newline.

✅ **Correcto:**
```rust
print!("mysh> ");
io::stdout().flush().unwrap();  // ← obligatorio
```

---

### Error 2: No Manejar Comando No Encontrado
❌ **Incorrecto:**
```rust
Command::new(comando).spawn().unwrap();  // crash si no existe
```

✅ **Correcto:**
```rust
match Command::new(comando).spawn() {
    Ok(mut child) => { child.wait(); }
    Err(e) => { println!("Error: {}", e); }
}
```

---

### Error 3: Asumir que `cd` Afecta al Programa Padre
❌ **Incorrecto en bash:**
```bash
bash -c "cd /tmp; pwd"  # pwd muestra /tmp
# pero cuando bash termina, el shell original aún está en ~
```

✅ **Correcto en tu shell:**
```rust
env::set_current_dir(ruta)?;  // cambia el directorio del PROCESO
// todos los comandos posteriores se ejecutan desde aquí
```

---

### Error 4: Índice Sin Verificar
❌ **Incorrecto:**
```rust
let ruta = argumentos[0];  // crash si argumentos vacío
```

✅ **Correcto:**
```rust
if argumentos.is_empty() {
    println!("cd: se requiere una ruta");
    return true;
}
let ruta = argumentos[0];
```

---

## 🚀 Lo que Este Proyecto Te Preparó Para

**Conceptos Avanzados:**
- **Async Shell:** Ejecutar múltiples comandos en paralelo
- **Pipe Operator (`|`):** `cat file.txt | grep error`
- **Redirection (`>`, `<`):** `ls > output.txt`
- **Background Jobs (`&`):** `long_command &`
- **Job Control:** `fg`, `bg`, `jobs`

**Proyectos Siguientes:**
- Proyecto 4-6: Kernels — Entender cómo el SO maneja procesos
- Proyecto 7-9: Networking — Shells remotos (SSH)
- Proyecto 10-12: Distributed Systems — Shells distribuidos

---

## 📊 Comparativa: File Explorer vs Process Manager vs Mini Shell

| Aspecto | Proyecto 1 | Proyecto 2 | Proyecto 3 |
|--------|-----------|-----------|-----------|
| **Interacción** | Argumentos CLI | Información de SO | Loop interactivo |
| **Complejidad** | Básica | Media | Alta |
| **I/O** | Filesystem | Lectura de archivos | stdin/stdout |
| **Procesos** | Lee info | Lee datos | Ejecuta programas |
| **Estado** | Ninguno | Ninguno | Directorio actual |
| **Conceptos** | Ownership | Parsing | Process management |

---

## ✅ Checklist de Comprensión

Después de este proyecto, deberías entender:

- [ ] Cómo funciona un shell interactivo (REPL)
- [ ] Diferencia entre built-in y comandos externos
- [ ] Cómo usar `Command` para ejecutar procesos hijo
- [ ] Por qué `.flush()` es necesario
- [ ] Cómo `cd` cambia el directorio del proceso
- [ ] Manejo robusto de errores en procesos
- [ ] Pattern matching en Rust

---

## 🔗 Arquitectura del Mini Shell

```
┌─────────────────────────────────────┐
│         Loop Principal              │
│  (repetir infinitamente)            │
└────────────┬────────────────────────┘
             │
             ▼
    ┌───────────────────┐
    │  print!("mysh>")  │
    │  flush()          │  ← Mostrar prompt
    └─────────┬─────────┘
              │
              ▼
    ┌─────────────────────┐
    │ read_line(&mut inp) │  ← Leer del usuario
    └─────────┬───────────┘
              │
              ▼
    ┌──────────────────────┐
    │ parsear_comando()    │  ← Dividir en partes
    └─────────┬────────────┘
              │
              ▼
    ┌──────────────────────────┐
    │ procesar_builtin()?      │  ← ¿Es built-in?
    └─────────┬────────────────┘
              │
        ┌─────┴─────┐
        │           │
       SÍ           NO
        │           │
        ▼           ▼
   Ejecutar   ejecutar_comando()
   Built-in   (spawn proceso)
```

---

## 🎓 Conclusión

**Proyecto 3 te enseñó:**
- Cómo construir un programa interactivo
- Gestión de procesos hijo
- Integración con el sistema operativo
- Pattern matching y control de flujo
- Manejo robusto de errores

**Siguiente paso:** Proyecto 4 — Kernel Hello World (entra en low-level real)

---

**Creado:** 2026-08-25 | **Versión:** 1.0 | **Status:** ✅ Completo
