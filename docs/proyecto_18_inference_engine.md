# 📚 Proyecto 18: Inference Engine — Documentación Educativa

**Campo:** 🤖 AI / Machine Learning | **Nivel:** 🔴 Avanzado | **Estado:** ✅ COMPLETADO
**Carpeta:** `ia-ml/inference_engine/`

---

## 🎯 Resumen: ¿Qué Construimos?

Separamos "entrenar" de "predecir" en dos programas independientes, comunicados por un archivo binario — exactamente el pipeline real de producción en IA.

```
src/bin/train.rs   → entrena XOR (10000 épocas) → guarda pesos en model.bin
src/bin/infer.rs   → lee model.bin → reconstruye la red → predice (sin entrenar)
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Múltiples binarios en un solo proyecto (`src/bin/`)

```
src/
├── bin/
│   ├── train.rs
│   └── infer.rs
```

Cargo compila CADA archivo de `src/bin/` como un ejecutable independiente (`cargo run --bin train`, `cargo run --bin infer`), compartiendo el mismo `Cargo.toml` y dependencias — útil cuando un proyecto necesita varios programas relacionados que no tiene sentido separar en carpetas distintas.

### 2. Serialización binaria manual: `to_le_bytes()` / `from_le_bytes()`

```rust
file.write_all(&w.to_le_bytes())?;           // f64 → 8 bytes crudos
let v = f64::from_le_bytes(buf[pos..pos+8].try_into().unwrap());  // 8 bytes → f64
```

**¿Qué es "little-endian" (`le`)?** El orden en que se guardan los bytes de un número multi-byte en memoria. "Little-endian" guarda el byte MENOS significativo primero — es el orden nativo de las CPUs x86/x86-64 (las más comunes), por eso es la elección natural aquí. Mientras se escriba y se lea con el MISMO orden, el formato es consistente.

### 3. El formato binario es un contrato implícito

```rust
// Escribir: dimensiones → pesos ocultos → biases ocultos → pesos salida → bias salida
// Leer:     EXACTAMENTE el mismo orden, o los números salen sin sentido
```

No hay ningún "nombre de campo" en el archivo binario — solo bytes crudos en secuencia. El ORDEN en que escribes y lees ES la estructura de datos. Si `train.rs` y `infer.rs` alguna vez se desincronizan en ese orden, el modelo cargado sería basura (números sin sentido, pero sin ningún error de compilación que lo detecte) — por eso la verificación cuidadosa fue clave en esta sesión.

### 4. `std::time::Instant` para medir rendimiento

```rust
let start = Instant::now();
let results = net.forward_batch(&batch);
let elapsed = start.elapsed();
```

`Instant::now()` captura un punto en el tiempo (un reloj monotónico, no afectado por cambios de hora del sistema); `.elapsed()` calcula la diferencia. Es la forma estándar en Rust de medir cuánto tarda una operación, sin dependencias externas.

### 5. Closures como funciones auxiliares locales

```rust
let read_u32 = |buf: &[u8], pos: &mut usize| -> u32 {
    let v = u32::from_le_bytes(buf[*pos..*pos + 4].try_into().unwrap());
    *pos += 4;
    v
};
```

Un closure definido dentro de una función, que recibe explícitamente todo lo que necesita (`buf`, `pos`) como parámetros — útil para evitar repetir la misma lógica de "leer N bytes y avanzar el cursor" varias veces dentro de `load()`.

---

## 🖥️ Conceptos de Sistemas de IA en Producción

### 1. Por qué se separa entrenamiento de inferencia

- **Entrenar** es costoso: puede tardar minutos, horas o días; requiere el dataset completo y mucha capacidad de cómputo
- **Inferir** (predecir) debe ser rápido: en producción, un usuario espera una respuesta en milisegundos, no minutos

Por eso el entrenamiento ocurre UNA vez (o periódicamente, para reentrenar), el resultado (los pesos) se guarda, y la inferencia se ejecuta miles o millones de veces sobre ese modelo ya fijo — sin volver a tocar el proceso de entrenamiento.

### 2. Qué es realmente un "modelo" guardado en disco

Un archivo `.bin`, `.onnx`, `.safetensors`, etc. de cualquier modelo real (por más grande que sea, incluyendo LLMs) es, en su núcleo, exactamente esto: números (pesos) serializados en algún formato binario, con una estructura que el runtime que lo carga debe conocer de antemano. La escala es enorme (miles de millones de parámetros en vez de 13), pero el PRINCIPIO es idéntico al que implementamos aquí.

### 3. Batch processing: por qué se agrupan las predicciones

Procesar varios inputs juntos (`forward_batch`) permite aprovechar mejor el hardware (más trabajo por cada "viaje" a memoria/CPU/GPU) que procesar uno por uno. En GPUs reales, esta diferencia es enorme — es una de las razones por las que la inferencia de modelos grandes casi siempre se hace en lotes.

---

## 📝 Código Final

Ver `ia-ml/inference_engine/src/bin/train.rs` e `infer.rs`. Piezas clave:

```rust
// train.rs
impl NeuralNetwork {
    fn save(&self, path: &str) -> std::io::Result<()> {
        // Escribe dimensiones + todos los pesos/biases en orden fijo
    }
}

// infer.rs
impl NeuralNetwork {
    fn load(path: &str) -> std::io::Result<Self> {
        // Lee en el MISMO orden exacto en que se escribió
    }
    fn forward_batch(&self, batch: &[[f64; 2]]) -> Vec<f64> {
        batch.iter().map(|inputs| self.forward(inputs)[0]).collect()
    }
}
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `src/bin/` | Múltiples ejecutables en un proyecto | Separar train/infer sin duplicar `Cargo.toml` |
| `to_le_bytes`/`from_le_bytes` | Serialización binaria manual | Guardar/leer números como bytes crudos |
| Orden fijo de escritura/lectura | El "formato" del archivo | Sin esto, el modelo cargado sería basura |
| `Instant`/`elapsed()` | Medición de tiempo | Verificar que la inferencia es realmente rápida |
| Batch processing | Procesar varios inputs juntos | Aprovechar mejor el hardware |

---

## 🚀 Cierre del Campo AI / Machine Learning

Con este proyecto se cierra **AI / Machine Learning** (Proyectos 16, 17, 18).

**Si se retoma en el futuro:**
- Formato de modelo más robusto (versión, checksums, metadata)
- Cargar un modelo REAL preentrenado (ONNX, safetensors) con un crate existente
- Paralelizar batch processing con threads o SIMD
- Servir el modelo detrás de un servidor HTTP (combinando con el Proyecto 8)

---

**🎉 Proyecto 18: Completado. Campo AI / Machine Learning (16, 17, 18) cerrado.**
