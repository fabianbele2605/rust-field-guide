# 🦀 Ruta Completa de Aprendizaje: Rust Field Guide

**Versión 1.0** | Diseñada para aprender TODO el ecosistema Rust mediante proyectos prácticos

---

## 📖 Tabla de Contenidos

1. [Tu Objetivo](#objetivo)
2. [Estructura del Aprendizaje](#estructura)
3. [Los 10 Campos Principales](#campos)
4. [Metodología de Aprendizaje](#metodologia)
5. [Niveles de Competencia](#niveles)
6. [Roadmap Recomendado](#roadmap)
7. [Ecosistema Completo (30+ campos)](#ecosistema)
8. [Familias de Campos](#familias)
9. [Tu Camino Personalizado](#camino-personalizado)

---

## Tu Objetivo {#objetivo}

Aprender Rust para poder:

- ✅ **Leer** código Rust complejo de cualquier proyecto
- ✅ **Comprender** la arquitectura y diseño de sistemas en Rust
- ✅ **Modificar** proyectos existentes sin necesidad de reescribir desde cero
- ✅ **Escribir** Rust cuando sea necesario (como consecuencia natural)

### Por qué NO es memorizar sintaxis

No necesitas ser un "experto escribiendo Rust de memoria". Para este objetivo, vale mucho más:

| Habilidad | Importancia | Por qué |
|-----------|-------------|--------|
| Leer código | ⭐⭐⭐⭐⭐ | Pasarás 80% del tiempo leyendo código existente |
| Comprender arquitectura | ⭐⭐⭐⭐⭐ | Te permite entender el "por qué" de cada decisión |
| Modificar código | ⭐⭐⭐⭐ | Puedes cambiar, extender, arreglar existente |
| Escribir desde cero | ⭐⭐⭐ | Llegará naturalmente después de leer mucho |

---

## Estructura del Aprendizaje {#estructura}

### El Modelo: Rust Field Guide

Imagina recorrer los principales "campos" del ecosistema Rust como si exploraras diferentes regiones en un mapa.

**Estructura:**
- **10 campos principales** del ecosistema Rust
- **3 proyectos por campo** (básico → intermedio → avanzado)
- **Total: 30 proyectos** (pero solo necesitas algunos)

### Progresión por Niveles

| Nivel | Símbolo | Duración | Objetivo |
|-------|---------|----------|----------|
| **Básico** | 🟢 | 2-4 días | Explorar conceptos fundamentales |
| **Intermedio** | 🟡 | 4-7 días | Profundizar y combinar conceptos |
| **Avanzado** | 🔴 | 1-2 semanas | Dominar completamente el campo |

### ⚠️ La Regla de Oro

**No tienes que terminar los 30 proyectos.**

El propósito es que los primeros proyectos (niveles básicos) te permitan **descubrir qué campos te atrae realmente**. Luego puedes profundizar en esos campos específicos.

```
EXPLORACIÓN (Proyectos 1-10, básicos)
        ↓
DESCUBRIMIENTO (¿Cuáles 3-5 campos me atraen?)
        ↓
ESPECIALIZACIÓN (Profundizar a nivel avanzado)
        ↓
DOMINIO (Experto en esos campos)
```

---

## 📍 Tu Progreso Actual y Organización de Carpetas {#progreso}

**Convención:** cada campo vive en su propia carpeta al nivel de `Rust_ruta/`. Los proyectos ya completados se quedan donde están (no se mueven, para no romper cachés de compilación); los campos nuevos se organizan así desde el inicio.

| Campo | Carpeta | Proyectos | Estado |
|-------|---------|-----------|--------|
| ⚙️ System Programming | `file_explorer/`, `process_manager/`, `mini_shell/` | 1, 2, 3 | ✅ Completados |
| 🖥️ Kernel Development | `kernel_hello_world/` | 4, 5, 6, **6.1 (Paging), 6.2 (Syscalls), 6.3 (Filesystem)** | ✅ Completados |
| 🌐 Networking | `networking/` | 7 (TCP Client) ✅, 8 (HTTP Server) ✅, 9 (Reverse Proxy) ✅ | ✅ Completado |
| ☁️ Distributed Systems | `distributed-systems/` | 10 (KV Server) ✅, 11 (Distributed KV Store) ✅, 12 (Mini Distributed DB) ✅ | ✅ Completado |
| 📦 CLI / Developer Tools | `cli-tools/` | 13 (rgrep) ✅, 14 (Mini Git) ✅, 15 (Mini Compiler) ✅ | ✅ Completado |
| 🤖 AI / ML | `ia-ml/` | 16 (Tensor) ✅, 17 (Neural Network) ✅, 18 (Inference Engine) ✅ | ✅ Completado |
| 🔐 Cybersecurity | `cybersecurity/` | 19 (Port Scanner) ✅, 20 (Packet Analyzer) ✅, 21 (Mini IDS) ✅ | ✅ Completado |
| 🌐 WebAssembly | `webassembly/` | 28 (Rust→WASM) ✅, 29 (Image Processing), 30 (WASM Runtime) | 🔄 En progreso |

> **Nota sobre numeración:** 6.1/6.2/6.3 son profundizaciones dentro de Kernel Development (Paging, Syscalls, Filesystem) hechas después del Proyecto 6 — no reemplazan el "Proyecto 7: TCP Client" de Networking definido más abajo en esta guía.

---

## Los 10 Campos Principales {#campos}

### 1. ⚙️ System Programming — Tu Base Fundamental

**¿Por qué es importante?**  
System Programming es la base de TODO en Rust. Aquí aprendes ownership, el filesystem, procesos del SO, y cómo piensa Rust a nivel bajo.

#### 🟢 Proyecto 1: File Explorer CLI

**Problema a resolver:**  
Crear una herramienta que explore directorios y muestre información de archivos (similar a `ls` pero en Rust).

**Ejemplo de uso:**
```bash
$ rust-explorer .

📁 src/
  📄 main.rs (2.3 KB)
  📄 lib.rs (1.1 KB)
📄 Cargo.toml (0.8 KB)
📄 README.md (3.2 KB)
📁 target/
```

**Conceptos Rust:**
- `std::fs` para acceso al filesystem
- Manipulación de rutas (`Path`, `PathBuf`)
- Structs y enums básicos
- Manejo de errores con `Result`
- Iteradores
- Ownership y borrowing en la práctica

**Concepto clave:**  
Entender cómo Rust interactúa con el sistema operativo a través de APIs estándar seguras.

---

#### 🟡 Proyecto 2: Process Manager (mini `ps`/`top`)

**Problema a resolver:**  
Crear una herramienta que muestre procesos activos, con información de CPU, memoria y comando.

**Ejemplo de uso:**
```
PID    CPU      MEMORY    COMMAND
1204   2.3%     120MB     firefox
1321   0.4%     40MB      code
1450   0.1%     12MB      ssh
```

**Conceptos Rust:**
- Lectura del filesystem (`/proc` en Linux)
- Parsing de datos del sistema
- Iteradores y collections
- Concurrencia básica (leer múltiples procesos)
- Manejo de datos numéricos y strings
- Performance

**Concepto clave:**  
Cómo Rust se integra con APIs del kernel de forma segura y eficiente.

---

#### 🔴 Proyecto 3: Mini Shell

**Problema a resolver:**  
Construir un shell interactivo que ejecute comandos básicos.

**Ejemplo de uso:**
```bash
$ mysh
mysh> ls
mysh> cd projects
mysh> cat file.txt
mysh> ./program
mysh> exit
```

**Arquitectura:**
```
Usuario
   ↓ (input)
Parser (tokenizar y parsear comando)
   ↓
Executor (std::process::Command)
   ↓
Kernel (fork/exec)
   ↓
Resultado (output al usuario)
```

**Conceptos Rust:**
- Manejo completo de procesos hijo (`std::process`)
- Pipes y redirección I/O
- Control de flujo interactivo
- Gestión del estado del programa
- Integración sistema operativo

**Concepto clave:**  
Entender cómo el OS, los procesos, Rust y tu aplicación interactúan en sistemas reales.

---

### 2. 🖥️ Kernel / OS — Tu Rama Principal

**¿Por qué es importante?**  
Este es probablemente tu área de mayor interés. Aquí aprendes cómo funcionan realmente los sistemas operativos desde la perspectiva de Rust.

#### 🟢 Proyecto 4: Kernel "Hello World"

**Problema a resolver:**  
Escribir un kernel minimal que bootee en QEMU y escriba un mensaje.

**Pipeline de boot:**
```
BIOS/UEFI
   ↓
Bootloader (carga kernel)
   ↓
Kernel Rust (toma el control)
   ↓
"Hello Kernel" (imprime mensaje)
```

**Conceptos Rust:**
- `#![no_std]` y `#![no_main]` (Rust sin la standard library)
- Inline assembly básico
- Memory layout y linker scripts
- Boot en QEMU
- Arquitectura x86_64

**Concepto clave:**  
Entender cómo una CPU pasa el control a tu código, sin que haya un SO previo.

---

#### 🟡 Proyecto 5: Mini Kernel con Interrupts

- **Archivo:** [proyecto_5_mini_kernel_interrupts.md](proyecto_5_mini_kernel_interrupts.md)
- **Estado:** ✅ COMPLETADO

**Problema a resolver:**  
Extender el kernel anterior con soporte para interrupts, manejo de teclado, exception handlers, y memory allocator.

**Características:**
```
CPU
 ├── Interrupts (INT 33 - teclado)
 ├── Exception Handlers (INT 0, 13, 14)
 ├── Memory Allocator (BumpAllocator - 4KB heap)
 ├── Keyboard Handler (entrada del usuario)
 ├── Scan Code Table (256 elementos)
 └── Inline Assembly (lectura de puertos)
```

**Conceptos Rust:**
- Interrupt descriptor tables (IDT)
- Inline assembly (`asm!()` macro)
- Extern "x86-interrupt" calling convention
- Global Allocator trait
- Cell<T> para mutabilidad interior
- Unsafe impl Sync
- Port I/O (lectura de hardware)

**Concepto clave:**  
Construir un kernel que realmente pueda interactuar con hardware, responder a eventos, y gestionar memoria dinámica.

---

#### 🔴 Proyecto 6: Mini OS con Multitasking

- **Archivo:** [proyecto_6_mini_os_multitasking.md](proyecto_6_mini_os_multitasking.md)
- **Estado:** ✅ COMPLETADO

**Problema a resolver:**  
Construir un kernel que ejecute múltiples tareas simultáneamente usando context switching y time-slicing.

**Componentes Implementados:**
```
Kernel Multitarea
 ├── Context Switching (save/restore registros CPU)
 ├── Task Manager (max 10 tareas)
 ├── Scheduler FIFO (siguiente tarea lista)
 ├── Timer Interrupt INT 32 (cada 1ms)
 ├── 16 Registros x86-64 guardados
 └── Time-slicing (1ms por tarea)
```

**Conceptos Rust:**
- Inline assembly (asm! macro) para leer/escribir registros
- Context structure (state del CPU)
- Task structure y Task state machine
- TaskManager (gestor de tareas)
- Scheduler algorithm (FIFO)
- Mutex para sincronización en interrupt context

**Concepto Clave:**  
Context switching es lo más complejo de un OS. Entiendes cómo funciona realmente multitasking: cambiar entre tareas cada 1ms, guardando/restaurando estado del CPU.

**Concepto clave:**  
Entender cómo TODO el OS funciona: desde el boot hasta la ejecución multitarea.

---

### 3. 🌐 Networking — Una de las Mejores Áreas para Aprender Rust

**¿Por qué es importante?**  
Networking es donde Rust realmente brilla. La combinación de seguridad + concurrencia es perfecta para redes.

#### 🟢 Proyecto 7: TCP Client

**Problema a resolver:**  
Crear un cliente que se conecte a un servidor TCP y transmita mensajes.

**Ejemplo:**
```bash
$ rust-client google.com:80
Connected to google.com:80
> GET / HTTP/1.1
> Host: google.com
>
(response...)
```

**Conceptos Rust:**
- TCP sockets
- Connection handling
- Buffers y bytes
- Error handling en networking
- Timeouts y reconnection

**Concepto clave:**  
Entender cómo Rust maneja conexiones de red de forma segura.

---

#### 🟡 Proyecto 8: HTTP Server

**Problema a resolver:**  
Construir un servidor HTTP que maneje múltiples clientes simultáneamente.

**Endpoints:**
```
GET /           → Página principal
GET /users      → Lista de usuarios
GET /products   → Catálogo
POST /data      → Enviar datos
```

**Dos enfoques:**
1. **Primero:** HTTP server from scratch (sin frameworks)
2. **Luego:** Usar frameworks (`Tokio`, `Axum`)

**Conceptos Rust:**
- HTTP protocol parsing
- Multithreading vs async
- Tokio runtime (async Rust)
- Axum framework
- Routing y handlers
- Serialization (JSON)

**Concepto clave:**  
Entender por qué Rust es tan interesante para servidores backend modernos.

---

#### 🔴 Proyecto 9: Reverse Proxy

**Problema a resolver:**  
Construir un proxy que enrute solicitudes a múltiples servidores backend.

**Arquitectura:**
```
Cliente 1  \
Cliente 2  → Proxy → Server A
Cliente 3  /  \
            \ → Server B
```

**Características:**
- Load balancing (distribución de carga)
- Health checks (verificar servidores sanos)
- Connection pooling (reutilizar conexiones)
- Logging (rastreo)
- Timeouts (control de tiempo)
- Concurrent requests (manejar múltiples simultáneamente)

**Conceptos Rust:**
- Async/await avanzado
- Connection pooling patterns
- Load balancing algorithms
- Health checking
- Graceful shutdown
- Performance optimizations

**Concepto clave:**  
Aquí empiezas a entender por qué Rust es interesante para infraestructura y DevOps.

---

### 4. ☁️ Distributed Systems / Cloud

**¿Por qué es importante?**  
Distributed Systems te enseña a pensar en términos de múltiples máquinas comunicándose, coordinándose, y tolerando fallos.

#### 🟢 Proyecto 10: Key/Value Server

**Problema a resolver:**  
Crear un servidor simple que almacene y recupere datos.

**Protocolo:**
```
SET name Juan   → OK
GET name        → Juan
DEL name        → OK
GET name        → NOT FOUND
```

**Conceptos Rust:**
- TCP server básico
- Serialization (formato de datos)
- Memory storage
- Error handling
- Simple protocol design

**Concepto clave:**  
Base para entender sistemas más complejos.

---

#### 🟡 Proyecto 11: Distributed Key/Value Store

**Problema a resolver:**  
Extender el anterior para que múltiples nodos reupliquen datos entre ellos.

**Topología:**
```
Node A
  ↕ (replicación)
Node B
  ↕ (replicación)
Node C
```

**Conceptos Rust:**
- Replication (copiar datos entre nodos)
- Consensus (ponerse de acuerdo)
- Networking (comunicación entre nodos)
- Serialization
- Failure handling (qué pasa si un nodo falla)

**Concepto clave:**  
Distribuir datos de forma que no se pierdan aunque fallen máquinas.

---

#### 🔴 Proyecto 12: Mini Distributed Database

**Problema a resolver:**  
Construir una base de datos distribuida mini con replicación, leader/follower, y recuperación.

**Arquitectura:**
```
Client
   ↓
Node 1 (Leader)
 ↙    ↘
Node 2 Node 3 (Followers)
```

**Características:**
- Replication
- Leader election
- Write-ahead logs
- Persistence
- Recovery después de fallos
- Consistency guarantees

**Conceptos Rust:**
- Raft/Paxos (protocolos de consenso)
- Distributed transactions
- State machines
- Logging y recovery
- Network protocols

**Concepto clave:**  
Entender cómo construir sistemas que siguen funcionando incluso cuando fallan partes.

> **Nota:** Este proyecto puede durar meses. Es completamente normal tomar tiempo.

---

### 5. 📦 CLI / Developer Tools

**¿Por qué es importante?**  
Proyectos relativamente pequeños pero MUY educativos. Construyes herramientas que otros usan.

#### 🟢 Proyecto 13: grep Simplificado

**Problema a resolver:**  
Crear una herramienta que busque patrones en archivos (como `grep`).

**Ejemplo:**
```bash
$ rgrep "error" ./logs/

error.log:23   ERROR: Connection failed
error.log:45   ERROR: Timeout
app.log:12     error: low memory
```

**Conceptos Rust:**
- Filesystem traversal (recorrer directorios)
- String searching
- Regex patterns
- Iterators
- CLI argument parsing

**Concepto clave:**  
Herramientas simples que resuelven problemas reales.

---

#### 🟡 Proyecto 14: Mini Git

**Problema a resolver:**  
Implementar una versión simplificada de Git. No necesitas TODO Git, solo los conceptos.

**Comandos:**
```bash
mygit init              # Inicializar repo
mygit add file.txt      # Agregar archivo
mygit commit "msg"      # Guardar cambios
mygit log               # Ver historial
```

**Conceptos Rust:**
- Hashing (SHA-256 para identificar cambios)
- Filesystem storage
- Serialization (guardar estado)
- DAGs (directed acyclic graphs) para historial
- CLI design

**Concepto clave:**  
Entender cómo los VCS realmente funcionan bajo el capó.

---

#### 🔴 Proyecto 15: Mini Compiler / Interpreter

**Problema a resolver:**  
Construir un intérprete para un lenguaje toy (muy simple).

**Ejemplo de programa:**
```
let x = 10;
let y = x + 20;
print(y);        // Output: 30
```

**Pipeline:**
```
Código fuente
   ↓
Lexer (tokenizar)
   ↓
Parser (construir AST)
   ↓
AST (Abstract Syntax Tree)
   ↓
Interpreter (ejecutar)
   ↓
Resultado
```

**Conceptos Rust:**
- Tokenization
- Parsing (usando nom/pest)
- AST construction
- Pattern matching
- Recursive evaluation
- Traits y generics avanzados

**Concepto clave:**  
Uno de los MEJORES proyectos para aprender a leer código Rust complejo. Los compiladores son código bien escrito.

---

### 6. 🤖 AI / Machine Learning

**¿Por qué es importante?**  
No es para competir con Python en ML research, sino para entender cómo Rust entra en el stack de IA.

#### 🟢 Proyecto 16: Tensor Básico

**Problema a resolver:**  
Implementar una estructura de tensores con operaciones básicas.

**Conceptos:**
```
Vector: [1, 2, 3]
Matrix: [[1, 2],
         [3, 4]]
Tensor: arreglo de cualquier dimensión
```

**Operaciones:**
- Adición (+)
- Sustracción (-)
- Multiplicación (*)
- Dot product
- Transpose

**Conceptos Rust:**
- Generic data structures
- Operator overloading
- Memory layout
- Performance

**Concepto clave:**  
Base para entender ML a nivel inferior.

---

#### 🟡 Proyecto 17: Neural Network Pequeña

**Problema a resolver:**  
Construir una red neuronal simple y entrenarla.

**Arquitectura simple:**
```
Input (pixels)
  ↓
Layer 1 (weights + bias)
  ↓
ReLU (activation)
  ↓
Layer 2 (weights + bias)
  ↓
Softmax (probabilities)
  ↓
Output (predicción)
```

**Conceptos Rust:**
- Arrays y memory layout
- Álgebra lineal
- Backpropagation
- Performance
- SIMD (Single Instruction Multiple Data) para speed

**Concepto clave:**  
Entender cómo funcionan las redes neuronales desde cero.

---

#### 🔴 Proyecto 18: Inference Engine

**Problema a resolver:**  
Cargar un modelo preentrenado y hacer predicciones.

**Pipeline:**
```
Modelo preentrenado (model.bin)
   ↓
Rust Runtime
   ↓
Input (foto, texto, etc)
   ↓
Inference (forward pass)
   ↓
Output (predicción)
```

**Conceptos Rust:**
- Binary format reading
- Optimized tensor operations
- Memory efficiency
- Batch processing

**Concepto clave:**  
Esta es la parte donde Rust realmente brilla: ejecutar modelos rápido y seguro en producción.

---

### 7. 🔐 Cybersecurity

**¿Por qué es importante?**  
Rust es extremadamente bueno para seguridad: memoria segura + concurrencia.

#### 🟢 Proyecto 19: Port Scanner

**Problema a resolver:**  
Crear una herramienta que escanee puertos abiertos en un host.

**Ejemplo:**
```bash
$ scanner 192.168.1.1

22   OPEN   (SSH)
80   OPEN   (HTTP)
443  OPEN   (HTTPS)
3000 CLOSED
```

**Conceptos Rust:**
- Sockets
- Concurrency (escanear múltiples puertos en paralelo)
- Networking
- Timeouts
- Error handling

**Concepto clave:**  
Herramientas básicas de seguridad y red.

---

#### 🟡 Proyecto 20: Packet Analyzer

**Problema a resolver:**  
Capturar tráfico de red y analizarlo.

**Protocolos a parsear:**
- TCP
- UDP
- HTTP
- DNS

**Conceptos Rust:**
- Binary data parsing
- Protocol specifications
- Memory safety en parsers
- Performance

**Concepto clave:**  
Entender cómo realmente funciona el tráfico de red.

---

#### 🔴 Proyecto 21: Mini IDS (Intrusion Detection System)

**Problema a resolver:**  
Detectar patrones anormales en tráfico de red.

**Pipeline:**
```
Connection attempts
   ↓
Analyzer (aplicar reglas)
   ↓
Rules (patrones de ataque conocidos)
   ↓
Alert (notificar si se detecta)
```

**Conceptos Rust:**
- Real-time packet processing
- Pattern matching
- Concurrency bajo carga
- Logging y alertas
- Performance en producción

**Concepto clave:**  
Aquí combinamos Rust, Networking, Concurrency y Security.

---

### 8. 🔌 Embedded / IoT

**¿Por qué es importante?**  
Aquí Rust tiene una ventaja única: control de hardware + memoria segura + rendimiento.

#### 🟢 Proyecto 22: Microcontroller LED

**Problema a resolver:**  
Hacer parpadear un LED en un microcontrolador.

**Flujo:**
```
GPIO pin
   ↓
Rust code (toggle ON/OFF)
   ↓
LED (parpadea)
```

**Conceptos Rust:**
- `embedded-hal` (Hardware Abstraction Layer)
- GPIO control
- Timing
- No_std programming

**Concepto clave:**  
Rust puede controlar directamente hardware sin garbage collection ni runtime.

---

#### 🟡 Proyecto 23: Temperature Monitor

**Problema a resolver:**  
Leer un sensor de temperatura y mostrar los datos.

**Flujo:**
```
Sensor de temperatura
   ↓
Microcontroller
   ↓
Rust (procesar datos)
   ↓
Display/Network (mostrar/enviar)
```

**Conceptos Rust:**
- I2C/SPI communication
- Sensor interfacing
- Data processing
- Display interfacing

**Concepto clave:**  
Integrar múltiples componentes en un sistema coherente.

---

#### 🔴 Proyecto 24: Mini RTOS

**Problema a resolver:**  
Crear un Real-Time Operating System simple que ejecute múltiples tareas.

**Arquitectura:**
```
Task A (sensor reading)
Task B (data processing)
Task C (network transmission)
   ↓
Scheduler (decide quién corre cuándo)
   ↓
CPU (ejecuta una tarea a la vez)
```

**Conceptos Rust:**
- Task scheduling
- Interrupts
- Context switching
- Real-time constraints
- Memory safety en embedded

**Concepto clave:**  
Combina: Rust + OS + Concurrency + Hardware.

---

### 9. 🎮 Game Development

**¿Por qué es importante?**  
No es mi primera recomendación para tu objetivo, pero es excelente para aprender arquitectura y performance.

#### 🟢 Proyecto 25: 2D Game Simple

**Problema a resolver:**  
Crear un juego 2D simple (ejemplo: Snake).

**Conceptos Rust:**
- Game loops
- Rendering
- Input handling
- State management
- Graphics libraries

---

#### 🟡 Proyecto 26: ECS (Entity Component System)

**Problema a resolver:**  
Construir un pequeño Entity Component System (arquitectura común en games).

**Idea:**
```
Entity (objeto en el juego)
 ├── Position (x, y)
 ├── Velocity (dx, dy)
 ├── Health (hp)
 └── Sprite (imagen)
```

**Conceptos Rust:**
- Component design
- Trait-based architecture
- Data-oriented design
- Performance optimization

---

#### 🔴 Proyecto 27: Mini Game Engine

**Problema a resolver:**  
Construir un pequeño motor de juegos con múltiples componentes.

**Componentes:**
- Renderer (dibuja en pantalla)
- ECS system (manejo de entidades)
- Physics (gravedad, colisiones)
- Input system (teclado/mouse)
- Audio system
- Asset loading

**Concepto clave:**  
Aprender arquitectura de software a gran escala.

---

### 10. 🌐 WebAssembly

**¿Por qué es importante?**  
WASM te permite ejecutar Rust en navegadores. Combina web + Rust.

#### 🟢 Proyecto 28: Rust → WASM

**Problema a resolver:**  
Escribir una función en Rust y llamarla desde JavaScript en un navegador.

**Flujo:**
```
Browser
   ↓
JavaScript
   ↓
Rust/WASM (lógica)
   ↓
Resultado (mostrar en page)
```

**Conceptos Rust:**
- `wasm-bindgen` (puente Rust ↔ JS)
- WASM modules
- JavaScript interop

---

#### 🟡 Proyecto 29: Image Processing en WASM

**Problema a resolver:**  
Procesar imágenes en el navegador usando Rust.

**Ejemplo:**
```
Imagen (user upload)
   ↓
Rust/WASM (aplicar filtro)
   ↓
Canvas (mostrar resultado)
```

**Conceptos Rust:**
- Image libraries
- Performance in WASM
- Canvas API interaction

---

#### 🔴 Proyecto 30: WASM Runtime

**Problema a resolver:**  
Construir un runtime para ejecutar WASM binaries.

**Flujo:**
```
WASM binary (bytecode)
   ↓
Parser (decodificar)
   ↓
Validator (verificar)
   ↓
Execution (ejecutar)
   ↓
Resultado
```

**Concepto clave:**  
Muy avanzado. Entiendes cómo funciona WASM internamente.

---

## Metodología de Aprendizaje {#metodologia}

### El Flujo de Cada Proyecto

**No es:** Proyecto → Código → Memorizar

**Es:** Proyecto → Problema → Investigación → Lectura → Modificación → Comprensión

```
PROYECTO COMIENZA
        ↓
    Problema/Reto
        ↓
   Aparece un bloqueador
        ↓
    ┌────┴────┐
    ↓         ↓
 TEORÍA   DOCUMENTACIÓN
    ↓         ↓
    └────┬────┘
        ↓
     LEER código existente
        ↓
    ENTENDER cómo funciona
        ↓
   MODIFICAR pequeñas partes
        ↓
    AGREGAR nuevas features
        ↓
   PROYECTO COMPLETADO
```

### Ejemplo Real

Estás trabajando en un proyecto y encuentras:

```rust
let data = Arc::new(Mutex::new(Vec::new()));
```

No entiendes por qué existen `Arc` y `Mutex`.

**No hagas:** Leer 200 páginas de "Rust Programming Language"

**Haz:** Apunta rápidamente lo que necesitas:
- ownership
- reference counting
- Arc (Atomic Reference Counted)
- Mutex (Mutual Exclusion)
- Send / Sync

Luego vuelves al proyecto. **Ahí la teoría tiene contexto.**

---

## Niveles de Competencia {#niveles}

Hay 4 niveles progresivos de lo que puedes hacer con Rust:

### Nivel 1: Leer

**Ves código como este:**
```rust
let x = Arc::new(Mutex::new(Vec::new()));
```

**Y sabes aproximadamente:**
> "Tenemos un objeto compartido, con ownership compartido (`Arc`) y acceso sincronizado (`Mutex`)."

No necesitas recordar escribirlo de memoria. Solo comprendes la idea.

### Nivel 2: Entender

Puedes entrar en código existente y explicar:

> "Esto usa `Arc` porque varias tareas poseen una referencia al mismo objeto. Usa `Mutex` porque necesitan acceso mutable sincronizado para evitar race conditions."

### Nivel 3: Modificar

Puedes cambiar:
```rust
Vec::new()     // Cambiar a
HashMap::new() // Esto
```

O cambiar la estrategia de sincronización.

### Nivel 4: Crear

Finalmente puedes escribir `Arc<Mutex<Vec>>` desde cero en nuevo código.

### Para tu Objetivo

**Los niveles 1-3 son MUCHO más importantes inicialmente que el nivel 4.**

Escribir llegará casi automáticamente después de haber leído cientos o miles de líneas de código bien escrito.

---

## Roadmap Recomendado {#roadmap}

### Para alguien que quiere explorar TODO el ecosistema y especializarse en Systems/Kernel

**Orden recomendado:**

```
FASE 1: System Programming
        ↓
FASE 2: CLI / Tools
        ↓
FASE 3: Networking
        ↓
FASE 4: Backend (opcional)
        ↓
FASE 5: Concurrency (opcional)
        ↓
FASE 6: Distributed Systems
        ↓
FASE 7: Security
        ↓
FASE 8: Embedded
        ↓
FASE 9: Kernel / OS
        ↓
FASE 10: WASM (opcional)
```

### Por qué este orden

1. **System Programming primero:** Base fundamental
2. **CLI Tools después:** Proyectos pequeños, éxito rápido
3. **Networking después:** Construye sobre syscalls
4. **Concurrency cuando entiendas ownership:** Es más fácil
5. **Distributed Systems después:** Requiere networking + concurrency
6. **Security después:** Networking + systems = seguridad
7. **Embedded después:** Acceso a hardware, que entiendes
8. **Kernel después:** TODO lo anterior es prerequisito
9. **WASM al final:** Exploratorio, menos crítico

### Timeline Realista

| Fase | Duración | Proyectos |
|------|----------|-----------|
| System Programming | 2-4 semanas | 3 |
| CLI Tools | 2-3 semanas | 3 |
| Networking | 3-4 semanas | 3 |
| Distributed Systems | 4-6 semanas | 3 |
| Security | 2-3 semanas | 3 |
| **Total hasta especialización** | **13-20 semanas** | **15 proyectos** |
| Especialización (3-5 campos) | 12+ semanas | 9-15 |
| **TOTAL** | **25-35 semanas** | **24-30** |

---

## Ecosistema Completo (30+ Campos) {#ecosistema}

Aquí está el mapa COMPLETO del ecosistema Rust. No necesitas aprender todo, pero vale la pena saber qué existe.

| Campo | Qué Se Usa Para | Madurez | Dificultad |
|-------|-----------------|---------|-----------|
| 🖥️ Sistemas / OS / Kernel | Kernels, bootloaders, drivers, firmware | ⭐⭐⭐⭐ | 🔴 |
| ⚙️ System Programming | Herramientas de bajo nivel, runtimes | ⭐⭐⭐⭐⭐ | 🟡 |
| 🌐 Backend / Web | APIs, microservicios, servidores | ⭐⭐⭐⭐ | 🟡 |
| ☁️ Cloud / Infraestructura | Proxies, servicios cloud, orquestación | ⭐⭐⭐⭐⭐ | 🟡 |
| 🤖 IA / ML | Inferencia, runtimes, bindings | ⭐⭐⭐ | 🔴 |
| 📦 CLI / Developer Tools | CLI, compiladores, linters, formatters | ⭐⭐⭐⭐⭐ | 🟢 |
| 🌐 WebAssembly | Rust → WASM en navegadores | ⭐⭐⭐⭐⭐ | 🟡 |
| 🔗 Blockchain / Web3 | Nodos, protocolos, smart contracts | ⭐⭐⭐⭐ | 🔴 |
| 🎮 Game Development | Motores, juegos, herramientas | ⭐⭐⭐ | 🟡 |
| 📱 Embedded / IoT | Microcontroladores, firmware, RTOS | ⭐⭐⭐⭐⭐ | 🔴 |
| 🔐 Cybersecurity | Security tools, parsers, networking | ⭐⭐⭐⭐ | 🔴 |
| 🗄️ Databases | Storage engines, DB components | ⭐⭐⭐⭐ | 🔴 |
| 🌐 Networking | TCP/UDP, HTTP, DNS, proxies | ⭐⭐⭐⭐⭐ | 🟡 |
| 🧮 Computación Científica | Simulación, álgebra, procesamiento | ⭐⭐⭐ | 🔴 |
| 🖥️ Desktop Apps | Aplicaciones multiplataforma | ⭐⭐⭐ | 🟡 |
| 📡 Distributed Systems | Clusters, consenso, replicación | ⭐⭐⭐⭐ | 🔴 |
| 🔧 DevOps / SRE | Automatización, agentes, tooling | ⭐⭐⭐⭐ | 🟡 |
| 🧬 Compiladores / Lenguajes | Compiladores, intérpretes, parsers | ⭐⭐⭐⭐⭐ | 🔴 |
| 🧠 Operating Infrastructure | Hypervisors, runtimes, sandboxes | ⭐⭐⭐⭐ | 🔴 |
| 🕸️ P2P / Decentralized | DHT, redes P2P, protocolos | ⭐⭐⭐⭐ | 🔴 |
| 📡 Telecom / Protocolos | Implementación de protocolos de red | ⭐⭐⭐ | 🔴 |
| 🛰️ Robótica | Control, sensores, comunicaciones | ⭐⭐⭐ | 🔴 |
| 🚗 Automotive | ECU, sistemas embebidos, seguridad | ⭐⭐⭐⭐ | 🔴 |
| ✈️ Aerospace | Firmware, sistemas críticos | ⭐⭐⭐ | 🔴 |
| 🏭 Industrial / OT | PLC, control industrial | ⭐⭐⭐ | 🔴 |
| 🧪 Bioinformatics | Procesamiento de secuencias | ⭐⭐ | 🔴 |
| 📊 Data Engineering | Procesamiento de datos, pipelines | ⭐⭐⭐ | 🟡 |
| 🔎 Observability | Logging, tracing, métricas, agentes | ⭐⭐⭐⭐⭐ | 🟡 |
| 🔒 Cryptography | Implementaciones criptográficas | ⭐⭐⭐⭐ | 🔴 |
| 🛡️ Privacy / Sandboxing | Sandboxes, aislamiento, seguridad | ⭐⭐⭐⭐ | 🔴 |
| 🧩 Plugins / Extensions | Sistemas de plugins | ⭐⭐⭐ | 🟡 |
| 🖼️ Graphics / Rendering | GPU, rendering, Vulkan, WebGPU | ⭐⭐⭐ | 🔴 |
| 🎥 Multimedia | Audio, vídeo, codecs, streaming | ⭐⭐⭐ | 🔴 |
| 📱 Mobile | Componentes compartidos, lógica nativa | ⭐⭐ | 🟡 |
| 🖥️ GUI Frameworks | Interfaces gráficas | ⭐⭐⭐ | 🟡 |
| 🧰 Build Systems | Build tools, package managers | ⭐⭐⭐⭐⭐ | 🟡 |
| 🔄 Runtime / VM | Virtual machines, interpreters | ⭐⭐⭐⭐ | 🔴 |
| 🧱 Storage Systems | Filesystems, object storage, caching | ⭐⭐⭐⭐ | 🔴 |

---

## Familias de Campos {#familias}

Hay una diferencia entre "campos donde Rust puede usarse" y "campos donde realmente deberías aprender Rust profundamente".

Para tu objetivo específico, agrupa estos campos en familias:

### 🖥️ Familia 1: Low-Level / Systems

```
System Programming
   ├── OS / Kernel
   ├── Drivers
   ├── Embedded
   ├── Firmware
   ├── Runtime / VM
   ├── Hypervisor
   └── Storage
```

**¿Por qué?** Excelente para aprender Rust profundamente. Aquí todo es sobre performance y seguridad de memoria.

---

### 🌐 Familia 2: Networking / Infrastructure

```
Networking
   ├── Backend / Web
   ├── Proxies
   ├── Cloud
   ├── Distributed Systems
   ├── P2P / Decentralized
   ├── Protocols
   └── Observability
```

**¿Por qué?** Una de las familias más recomendadas. Rust + Concurrency + Networking = perfecto.

---

### 🧠 Familia 3: Languages / Developer Tools

```
CLI / Tools
   ├── Parsers
   ├── Compilers
   ├── Interpreters
   ├── Linters
   ├── Formatters
   ├── Language Servers
   └── Build Systems
```

**¿Por qué?** Espectacular para aprender a LEER Rust. Los compiladores tienen código bien escrito.

Aquí encontrarás: traits, generics, lifetimes, enums, iterators, macros, AST, parsing, concurrency.

---

### 🔐 Familia 4: Security

```
Cybersecurity
   ├── Network Security
   ├── Cryptography
   ├── Sandboxing
   ├── Malware Analysis
   ├── Security Tools
   └── Privacy
```

**¿Por qué?** Combina muy bien con Systems. Rust + Seguridad = un match hecho en el cielo.

---

### 🤖 Familia 5: AI / Data

```
AI
   ├── ML / Inference
   ├── AI Runtime
   ├── Data Processing
   ├── Scientific Computing
   └── Data Engineering
```

**¿Por qué?** No pondría Rust como primera opción para aprender ML, pero es excelente para infraestructura.

---

### 🎮 Familia 6: Graphics / Games

```
Graphics
   ├── Rendering
   ├── GPU
   ├── WebGPU
   ├── Game Engine
   └── Game Development
```

**¿Por qué?** Muy buena para aprender arquitectura y performance.

---

### 🔌 Familia 7: Hardware

```
Embedded
   ├── IoT
   ├── Robotics
   ├── Automotive
   ├── Aerospace
   └── Industrial / OT
```

**¿Por qué?** Rust tiene una ventaja única: control de hardware + memoria segura + rendimiento.

---

## Tu Camino Personalizado {#camino-personalizado}

### Para tu objetivo específico...

**Basado en lo que has planteado, yo recomendaría:**

No una ruta "Rust Backend Developer"

**Sino:** Ruta **"Rust Systems Engineer"** pero explorando TODO el ecosistema.

### El Núcleo

```
                    RUST
                     │
                     ▼
           SYSTEM PROGRAMMING
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
    NETWORKING   SECURITY     TOOLING
        │            │            │
        ▼            ▼            ▼
DISTRIBUTED      LOW LEVEL    COMPILERS
SYSTEMS             │
        │           ▼
        └────────► KERNEL / OS
                      │
                      ▼
                  EMBEDDED
                      │
                      ▼
                  HARDWARE
```

### Alrededor del Núcleo

Explora también (pero menos profundidad):

- Backend / Web APIs
- Cloud / Infraestructura
- Databases
- WebAssembly
- AI Infrastructure
- Graphics / Games
- Blockchain

---

## Metodología Específica por Proyecto {#metodologia-proyecto}

Cada proyecto sigue este flujo:

1. **Qué problema resuelve** (claridad en el objetivo)
2. **Arquitectura** (cómo se estructura)
3. **Conceptos Rust clave** (qué aprendes)
4. **Concepto de sistemas** (por qué es importante en sistemas)
5. **Lectura de código existente** (lee proyectos reales)
6. **Modificación pequeña** (cambia algo)
7. **Agregar una funcionalidad** (extiende)
8. **Proyecto final** (completa)

---

## Próximos Pasos

### Si estás listo para empezar:

1. **Elige tu primer proyecto:** Recomiendo **File Explorer CLI** (Proyecto 1)
2. **Monta tu ambiente:** Rust, Cargo, editor
3. **Sigue la metodología:** No memorices, entiende
4. **Documenta tu aprendizaje:** Anota conceptos

### Si tienes dudas sobre un campo:

Mira la descripción del campo que te interesa y revisa los 3 proyectos. Eso te da una idea de la dificultad progresiva.

---

## Resumen

**Tu ruta de aprendizaje es:**
- **Explorar** todos los principales campos (Proyectos 1-15)
- **Descubrir** cuáles 3-5 te atraen realmente
- **Profundizar** en esos campos hasta nivel avanzado
- **Dominar** esos campos a nivel experto

Al final, habrás:
- ✅ Explorado 30+ campos de Rust
- ✅ Escrito 30+ proyectos
- ✅ Aprendido a leer código Rust complejo
- ✅ Entendido arquitectura de sistemas reales
- ✅ Podido modificar y extender código existente
- ✅ Ser capaz de escribir Rust cuando sea necesario

---

**¡Bienvenido a tu jornada por el ecosistema de Rust!**
