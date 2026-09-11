# 📚 Proyecto 5: Mini Kernel con Interrupts — Documentación Educativa

**Nivel:** 🔴 AVANZADO+ | **Duración:** 2-3 semanas | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un **kernel Rust con soporte de interrupts** que:
1. Captura eventos de hardware (teclado)
2. Maneja excepciones del CPU (divide by zero, page fault)
3. Asigna memoria dinámicamente (heap allocator)
4. Escribe caracteres en VGA cuando presionas teclas

**Diferencia con Proyecto 4:**
- Proyecto 4 = Kernel pasivo (solo escribe en pantalla)
- Proyecto 5 = Kernel reactivo (responde a eventos)

---

## 🦀 Conceptos Rust Aprendidos

### 1. **Inline Assembly: `asm!()` Macro**

```rust
use core::arch::asm;

let scan_code: u8;
unsafe {
    asm!("in al, 0x60", out("al") scan_code);
}
```

**¿Qué es?**
- `asm!()` = Inline assembly (código x86-64 directo)
- `"in al, 0x60"` = Instrucción x86: lee puerto 0x60 al registro AL
- `out("al") scan_code` = Guarda valor de AL en variable Rust
- `unsafe { }` = Requerido (acceso directo a hardware)

**¿Por qué es importante?**
- Rust no puede leer puertos directamente
- Necesitas assembly para comunicación de bajo nivel
- Inline assembly es la forma segura de hacerlo

**Ejemplo real:**
```rust
// Leer puerto del teclado
asm!("in al, 0x60", out("al") scan_code);

// Escribir a puerto (por ejemplo, reiniciar controlador)
asm!("out 0x20, al", in("al") 0x20);
```

---

### 2. **Interrupt Descriptor Table (IDT)**

```rust
static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt[0].set_handler_fn(divide_by_zero_handler);
    idt[33].set_handler_fn(keyboard_handler);
    idt
});

IDT.load();  // Cargar en CPU
```

**¿Qué es?**
- IDT = Tabla de manejadores de interrupts
- Cada entrada (0-255) apunta a una función
- El CPU consulta IDT cuando recibe un interrupt
- `load()` = Registra la IDT en el CPU

**¿Por qué es importante?**
- Sin IDT, el CPU no sabe qué hacer con interrupts
- IDT es el "receptor" de eventos
- Fundamental en kernels reales

**Estructura:**
```
INT 0   → divide_by_zero_handler()
INT 3   → breakpoint_handler()
INT 13  → general_protection_fault_handler()
INT 14  → page_fault_handler()
INT 33  → keyboard_handler()
```

---

### 3. **Extern "x86-interrupt" Calling Convention**

```rust
extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    // Tu código
}
```

**¿Qué es?**
- `extern "x86-interrupt"` = Convención de llamadas especial para interrupts
- Automáticamente salva/restaura registros del CPU
- Pasa stack frame con info del CPU (registros, dirección de retorno, etc.)
- Diferente a `extern "C"` (bootloader) o `extern "Rust"` (normal)

**¿Por qué es importante?**
- Interrupt handlers tienen requisitos especiales
- Rust proporciona la convención correcta
- Sin ella, se corrompen registros del CPU

---

### 4. **Global Allocator: `#[global_allocator]`**

```rust
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Asignar memoria
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Liberar memoria
    }
}
```

**¿Qué es?**
- `#[global_allocator]` = Designa el allocator para TODO el programa
- `GlobalAlloc` = Trait que define `alloc()` y `dealloc()`
- Sin esto, `Vec`, `String`, `Box` no funcionan

**¿Por qué es importante?**
- En bare metal NO HAY allocator por defecto
- Necesitas implementar uno (aunque sea simple)
- Permite usar colecciones dinámicas en kernels

---

### 5. **Cell<T> para Mutabilidad Interior**

```rust
pub struct BumpAllocator {
    heap: [u8; 0x1000],
    offset: Cell<usize>,  // ← Mutabilidad interior
}

impl BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = self.offset.get();
        self.offset.set(current + layout.size());  // Muta sin &mut
        // ...
    }
}
```

**¿Qué es?**
- `Cell<T>` = Permite mutar `&T` (sin `&mut T`)
- `get()` = Lee el valor
- `set()` = Escribe el valor
- No es thread-safe (por eso existe `Mutex`)

**¿Por qué es importante?**
- `GlobalAlloc::alloc()` recibe `&self` (no `&mut self`)
- Necesitas mutar `offset` con `&self`
- `Cell` es la solución (junto con `unsafe impl Sync`)

---

### 6. **Unsafe Impl Sync**

```rust
unsafe impl Sync for BumpAllocator {}
```

**¿Qué es?**
- `Sync` = Trait que indica "seguro compartir entre threads"
- `unsafe impl` = Le dices al compilador "confía en mí"
- Sin esto, `static ALLOCATOR: BumpAllocator` falla

**¿Por qué es importante?**
- Los `static` requieren `Sync`
- En bare metal single-core, ES seguro
- Usar `unsafe impl` correctamente es responsabilidad tuya

---

### 7. **Scan Code Tables y Conversión**

```rust
const SCAN_CODE_TABLE: [u8; 256] = [
    0, 27, b'1', b'2', b'3', // ... 256 elementos
];

let scan_code = 0x1E;  // ← Del hardware
let ascii = SCAN_CODE_TABLE[scan_code as usize];  // ← 'A'
```

**¿Qué es?**
- Scan code = Número que envía el teclado (0x1E = 'A')
- ASCII = Carácter que entendemos ('A')
- Tabla = Mapeo directo scan_code → ASCII

**¿Por qué es importante?**
- Hardware envía números, no caracteres
- Necesitas tabla de conversión
- Cada teclado/layout tiene tabla diferente

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. **Interrupts vs Exceptions**

```
INTERRUPTS (Hardware)
├── Externas (periféricos)
│   ├── Teclado (INT 33)
│   ├── Timer (INT 32)
│   └── Disco duro
└── Síncronas (responden eventos)

EXCEPTIONS (CPU)
├── Internas (CPU genera)
│   ├── Divide by zero (INT 0)
│   ├── Page fault (INT 14)
│   └── General protection fault (INT 13)
└── Síncronas (durante ejecución)
```

**Flujo de un interrupt:**
```
Usuario presiona tecla 'A'
   ↓
Teclado envía señal al CPU
   ↓
CPU pausa ejecución actual
   ↓
CPU consulta IDT[33]
   ↓
CPU salta a keyboard_handler()
   ↓
Tu código ejecuta
   ↓
CPU retorna a código original
```

**¿Por qué es importante?**
- Diferencia entre interrupts/exceptions
- Permite multitarea (pausar/reanudar código)
- Base de sistemas operativos

---

### 2. **Interrupt Descriptor Table (IDT) en Hardware**

```
IDT (cargada en CPU)
┌─────────────────────────────────┐
│ Entry 0: Divide by Zero Handler │ ← INT 0
│ Entry 1: Debug Handler          │ ← INT 1
│ Entry 2: NMI Handler            │ ← INT 2
│ Entry 3: Breakpoint Handler     │ ← INT 3
│ ...                             │
│ Entry 33: Keyboard Handler      │ ← INT 33
│ ...                             │
└─────────────────────────────────┘
     ↑
     CPU consulta aquí cuando recibe interrupt
```

**¿Qué información contiene cada entrada?**
```
┌──────────────────────────────────────────┐
│  Dirección de la función handler         │
│  Prioridad (nivel de privilegio)         │
│  Tipo (trap, interrupt, task gate)       │
│  Presente (¿existe este handler?)        │
└──────────────────────────────────────────┘
```

---

### 3. **Heap Allocator (Memory Management)**

```
Heap Memory Layout:

Inicio (0x1000)
┌──────────────────────────────┐
│  Libre (0 bytes usados)      │
│                              │
└──────────────────────────────┘
     ↑
     offset = 0

Después de Vec::new():
┌──────────────────────────────┐
│  Asignado (Vec)              │  (100 bytes)
│  Libre                       │
│                              │
└──────────────────────────────┘
     ↑
     offset = 100

Después de String::new():
┌──────────────────────────────┐
│  Asignado (Vec)              │  (100 bytes)
│  Asignado (String)           │  (50 bytes)
│  Libre                       │
│                              │
└──────────────────────────────┘
     ↑
     offset = 150
```

**¿Por qué es importante?**
- Necessario para estructuras dinámicas
- Permite flexibilidad en tamaños
- Base de memory management

---

### 4. **Bump Allocator (Estrategia Simple)**

```rust
pub struct BumpAllocator {
    heap: [u8; 0x1000],      // 4KB
    offset: Cell<usize>,      // Próxima posición libre
}

impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current_offset = self.offset.get();
        let new_offset = current_offset + layout.size();
        
        if new_offset > 0x1000 {
            panic!("Heap overflow!");
        }
        
        self.offset.set(new_offset);
        (self.heap.as_ptr() as usize + current_offset) as *mut u8
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator no desasigna
        // (solo suma offsets, nunca "recupera" memoria)
    }
}
```

**Ventajas:**
- ✅ Muy simple de implementar
- ✅ Rápido (una suma)
- ✅ Perfecto para kernels pequeños

**Desventajas:**
- ❌ Nunca libera memoria (fragmentación)
- ❌ Solo funciona si asignaciones son temporales
- ❌ No apto para programas largos

**Alternativas (más complejas):**
- Allocador de arena (zones)
- Allocador de linked-list
- Allocador de buddy system

---

### 5. **Port I/O (Lectura de Hardware)**

```
Hardware Ports (direcciones I/O)

0x60 = Teclado (datos)
0x64 = Teclado (estado)
0x20 = PIC (Programmable Interrupt Controller)
0x21 = PIC (máscara de interrupts)
```

**Lectura:**
```rust
asm!("in al, 0x60", out("al") scan_code);
// Espera en puerto 0x60, lee 8 bits al registro AL
```

**Escritura:**
```rust
asm!("out 0x20, al", in("al") 0x20);
// Envía valor a puerto 0x20
```

---

## 📝 Código Final Completo

### `src/main.rs`

```rust
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::cell::Cell;
use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

// ============= GLOBAL ALLOCATOR =============

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

pub struct BumpAllocator {
    heap: [u8; 0x1000],
    offset: Cell<usize>,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap: [0; 0x1000],
            offset: Cell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current_offset = self.offset.get();
        let new_offset = current_offset + layout.size();
        self.offset.set(new_offset);
        (self.heap.as_ptr() as usize + current_offset) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator no desasigna
    }
}

unsafe impl Sync for BumpAllocator {}

// ============= KERNEL ENTRY =============

mod interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u16;
    
    // Inicializar tabla de interrupts
    interrupts::init_idt();
    
    // Escribir "Hello Kernel!" en VGA
    for i in 0..13 {
        let chars = b"Hello Kernel!";
        unsafe {
            *vga.add(i) = ((0x0F as u16) << 8) | (chars[i] as u16);
        }
    }
    
    for i in 13..80 {
        unsafe {
            *vga.add(i) = ((0x0A as u16) << 8) | (b' ' as u16);
        }
    }
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

### `src/interrupts.rs`

```rust
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Lazy;
use core::arch::asm;
use core::cell::Cell;

// ============= SCAN CODE TABLE =============

const SCAN_CODE_TABLE: [u8; 256] = [
    // Tabla completa con 256 elementos
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', b'\x08', b'\t',
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', 0, b'a', b's',
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, b'\\', b'z', b'x', b'c', b'v',
    b'b', b'n', b'm', b',', b'.', b'/', 0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1',
    b'2', b'3', b'0', b'.', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// ============= CURSOR GLOBAL =============

static CURSOR_X: spin::Mutex<usize> = spin::Mutex::new(0);
static CURSOR_Y: spin::Mutex<usize> = spin::Mutex::new(0);

// ============= IDT =============

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt[33].set_handler_fn(keyboard_handler);
    idt[0].set_handler_fn(divide_by_zero_handler);
    idt[14].set_handler_fn(page_fault_handler);
    idt[13].set_handler_fn(general_protection_fault_handler);
    idt
});

pub fn init_idt() {
    IDT.load();
}

// ============= HANDLERS =============

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // Breakpoint handler
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let scan_code: u8;
    unsafe {
        asm!("in al, 0x60", out("al") scan_code);
    }

    let ascii = SCAN_CODE_TABLE[scan_code as usize];

    if ascii != 0 {
        let vga = 0xb8000 as *mut u16;
        let color = 0x0F;
        
        let mut x = CURSOR_X.lock();
        let mut y = CURSOR_Y.lock();

        unsafe {
            let pos = *y * 80 + *x;
            *vga.add(pos) = ((color as u16) << 8) | (ascii as u16);
        }

        *x += 1;
        if *x >= 80 {
            *x = 0;
            *y += 1;
        }
    }
}

extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: InterruptStackFrame) {
    // Divide by zero handler
}

extern "x86-interrupt" fn page_fault_handler(_stack_frame: InterruptStackFrame) {
    // Page fault handler
}

extern "x86-interrupt" fn general_protection_fault_handler(_stack_frame: InterruptStackFrame) {
    // General protection fault handler
}
```

### `Cargo.toml`

```toml
[package]
name = "kernel_hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
bootloader = "0.9"
x86_64 = "0.15"
spin = "0.9"

[[bin]]
name = "kernel_hello_world"
path = "src/main.rs"
```

---

## 🎓 Lecciones Clave

### Lección 1: Interrupts Pausan la Ejecución
**Lo que podrías pensar:** El kernel está siempre en `loop {}`, nunca hace nada.

**La realidad:**
- `loop {}` espera interrupts
- Cuando llega INT 33, CPU pausa el loop
- Ejecuta `keyboard_handler()`
- CPU retorna al loop

---

### Lección 2: Bare Metal Requiere Assembly
**Lo que podrías pensar:** Rust maneja todo en alto nivel.

**La realidad:**
- Acceso a puertos = assembly (`in`, `out`)
- Acceso a memoria especial = unsafe
- Interrupts = convención `x86-interrupt`
- No hay forma "pura" de hacerlo

---

### Lección 3: Allocator No Es Trivial
**Lo que podrías pensar:** Asignar memoria es simple.

**La realidad:**
- Necesitas implementar `GlobalAlloc`
- Necesitas `Cell<T>` para mutabilidad interior
- Necesitas `unsafe impl Sync`
- Bump allocator es el mínimo viable
- Allocadores reales son mucho más complejos

---

### Lección 4: El CPU Espera Estructura Exacta
**Lo que podrías pensar:** Los interrupts "simplemente funcionan".

**La realidad:**
- IDT debe estar en formato exacto x86-64
- Handlers deben usar convención `x86-interrupt`
- Entry points requieren `#[no_mangle]`
- Una línea mal y el kernel se cuelga

---

## ⚠️ Errores Comunes

### Error 1: Olvidar `unsafe impl Sync`
❌ **Incorrecto:**
```rust
pub struct BumpAllocator {
    heap: [u8; 0x1000],
    offset: Cell<usize>,
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = ...;  // Error: Cell no es Sync
```

✅ **Correcto:**
```rust
pub struct BumpAllocator { ... }

unsafe impl Sync for BumpAllocator {}  // ← Confía en mí

#[global_allocator]
static ALLOCATOR: BumpAllocator = ...;  // OK
```

---

### Error 2: No Usar `Cell<T>` en Allocator
❌ **Incorrecto:**
```rust
pub struct BumpAllocator {
    offset: usize,  // ← No mutable
}

unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    self.offset += layout.size();  // Error: no puede asignar a &self
}
```

✅ **Correcto:**
```rust
pub struct BumpAllocator {
    offset: Cell<usize>,  // ← Cell permite mutabilidad interior
}

unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let new_offset = self.offset.get() + layout.size();
    self.offset.set(new_offset);  // OK: set() no requiere &mut
}
```

---

### Error 3: Registrar Handler Incorrecto en IDT
❌ **Incorrecto:**
```rust
idt.keyboard.set_handler_fn(keyboard_handler);  // ¿Qué campo es "keyboard"?
```

✅ **Correcto:**
```rust
idt[33].set_handler_fn(keyboard_handler);  // INT 33 = keyboard
```

---

### Error 4: Pasar error_code Sin Soporte
❌ **Incorrecto:**
```rust
extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,  // Error: no soporta error_code
) { }

// Cuando registras:
idt[14].set_handler_fn(page_fault_handler);  // Falla en compilación
```

✅ **Correcto:**
```rust
extern "x86-interrupt" fn page_fault_handler(_stack_frame: InterruptStackFrame) { }

idt[14].set_handler_fn(page_fault_handler);  // OK
```

---

## 🚀 Lo que Este Proyecto Te Preparó Para

**Conceptos Avanzados:**
- PIC (Programmable Interrupt Controller) — inicializar interrupts de hardware
- Advanced exception handlers — guardias de memoria, traceback
- Allocadores sofisticados — linked-list, arena, buddy system
- Task scheduling — scheduler de procesos
- Paging — memoria virtual
- Multitarea — cambio de contexto

**Proyectos Siguientes:**
- Proyecto 6: Mini OS (processes, multitasking)
- Proyecto 7-9: Kernels reales (Linux-like)

---

## 📊 Comparativa: Los 5 Proyectos

| Aspecto | P1 | P2 | P3 | P4 | P5 |
|--------|----|----|----|----|-----|
| **Nivel** | Básico | Intermedio | Avanzado | Avanzado | Avanzado+ |
| **Entorno** | SO | SO | SO | Bare metal | Bare metal |
| **I/O** | Filesystem | /proc | stdin/stdout | VGA | Hardware ports |
| **Concepto** | Files | Procesos | Shell | Kernel | Interrupts |
| **Interactividad** | No | No | Sí | No | Sí (potencial) |
| **Memory** | Stack | Stack | Stack | Stack | Heap |

---

## ✅ Checklist de Comprensión

Después de este proyecto, deberías entender:

- [ ] Qué es un interrupt y cómo funciona
- [ ] Diferencia entre interrupts (HW) y exceptions (CPU)
- [ ] Cómo funciona la IDT (Interrupt Descriptor Table)
- [ ] Convención `extern "x86-interrupt"`
- [ ] Cómo leer puertos con inline assembly `asm!()`
- [ ] Scan codes vs caracteres ASCII
- [ ] Qué es un Global Allocator
- [ ] Cómo implementar `GlobalAlloc` trait
- [ ] `Cell<T>` para mutabilidad interior
- [ ] `unsafe impl Sync` y por qué es necesario
- [ ] Bump allocator (estrategia simple)
- [ ] Por qué bare metal es diferente (no hay runtime)

---

## 🔗 Comparativa: Keyboards en Diferentes Niveles

```
NIVEL 1 (Aplicación Normal)
let input = std::io::stdin().read_line(...);
    ↓
SO proporciona buffer del teclado
    ↓
Tu código lee buffer

NIVEL 2 (Bare Metal, Tu Proyecto 5)
Teclado genera INT 33
    ↓
Tu kernel_handler() ejecuta
    ↓
Lee puerto 0x60 directamente
    ↓
Escribe en VGA

NIVEL 3 (Kernel Real, Linux)
Teclado → Controlador → IRQ1 → Kernel Handler
    ↓
Kernel decodifica scan code
    ↓
Kernel envía a aplicación
    ↓
Tu aplicación lee
```

---

## 🎓 Conclusión

**Proyecto 5 te enseñó:**
- Qué pasa "debajo" cuando presionas una tecla
- Cómo funciona la reactividad en kernels
- Memory management en bare metal
- Inline assembly para hardware
- Interrupts como base de multitarea

**Siguiente paso:** Proyecto 6 — Mini OS (scheduler, multitasking)

---

## 📈 Resumen de Aprendizaje: 5 Proyectos Completados

✅ **Proyecto 1:** File Explorer — Ownership, filesystem, iterators
✅ **Proyecto 2:** Process Manager — Parsing, /proc, conversiones  
✅ **Proyecto 3:** Mini Shell — I/O interactivo, procesos hijo
✅ **Proyecto 4:** Kernel Hello World — Bare metal, VGA, boot
✅ **Proyecto 5:** Mini Kernel — Interrupts, allocator, hardware

**¡Has progresado desde aplicaciones simples hasta kernels reactivos!**

---

**Creado:** 2026-08-27 | **Versión:** 1.0 | **Status:** ✅ Completo
