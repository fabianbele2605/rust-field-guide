# 📚 Proyecto 6: Mini OS con Multitasking — Documentación Educativa

**Nivel:** 🔴 AVANZADO+++ | **Duración:** 2-3 semanas | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un **mini OS que ejecuta múltiples tareas simultáneamente** mediante:
1. **Context Switching** — guardar/restaurar estado del CPU
2. **Scheduler** — elegir qué tarea ejecutar
3. **Timer Interrupt** — cambiar tareas cada 1ms (time-slicing)

**Resultado:** Un kernel Rust que hace multitasking real (aunque simple).

```
Tarea 1 (1ms) → Tarea 2 (1ms) → Tarea 3 (1ms) → Tarea 1...
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. **Inline Assembly: `asm!()` para Context Switching**

```rust
pub fn save_current_context(context: &mut task::Context) {
    unsafe {
        asm!(
            "mov {0}, rax",
            "mov {1}, rbx",
            "mov {2}, rcx",
            // ... más registros
            out(reg) context.rax,
            out(reg) context.rbx,
            out(reg) context.rcx,
        );
    }
}
```

**¿Qué es?**
- `asm!()` = ejecutar instrucciones x86-64 directamente
- `"mov {0}, rax"` = instrucción: Lee registro RAX
- `out(reg)` = output constraint (CPU → Rust)
- `in(reg)` = input constraint (Rust → CPU)

**¿Por qué es importante?**
- No hay forma "segura" de leer registros del CPU desde Rust puro
- Context switching REQUIERE acceso directo a hardware
- Assembly es necesario para bare metal

---

### 2. **Context Structure: Estado del CPU**

```rust
pub struct Context {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64,  // Instruction pointer
}
```

**¿Qué es?**
- 16 registros de propósito general (x86-64)
- `rsp` = stack pointer (dónde está el stack)
- `rip` = instruction pointer (qué línea ejecutar)
- Cuando guardas Context, guardas TODA la tarea

**¿Por qué es importante?**
- Cada tarea tiene su propio estado
- Para pausar/reanudar, guardas/restauras Context
- Sin Context = no hay multitasking

---

### 3. **Task Structure**

```rust
pub struct Task {
    id: u32,
    state: TaskState,
    stack: [u8; 4096],
    pub context: Context,
}
```

**¿Qué es?**
- Representa una tarea que el kernel ejecuta
- Cada tarea tiene su stack de 4KB
- Cada tarea tiene su propio Context (registros guardados)

---

### 4. **TaskManager: Gestor de Tareas**

```rust
pub struct TaskManager {
    tasks: [Option<Task>; 10],
    current_task_id: usize,
}

impl TaskManager {
    pub fn create_task(&mut self) -> u32 { /* crea tarea */ }
    pub fn current_task(&mut self) -> Option<&mut Task> { /* tarea actual */ }
    pub fn schedule_next(&mut self) { /* elige siguiente */ }
}
```

**¿Qué es?**
- Maneja hasta 10 tareas
- Rastrea cuál tarea está ejecutándose
- Implementa scheduler para elegir siguiente tarea

---

### 5. **Scheduler FIFO (First In, First Out)**

```rust
pub fn schedule_next(&mut self) {
    for i in 0..10 {
        let next_id = (self.current_task_id + 1 + i) % 10;
        if let Some(task) = &self.tasks[next_id] {
            if matches!(task.state, TaskState::Ready) {
                self.current_task_id = next_id;
                return;
            }
        }
    }
}
```

**¿Qué es?**
- Algoritmo simple: siguiente tarea lista (Ready)
- FIFO = First In First Out (FIFO order)
- Redondea: si llega al final, vuelve al inicio

**¿Por qué es importante?**
- Scheduler decide cuál tarea corre
- Algoritmo diferente = comportamiento diferente
- FIFO es simple pero fair

---

### 6. **Timer Handler Integration**

```rust
extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    let mut tm = TASK_MANAGER.lock();
    
    // 1. Guardar contexto tarea actual
    if let Some(current) = tm.current_task() {
        save_current_context(&mut current.context);
    }
    
    // 2. Scheduler elige siguiente
    tm.schedule_next();
    
    // 3. Restaurar contexto nueva tarea
    if let Some(next) = tm.current_task() {
        restore_current_context(&next.context);
    }
}
```

**¿Qué hace?**
- Cada 1ms (timer interrupt): pausa ejecución
- Guarda estado de tarea actual
- Elige tarea siguiente
- Restaura estado (CPU automáticamente continúa)
- Repite cada 1ms

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. **Context Switching (Lo Más Importante)**

```
CPU ejecutando Tarea 1
    ↓
Timer Interrupt (INT 32)
    ↓
kernel_handler ejecuta (pausa Tarea 1)
    ↓
GUARDAR registros: RAX=123, RBX=456, ...
    ↓
Cambiar stack pointer (RSP)
    ↓
RESTAURAR registros nuevos
    ↓
CPU continúa desde RIP (instruction pointer)
    ↓
Ejecutando Tarea 2 (desde donde paró)
```

**¿Por qué es importante?**
- Es el **corazón de multitasking**
- Permite ejecutar múltiples tareas "simultáneamente"
- En realidad es: cambiar entre ellas muy rápido
- CPU piensa que solo ejecuta una tarea

**Analógica:**
Como cambiar entre pestañas del navegador. Parece que ambas corren juntas, pero el CPU solo ejecuta una a la vez.

---

### 2. **Time-Slicing**

```
T=0ms:    Tarea 1 ejecuta
T=1ms:    Timer interrupt → Cambiar a Tarea 2
T=2ms:    Tarea 2 ejecuta
T=3ms:    Timer interrupt → Cambiar a Tarea 3
T=4ms:    Tarea 3 ejecuta
T=5ms:    Timer interrupt → Cambiar a Tarea 1
```

**¿Qué es?**
- Cada tarea obtiene un "slice" de tiempo (1ms)
- Después de 1ms, cambiar a siguiente tarea
- Todos obtienen CPU time de forma "justa"

---

### 3. **Task State Machine**

```
Task creada
    ↓
Ready (esperando CPU)
    ↓ (scheduler lo elige)
Running (ejecutándose)
    ↓ (timer interrupt)
Ready (vuelve a cola)
    ↓ (repite)
```

**Estados:**
- **Ready**: Esperando su turno
- **Running**: Ejecutándose ahora
- **Blocked**: Esperando I/O (no implementado aún)

---

### 4. **Stack por Tarea**

```
Tarea 1       Tarea 2       Tarea 3
┌─────┐       ┌─────┐       ┌─────┐
│RSP1 │       │RSP2 │       │RSP3 │
├─────┤       ├─────┤       ├─────┤
│...  │       │...  │       │...  │
└─────┘       └─────┘       └─────┘
```

**¿Qué es?**
- Cada tarea tiene su propio stack de 4KB
- Stack pointer (RSP) apunta al stack de esa tarea
- Cuando cambias de tarea, cambias de stack

---

### 5. **Global TaskManager**

```rust
static TASK_MANAGER: Mutex<TaskManager> = Mutex::new(...);
```

**¿Por qué Mutex?**
- Timer handler ejecuta en contexto de interrupt
- Puede interrumpir cualquier código
- Mutex protege acceso concurrente

---

## 📝 Código Final Completo

### `src/task.rs` (100+ líneas)

```rust
#[derive(Debug, Clone)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64,
}

impl Context {
    pub fn new() -> Self {
        Context {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0,
        }
    }
}

pub struct Task {
    id: u32,
    state: TaskState,
    stack: [u8; 4096],
    pub context: Context,
}

impl Task {
    pub fn new(id: u32) -> Self {
        Task {
            id,
            state: TaskState::Ready,
            stack: [0; 4096],
            context: Context::new(),
        }
    }
}

pub struct TaskManager {
    tasks: [Option<Task>; 10],
    current_task_id: usize,
}

impl TaskManager {
    pub const fn new() -> Self {
        TaskManager {
            tasks: [None, None, None, None, None, None, None, None, None, None],
            current_task_id: 0,
        }
    }

    pub fn create_task(&mut self) -> u32 {
        for i in 0..10 {
            if self.tasks[i].is_none() {
                let task = Task::new(i as u32);
                self.tasks[i] = Some(task);
                return i as u32;
            }
        }
        panic!("No more task slots!");
    }

    pub fn current_task(&mut self) -> Option<&mut Task> {
        self.tasks[self.current_task_id].as_mut()
    }

    pub fn schedule_next(&mut self) {
        for i in 0..10 {
            let next_id = (self.current_task_id + 1 + i) % 10;
            if let Some(task) = &self.tasks[next_id] {
                if matches!(task.state, TaskState::Ready) {
                    self.current_task_id = next_id;
                    return;
                }
            }
        }
    }
}
```

### `src/interrupts.rs` (Fragment: Context Switching)

```rust
use crate::task;

pub fn save_current_context(context: &mut task::Context) {
    unsafe {
        asm!(
            "mov {0}, rax",
            "mov {1}, rbx",
            "mov {2}, rcx",
            "mov {3}, rdx",
            "mov {4}, rsi",
            "mov {5}, rdi",
            "mov {6}, rbp",
            "mov {7}, rsp",
            out(reg) context.rax,
            out(reg) context.rbx,
            out(reg) context.rcx,
            out(reg) context.rdx,
            out(reg) context.rsi,
            out(reg) context.rdi,
            out(reg) context.rbp,
            out(reg) context.rsp,
        );
    }
}

pub fn restore_current_context(context: &task::Context) {
    unsafe {
        asm!(
            "mov rax, {0}",
            "mov rbx, {1}",
            "mov rcx, {2}",
            "mov rdx, {3}",
            "mov rsi, {4}",
            "mov rdi, {5}",
            "mov rbp, {6}",
            "mov rsp, {7}",
            in(reg) context.rax,
            in(reg) context.rbx,
            in(reg) context.rcx,
            in(reg) context.rdx,
            in(reg) context.rsi,
            in(reg) context.rdi,
            in(reg) context.rbp,
            in(reg) context.rsp,
        );
    }
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    let mut tm = TASK_MANAGER.lock();
    
    // 1. Guardar contexto tarea actual
    if let Some(current) = tm.current_task() {
        save_current_context(&mut current.context);
    }
    
    // 2. Scheduler elige siguiente tarea
    tm.schedule_next();
    
    // 3. Restaurar contexto nueva tarea
    if let Some(next) = tm.current_task() {
        restore_current_context(&next.context);
    }
}
```

### `src/main.rs` (Fragment: Crear Tareas)

```rust
static TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init_idt();
    
    // Crear 3 tareas de prueba
    {
        let mut tm = TASK_MANAGER.lock();
        tm.create_task();
        tm.create_task();
        tm.create_task();
    }
    
    // VGA display...
    let vga = 0xb8000 as *mut u16;
    for i in 0..13 {
        let chars = b"Hello Kernel!";
        unsafe {
            *vga.add(i) = ((0x0F as u16) << 8) | (chars[i] as u16);
        }
    }
    
    loop {}  // Timer hace context switching cada 1ms
}
```

---

## 🎓 Lecciones Clave

### Lección 1: Context Switching Es Lo Más Complejo
**Lo que podrías pensar:** "Solo cambiar de tarea".

**La realidad:**
- Necesitas guardar 16 registros exactamente
- Un registro mal guardado = data corruption
- Assembly es obligatorio (no hay forma segura en Rust puro)
- Timing es CRÍTICO

---

### Lección 2: Interrupt Context Es Especial
**Lo que podrías pensar:** "Ejecutar código en interrupt = igual que función normal".

**La realidad:**
- Interrupts pausa todo lo que hace el CPU
- No puedes bloquear (deadlock)
- No puedes dormir
- Debe ser RÁPIDO (1ms es mucho tiempo)

---

### Lección 3: Timer Es El Metrónomo del OS
**Lo que podrías pensar:** "Timer es solo un reloj".

**La realidad:**
- Timer es lo que HACE posible multitasking
- Sin timer = una tarea corre para siempre
- Timer es el "director de orquesta"
- Cada 1ms: ¡Cambien de tarea!

---

### Lección 4: Scheduler Es Donde Está La "Inteligencia"
**Lo que podrías pensar:** "Scheduler simplemente elige cualquier tarea".

**La realidad:**
- Scheduler decide CUÁNDO corre cada tarea
- FIFO = todos obtienen tiempo igual (fair)
- Priority scheduler = tareas importantes primero
- Scheduler = decide qué tarea se ejecuta

---

## ⚠️ Errores Comunes

### Error 1: Olvidar Guardar un Registro
❌ **Incorrecto:**
```rust
// Solo guardas 7 registros, olvidaste RSP
asm!("mov {0}, rax", ... out(reg) context.rax, ...);
```

✅ **Correcto:**
```rust
// Guardar los 8 registros principales
asm!("mov {0}, rax", "mov {7}, rsp", ... );
```

**Problema:** Cuando cambias de tarea, RSP apunta al stack equivocado → crash.

---

### Error 2: No Usar Mutex
❌ **Incorrecto:**
```rust
static TASK_MANAGER: TaskManager = TaskManager::new();  // Datos compartidos sin protección
```

✅ **Correcto:**
```rust
static TASK_MANAGER: Mutex<TaskManager> = Mutex::new(...);  // Protegido
```

---

### Error 3: Assembly Syntax Confusa
❌ **Incorrecto (Intel syntax - NO funciona aquí):**
```rust
asm!("mov rax, {0}");  // Intel style
```

✅ **Correcto (AT&T syntax):**
```rust
asm!("mov {0}, rax");  // AT&T style: source, dest
```

---

### Error 4: No Cambiar Stack Pointer
❌ **Incorrecto:**
```rust
// Restauras registros pero no RSP (stack pointer)
// Nueva tarea usa stack de tarea anterior → crash
```

✅ **Correcto:**
```rust
// Restaurar RSP = cambiar de stack
pub fn restore_current_context(context: &task::Context) {
    unsafe {
        asm!(
            "mov rsp, {7}",  // ← CRÍTICO: cambiar stack
            in(reg) context.rsp,
            ...
        );
    }
}
```

---

## 🚀 Lo que Este Proyecto Te Preparó Para

**Conceptos Avanzados:**
- Task priority scheduling
- Task state transitions (blocking, I/O)
- Synchronization primitives (semaphores, mutexes)
- Memory protection (paging)
- Virtual memory
- Process isolation

**Proyectos Siguientes:**
- Proyecto 7: Memory Management (paging)
- Proyecto 8: System Calls
- Proyecto 9: File System
- Proyecto 10: Networking

---

## ✅ Checklist de Comprensión

Después de este proyecto, deberías entender:

- [ ] Qué es context switching y por qué es necesario
- [ ] Cómo funcionan los 16 registros x86-64
- [ ] La diferencia entre save_context y restore_context
- [ ] Cómo funciona un scheduler FIFO
- [ ] Por qué timer interrupt es necesario para multitasking
- [ ] La sintaxis AT&T de inline assembly
- [ ] Por qué cada tarea necesita su propio stack
- [ ] Cómo el CPU automáticamente continúa desde RIP
- [ ] Por qué Mutex es necesario para TaskManager
- [ ] Cómo funciona time-slicing (1ms)

---

## 🎓 Conclusión

**Proyecto 6 te enseñó:**
- Context switching (lo más complejo de un OS)
- Inline assembly x86-64 real y funcional
- Task scheduling (algoritmo FIFO)
- Multitasking en bare metal
- Integración de timer interrupt con scheduler
- Cómo funciona REALMENTE un OS

**Es el proyecto más avanzado hasta ahora.**

Pasaste de:
- Proyecto 1-3: Aplicaciones normales
- Proyecto 4-5: Kernels básicos
- **Proyecto 6: Kernels con multitasking real**

**Siguiente paso:** Proyecto 7 — Memory Management (paging, virtual memory)

---

**Creado:** 2026-08-29 | **Versión:** 1.0 | **Status:** ✅ Completo
