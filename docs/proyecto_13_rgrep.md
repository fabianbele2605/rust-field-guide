# 📚 Proyecto 13: grep Simplificado — Documentación Educativa

**Campo:** 📦 CLI / Developer Tools | **Nivel:** 🟢 Básico | **Estado:** ✅ COMPLETADO
**Carpeta:** `cli-tools/rgrep/`

---

## 🎯 Resumen: ¿Qué Construimos?

Una versión simplificada de `grep`: busca un patrón de texto en un archivo o en TODOS los archivos de un directorio (recursivamente), mostrando archivo, número de línea y contenido.

```
rgrep [-i] <patrón> <archivo_o_directorio>

rgrep error logs/
  ↓
logs/app.log:1   connection error here
logs/test.txt:2   error: algo falló
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `fs::read_dir` + recursión — recorrer un árbol de directorios

```rust
fn search_in_dir(pattern: &str, dir: &Path, case_insensitive: bool) {
    let entries = fs::read_dir(dir)...;
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            search_in_dir(pattern, &path, case_insensitive); // recursión
        } else {
            search_in_file(pattern, &path, case_insensitive);
        }
    }
}
```

**¿Cómo funciona la recursión aquí?**
- `read_dir` lista el contenido INMEDIATO de una carpeta (no baja solo)
- Por cada entrada: si es otra carpeta, nos volvemos a llamar A NOSOTROS MISMOS con esa subcarpeta
- Si es un archivo, lo procesamos directamente
- Así se cubre CUALQUIER profundidad de subcarpetas, sin saber de antemano cuántos niveles hay

### 2. `content.lines().enumerate()` — número de línea gratis

```rust
for (i, line) in content.lines().enumerate() {
```

`.enumerate()` envuelve cualquier iterador y le agrega un índice empezando en 0. Como los humanos contamos líneas desde 1, usamos `i + 1` al imprimir.

### 3. Manejo de errores "silencioso" quando tiene sentido

```rust
let content = match fs::read_to_string(path) {
    Ok(c) => c,
    Err(_) => return, // puede ser binario, permisos, etc. — lo ignoramos
};
```

No todo error debe detener el programa o imprimirse. Si un archivo es binario (una imagen, por ejemplo) o no tenemos permiso de leerlo, simplemente lo SALTAMOS y seguimos con los demás — igual que hace el `grep` real.

### 4. Flags opcionales con `Vec::contains`

```rust
let case_insensitive = args.contains(&"-i".to_string());
let real_args: Vec<&String> = args.iter().skip(1).filter(|a| *a != "-i").collect();
```

Patrón simple para CLIs pequeñas: revisar si un flag está presente en cualquier posición, y luego construir una lista de "argumentos reales" sin ese flag ni el nombre del programa (`args[0]`).

---

## 🐛 Bug de esta sesión: usar `args` en vez de `real_args`

**Síntoma:** con `-i`, el programa no imprimía NADA, ni un error.

**Causa:**
```rust
let pattern = &args[1];              // ❌ toma "-i" como si fuera el patrón
let target = Path::new(&args[2]);    // ❌ toma "ERROR" como si fuera la ruta
```

Ya habíamos construido `real_args` (los argumentos SIN el flag `-i`), pero el código seguía usando `args` (CON el flag) para extraer `pattern` y `target`. Como `"-i"` es la posición 1 y `"ERROR"` la posición 2 en `args`, el programa buscaba el patrón `"-i"` dentro de un archivo llamado `"ERROR"` — que no existe, así que fallaba en silencio.

**Corrección:** usar `real_args[0]` y `real_args[1]` en vez de `args[1]`/`args[2]`.

**Lección:** cuando se filtra una lista para "limpiarla" (`real_args`), hay que asegurarse de usar la lista FILTRADA en el resto del código — es fácil, por costumbre, seguir usando la original.

---

## 📝 Código Final

```rust
use std::env;
use std::fs;
use std::path::Path;

fn search_in_file(pattern: &str, path: &Path, case_insensitive: bool) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    for (i, line) in content.lines().enumerate() {
        let matched = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        if matched {
            println!("{}:{}   {}", path.display(), i + 1, line);
        }
    }
}

fn search_in_dir(pattern: &str, dir: &Path, case_insensitive: bool) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            println!("Error leyendo directorio {}: {}", dir.display(), e);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if path.is_dir() {
            search_in_dir(pattern, &path, case_insensitive);
        } else {
            search_in_file(pattern, &path, case_insensitive);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Uso: rgrep [-i] <patrón> <archivo_o_directorio>");
        return;
    }

    let case_insensitive = args.contains(&"-i".to_string());
    let real_args: Vec<&String> = args.iter().skip(1).filter(|a| *a != "-i").collect();

    if real_args.len() < 2 {
        println!("Uso: rgrep [-i] <patrón> <archivo_o_directorio>");
        return;
    }

    let pattern = real_args[0];
    let target = Path::new(real_args[1]);

    if target.is_dir() {
        search_in_dir(pattern, target, case_insensitive);
    } else {
        search_in_file(pattern, target, case_insensitive);
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `fs::read_dir` + recursión | Recorrer árbol de carpetas | No se sabe de antemano cuántos niveles hay |
| `.lines().enumerate()` | Número de línea automático | Reportar exactamente dónde está la coincidencia |
| Error silencioso (`Err(_) => return`) | Ignorar y seguir | No todo fallo debe detener el programa |
| Filtrar args (`real_args`) | Separar flags de argumentos reales | Patrón común en parsing de CLI simple |

---

## 🚀 Próximos Pasos

- **Proyecto 14:** Mini Git (hashing, versionado)
- Soporte para regex real (crate `regex`)
- Mostrar contexto (líneas antes/después de la coincidencia, como `grep -A -B`)

---

**🎉 Proyecto 13: Completado. Primer proyecto del campo CLI / Developer Tools.**
