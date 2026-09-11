# 📚 Proyecto 14: Mini Git — Documentación Educativa

**Campo:** 📦 CLI / Developer Tools | **Nivel:** 🟡 Intermedio | **Estado:** ✅ COMPLETADO
**Carpeta:** `cli-tools/mygit/`

---

## 🎯 Resumen: ¿Qué Construimos?

Un VCS (control de versiones) simplificado que replica la idea central de Git: **el contenido se identifica por su hash**, y los commits forman una cadena hacia atrás.

```
mygit init                    → crea .mygit/objects/ + .mygit/HEAD
mygit add archivo1.txt        → guarda contenido como objeto (nombre = SHA-256)
mygit commit "mensaje"        → crea un commit que apunta al índice + al commit anterior
mygit log                     → recorre HEAD → parent → parent → ... → "none"
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `sha2::Sha256` — hashing criptográfico real

```rust
use sha2::{Sha256, Digest};

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result) // bytes → hexadecimal
}
```

**¿Qué hace `Digest`?** Es un trait que define la interfaz común de "algo que puede hashear bytes" (`update`, `finalize`) — así múltiples algoritmos (SHA-256, SHA-1, etc.) comparten la misma forma de uso.

**`{:x}` en `format!`:** formatea un valor como hexadecimal en minúsculas — necesario porque `finalize()` devuelve bytes crudos, no un string legible.

### 2. Objetos identificados por su propio contenido (content-addressing)

```rust
let hash = hash_content(&content);
let object_path = format!(".mygit/objects/{}", hash);
fs::write(&object_path, &content).unwrap();
```

El "nombre" del archivo guardado ES el hash de lo que contiene. Consecuencia directa: **el mismo contenido siempre produce el mismo nombre** — si dos archivos tienen exactamente el mismo contenido, terminan siendo el MISMO objeto en disco (verificado en esta sesión: `archivo1.txt` y su copia `archivo2.txt` comparten hash).

### 3. Reconstruir un `Vec` filtrando y reemplazando una línea

```rust
let mut lines: Vec<String> = index_content
    .lines()
    .filter(|l| !l.starts_with(&format!("{} ", filename)))
    .map(|l| l.to_string())
    .collect();
lines.push(index_line.trim().to_string());
```

Patrón común: "quita la línea vieja de este archivo (si existía), y agrega la nueva al final" — así `add` sobre el mismo archivo actualiza su entrada en vez de duplicarla.

### 4. `strip_prefix` para parsear un formato de texto propio (otra vez)

```rust
if let Some(p) = line.strip_prefix("parent: ") {
    parent = p.to_string();
} else if let Some(m) = line.strip_prefix("message: ") {
    message = m.to_string();
}
```

Ya usaste este patrón en el Proyecto 11 (`REPLICATE `) — aquí se repite para parsear el objeto commit, reforzando el mismo mecanismo en un contexto nuevo.

---

## 🐛 Bugs de esta sesión

### 1. Paréntesis desbalanceados en un closure encadenado

```rust
.map(|l| l.to_string()      // ❌ falta cerrar to_string()
.collect();
...
index_content = lines.join("\n") + "\n");   // ❌ paréntesis sobrante
```

**Lección:** en cadenas largas de métodos (`.filter().map().collect()`), es fácil perder la cuenta de paréntesis — el compilador señala el error varias líneas después de donde realmente está, por eso conviene revisar la cadena completa, no solo la línea marcada.

### 2. `HEAD` nunca se actualizaba tras un commit

El commit se creaba y se guardaba como objeto correctamente, pero faltaba la línea `fs::write(".mygit/HEAD", &hash).unwrap();` — sin ella, `log` nunca encontraría por dónde empezar, porque `HEAD` seguía vacío. Un error fácil de cometer: "guardé el dato, pero olvidé actualizar el puntero que dice dónde está el más reciente".

---

## 🖥️ Conceptos de Control de Versiones

### 1. Git no guarda diffs, guarda snapshots

Una creencia común incorrecta es que Git guarda "los cambios" entre versiones. En realidad, cada `add`+`commit` guarda una copia COMPLETA del contenido en ese momento (como blob), identificada por su hash. Los "diffs" que ves en `git diff` se CALCULAN al momento de pedirlos, comparando dos snapshots — no se almacenan como tales.

### 2. El historial es una lista enlazada (o DAG en Git real)

Cada commit apunta a su padre. Recorrer el historial es simplemente seguir esa cadena de punteros hacia atrás, exactamente como recorrer una lista enlazada. Git real permite múltiples padres (merges), convirtiendo la estructura en un DAG (grafo acíclico dirigido) en vez de una simple lista — no lo implementamos aquí, pero es la extensión natural.

### 3. Deduplicación automática y gratuita

Como el nombre del objeto ES su hash, Git (y nuestro Mini Git) **nunca duplica contenido idéntico**, sin necesidad de lógica extra para detectarlo — es una propiedad emergente de identificar por contenido en vez de por nombre de archivo.

---

## 📝 Código Final

Ver `cli-tools/mygit/src/main.rs` completo. Estructura de comandos:

```rust
match args[1].as_str() {
    "init" => cmd_init(),
    "add" => cmd_add(&args[2]),
    "commit" => cmd_commit(&args[2]),
    "log" => cmd_log(),
    _ => println!("Comando desconocido: {}", args[1]),
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| SHA-256 (`sha2` crate) | Hash criptográfico de contenido | Identificar objetos de forma única y determinista |
| Content-addressing | Nombre del objeto = hash de su contenido | Deduplicación automática, sin lógica extra |
| `HEAD` | Puntero al último commit | Punto de entrada para recorrer el historial |
| Commit encadenado (parent) | Cada commit referencia al anterior | Forma el historial navegable |

---

## 🚀 Próximos Pasos

- **Proyecto 15:** Mini Compiler / Interpreter
- Comando `mygit checkout <hash>` (restaurar archivos desde un commit)
- Soporte real de árboles de directorios (no solo archivos sueltos)
- Merges (múltiples padres, convirtiendo la lista en un DAG real)

---

**🎉 Proyecto 14: Completado.**
