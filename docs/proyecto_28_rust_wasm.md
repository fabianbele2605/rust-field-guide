# 📚 Proyecto 28: Rust → WASM — Documentación Educativa

**Campo:** 🌐 WebAssembly | **Nivel:** 🟢 Básico | **Estado:** ✅ COMPLETADO
**Carpeta:** `webassembly/rust_wasm/`

---

## 🎯 Resumen: ¿Qué Construimos?

Funciones Rust compiladas a WebAssembly y ejecutadas dentro de un navegador real, invocadas desde JavaScript como si fueran funciones nativas de JS.

```
src/lib.rs (Rust)
    ↓ wasm-pack build --target web
pkg/rust_wasm_bg.wasm + pkg/rust_wasm.js (glue code)
    ↓ import en index.html
JavaScript llama add(5, 7) → ejecuta código Rust compilado → devuelve 12
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. `crate-type = ["cdylib"]` — compilar como librería dinámica C-compatible

```toml
[lib]
crate-type = ["cdylib"]
```

Un binario Rust normal (`fn main()`) no tiene sentido para WASM — no hay "proceso" que arrancar, solo funciones que otro entorno (el navegador) invoca cuando quiere. `cdylib` le dice al compilador: "genera una librería con una interfaz C estable, sin nada específico de Rust expuesto" — el formato que WASM necesita para ser cargado como módulo.

### 2. `#[wasm_bindgen]` — el macro que genera el puente Rust↔JS

```rust
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Sin este atributo, la función existiría en el binario WASM pero JavaScript no sabría cómo llamarla (los tipos y la convención de llamada no coincidirían automáticamente). `#[wasm_bindgen]` genera, en tiempo de compilación, todo el código de conversión necesario — es la razón por la que `wasm-pack build` produce TANTO el `.wasm` como el `.js` (el glue code).

### 3. Conversión automática de tipos: `i32` ↔ `Number`, `String` ↔ `string`

```rust
pub fn greet(name: &str) -> String {
    format!("Hola, {}! ...", name)
}
```

```javascript
const message = greet("Fabian");  // JS ve esto como una función normal
```

`wasm-bindgen` sabe convertir automáticamente tipos simples (números, strings) entre las representaciones de memoria de Rust y JavaScript. Tipos más complejos (structs, Vec) requieren anotaciones adicionales — no cubierto aquí, pero es la extensión natural.

---

## 🖥️ Conceptos de WebAssembly

### 1. Qué es realmente un módulo WASM

WebAssembly es un formato de **bytecode binario** portable, diseñado para ejecutarse a velocidad cercana a nativa dentro de un sandbox seguro (el navegador, o runtimes independientes como Wasmtime). No es específico de Rust — C, C++, Go, AssemblyScript, entre otros, también compilan a WASM. Rust es popular para esto porque su modelo de memoria (sin garbage collector, sin runtime pesado) produce módulos WASM pequeños y rápidos.

### 2. Por qué no se puede abrir con `file://`

Los navegadores bloquean por seguridad que un módulo WASM (o incluso `fetch()` de cualquier recurso) se cargue desde el sistema de archivos local directamente — es una política CORS (Cross-Origin Resource Sharing) que trata `file://` como un origen distinto y restringido. Un servidor HTTP local (`python3 -m http.server`, o cualquier otro) resuelve esto sirviendo los archivos con las cabeceras HTTP correctas.

### 3. El flujo completo: de Rust a bits ejecutándose en el navegador

```
1. rustc compila Rust → WASM bytecode (instrucciones de una máquina virtual estandarizada)
2. wasm-bindgen genera el glue code JS que sabe llamar ese bytecode
3. El navegador descarga el .wasm, lo valida, y lo compila JIT a código máquina nativo
4. JavaScript invoca la función — la ejecución real ocurre en código nativo, no interpretado
```

Este último paso es la razón de que WASM sea tan rápido: no se interpreta como JavaScript tradicional, se compila a instrucciones de máquina real antes de ejecutarse.

---

## 🐛 Problema de esta sesión (no un bug de código)

**Síntoma:** `wasm-pack build` falló al intentar descargar `wasm-opt` (binaryen) desde GitHub.

**Causa:** `wasm-opt` es una herramienta EXTERNA (no escrita en Rust) que `wasm-pack` intenta descargar automáticamente para optimizar el tamaño del `.wasm` final — un problema de red/disponibilidad, no de nuestro código.

**Solución:** desactivar esa optimización opcional agregando en `Cargo.toml`:
```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

**Lección:** no todo error de build es un bug en el código propio — a veces es una herramienta auxiliar externa que falla por razones ajenas (red, disponibilidad del servidor). Diferenciar "mi código está mal" de "una herramienta del ecosistema falló" ahorra tiempo de debugging.

---

## 📝 Código Final

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hola, {}! Esto viene de Rust compilado a WASM.", name)
}
```

```html
<!-- index.html -->
<script type="module">
    import init, { add, greet } from './pkg/rust_wasm.js';
    async function run() {
        await init();
        const sum = add(5, 7);
        const message = greet("Fabian");
        document.getElementById('output').innerText = `add(5, 7) = ${sum}\n${message}`;
    }
    run();
</script>
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `cdylib` | Librería dinámica C-compatible | Formato que WASM necesita, no un binario con `main()` |
| `#[wasm_bindgen]` | Macro generador de glue code | Sin él, JS no sabría cómo invocar la función Rust |
| `wasm-pack build --target web` | Compilador + empaquetador | Produce el `.wasm` y el `.js` listos para usar en navegador |
| Servidor HTTP local | Necesario para cargar WASM | `file://` está bloqueado por política CORS del navegador |

---

## 🚀 Próximos Pasos

- **Proyecto 29:** Image Processing en WASM (procesar imágenes con Rust, mostrar en Canvas)
- Pasar structs/Vec entre Rust y JS (requiere anotaciones adicionales de `wasm-bindgen`)
- Medir el tamaño del `.wasm` con y sin `wasm-opt` activo

---

**🎉 Proyecto 28: Completado. Primer proyecto del campo WebAssembly.**
