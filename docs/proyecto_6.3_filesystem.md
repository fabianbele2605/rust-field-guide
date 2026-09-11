# 📚 Proyecto 6.3: File System Basics — Documentación Educativa

> Extensión de Proyecto 6 (Kernel Development). El próximo campo nuevo es Networking (Proyecto 7: TCP Client).

**Nivel:** 🔴 AVANZADO+++ | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un **filesystem simple en RAM**: no hay disco, los "archivos" viven como bloques de memoria dentro del propio kernel, con nombre y datos de tamaño fijo.

```
FileSystem
  ├─ files: [Option<File>; 16]
  ├─ create_file(nombre, datos)
  ├─ read_file(nombre) → &[u8]
  └─ list_files() → nombres
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Arrays de tamaño fijo en lugar de `Vec`

```rust
pub struct File {
    pub name: [u8; MAX_NAME_LEN],
    pub name_len: usize,
    pub data: [u8; MAX_FILE_SIZE],
    pub data_len: usize,
}
```

**¿Por qué no `String` o `Vec<u8>`?**
- En `no_std` sin heap real, evitamos allocaciones dinámicas
- Reservamos tamaño MÁXIMO de antemano (32 bytes para nombre, 256 para datos)
- `name_len`/`data_len` guardan cuánto del array está realmente en uso

Es el mismo patrón que ya usaste en `Task` (`stack: [u8; 4096]`) y en `TaskManager` (`tasks: [Option<Task>; 10]`).

### 2. `copy_from_slice` — copiar datos a un array existente

```rust
let n = name.len().min(MAX_NAME_LEN);
file.name[..n].copy_from_slice(&name[..n]);
```

**¿Qué hace?**
- `name.len().min(MAX_NAME_LEN)` — nunca copiar más de lo que cabe (evita overflow)
- `file.name[..n]` — un slice mutable de los primeros `n` bytes del array destino
- `.copy_from_slice(&name[..n])` — copia byte a byte desde el slice fuente

### 3. `impl Iterator<Item = &[u8]>` — devolver un iterador sin `Vec`

```rust
pub fn list_files(&self) -> impl Iterator<Item = &[u8]> {
    self.files.iter().filter_map(|slot| {
        slot.as_ref().map(|file| &file.name[..file.name_len])
    })
}
```

**¿Qué es `impl Iterator<...>`?**
- En vez de construir un `Vec` con todos los nombres (heap), devolvemos algo que **produce** los nombres uno a uno, sin allocar nada
- `filter_map` combina "filtrar" (descartar `None`) y "transformar" (`Some(file) → nombre`) en un solo paso

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. ¿Qué es un filesystem, en esencia?

Un filesystem real (ext4, NTFS, FAT) resuelve el mismo problema que este código, a mayor escala:

```
"Nombre" → dónde están los datos → cuántos bytes son
```

La diferencia es que un filesystem real:
- persiste en disco (sobrevive a un reinicio)
- soporta carpetas anidadas
- maneja fragmentación (los datos de un archivo no están todos juntos)
- tiene permisos y metadatos (fecha, dueño, etc.)

Nuestro filesystem es la versión mínima: **todo vive en RAM y se pierde al apagar**, pero el concepto (nombre → datos) es el mismo.

### 2. Por qué "tamaño fijo" en un kernel

Un kernel bare metal no tiene malloc/free sofisticado disponible desde el arranque. Reservar arrays de tamaño fijo (`[Option<File>; 16]`) evita necesitar un allocador de heap complejo — es la misma razón por la que `Task` usa `[u8; 4096]` en vez de `Vec<u8>`.

---

## 📝 Código Final

### `src/filesystem.rs`

```rust
const MAX_FILES: usize = 16;
const MAX_FILE_SIZE: usize = 256;
const MAX_NAME_LEN: usize = 32;

#[derive(Clone, Copy)]
pub struct File {
    pub name: [u8; MAX_NAME_LEN],
    pub name_len: usize,
    pub data: [u8; MAX_FILE_SIZE],
    pub data_len: usize,
}

impl File {
    pub fn empty() -> Self {
        File {
            name: [0; MAX_NAME_LEN],
            name_len: 0,
            data: [0; MAX_FILE_SIZE],
            data_len: 0,
        }
    }
}

pub struct FileSystem {
    files: [Option<File>; MAX_FILES],
}

impl FileSystem {
    pub const fn new() -> Self {
        FileSystem { files: [None; MAX_FILES] }
    }

    pub fn create_file(&mut self, name: &[u8], data: &[u8]) -> bool {
        for slot in self.files.iter_mut() {
            if slot.is_none() {
                let mut file = File::empty();
                let n = name.len().min(MAX_NAME_LEN);
                file.name[..n].copy_from_slice(&name[..n]);
                file.name_len = n;
                let d = data.len().min(MAX_FILE_SIZE);
                file.data[..d].copy_from_slice(&data[..d]);
                file.data_len = d;
                *slot = Some(file);
                return true;
            }
        }
        false
    }

    pub fn read_file(&self, name: &[u8]) -> Option<&[u8]> {
        for slot in self.files.iter() {
            if let Some(file) = slot {
                if &file.name[..file.name_len] == name {
                    return Some(&file.data[..file.data_len]);
                }
            }
        }
        None
    }

    pub fn list_files(&self) -> impl Iterator<Item = &[u8]> {
        self.files.iter().filter_map(|slot| {
            slot.as_ref().map(|file| &file.name[..file.name_len])
        })
    }
}
```

### `src/main.rs` (uso)

```rust
static FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new());

// dentro de _start():
let mut fs = FILESYSTEM.lock();
fs.create_file(b"hello.txt", b"Hola Kernel!");

if let Some(data) = fs.read_file(b"hello.txt") {
    // usar data...
}
```

---

## ⚠️ Bug encontrado y corregido en esta sesión

Al escribir el código de activación de paging (CR3/CR0) y la prueba de syscall, terminaron pegados **dentro de `panic_handler`** en vez de `_start()`. Eso significaba que:
- Nunca se ejecutaban en el flujo normal (solo si el kernel paniqueaba)
- El panic handler real quedó roto (con código que no debía estar ahí)

**Lección:** al pegar bloques de código nuevo, siempre verificar en qué función quedaron — un editor puede insertar en el punto equivocado sin avisar. Por eso pedir "verifica todos los archivos" antes de correr en QEMU fue la jugada correcta.

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| Array de tamaño fijo | `[u8; N]` en vez de `Vec<u8>` | Sin heap complejo en bare metal |
| `copy_from_slice` | Copiar bytes a un array existente | Llenar `name`/`data` de forma segura |
| `impl Iterator<Item=T>` | Retornar iterador sin `Vec` | Evita allocar, es "perezoso" |
| Mutex global | `FILESYSTEM: Mutex<FileSystem>` | Mismo patrón que `TASK_MANAGER` |

---

## 🚀 Próximos Pasos (Proyecto 10+)

- Permitir "actualizar" o "borrar" un archivo existente
- Directorios (jerarquía de nombres)
- Conectar syscalls (Proyecto 8) para que las tareas usen el filesystem
- Proyecto 10: Network Stack Basics

---

**🎉 Proyecto 9: Completado. El kernel ahora puede crear y leer "archivos" en memoria.**
