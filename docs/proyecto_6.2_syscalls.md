# 📚 Proyecto 6.2: System Calls Interface — Documentación Educativa

> Extensión de Proyecto 6 (Kernel Development).

**Nivel:** 🔴 AVANZADO+++ | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un **interfaz de system calls** que permite pedir servicios al kernel mediante una interrupción de software (`INT 0x80`), en vez de acceder hardware directamente.

```
Código (main.rs)
  ↓
mov rax, 1       ; número de syscall (WriteVGA)
mov rdi, pos     ; arg1
mov rsi, char    ; arg2
int 0x80         ; ¡dispara la syscall!
  ↓
CPU salta a syscall_handler (kernel)
  ↓
Kernel lee rax/rdi/rsi, ejecuta la acción
  ↓
Retorna a donde se llamó
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. Enum con valores explícitos (`#[repr(u64)]`)

```rust
#[repr(u64)]
pub enum SyscallNumber {
    WriteVGA = 1,
    ReadKeyboard = 2,
    Sleep = 3,
}
```

**¿Qué es?**
- `#[repr(u64)]` fuerza que el enum se represente como un `u64` en memoria
- Cada variante tiene un número fijo, para poder comparar contra el valor leído de `rax`

**¿Por qué?**
- El kernel recibe un número plano (desde un registro), no un enum de Rust
- El enum documenta el significado, aunque el `match` real compara contra literales (`1`, `2`, `3`)

---

### 2. Leer múltiples registros con un solo `asm!`

```rust
let syscall_num: u64;
let arg1: u64;
let arg2: u64;
unsafe {
    asm!(
        "mov {0}, rax",
        "mov {1}, rdi",
        "mov {2}, rsi",
        out(reg) syscall_num,
        out(reg) arg1,
        out(reg) arg2,
    );
}
```

**¿Qué hace?**
- Cada `{N}` es un placeholder que el compilador asigna a un registro libre
- `out(reg)` = "este registro sale hacia esta variable Rust"
- Tres instrucciones `mov` seguidas, cada una copia un registro fijo (rax/rdi/rsi) al registro temporal asignado

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. System Call — la frontera user/kernel

Un programa normal NO debería poder escribir directo en VGA, leer directo el teclado, etc. En un OS real, eso rompería la protección de memoria y el aislamiento entre procesos.

**Solución:** el código pide el servicio, el KERNEL lo ejecuta.

```
Tarea:  "quiero escribir 'X' en VGA"
   ↓ int 0x80 (interrupción de software)
Kernel: valida y ejecuta la escritura él mismo
   ↓
Tarea: continúa, sin haber tocado hardware directamente
```

### 2. Convención de Argumentos (ABI de syscall x86-64)

```
rax = número de syscall
rdi = arg1
rsi = arg2
rdx = arg3
rcx = arg4
r8  = arg5
r9  = arg6
Retorno: rax = resultado
```

Es la MISMA idea que una llamada a función, pero en vez de pasar por `call`, se pasa por una interrupción (`int 0x80`) porque cruza el límite user→kernel.

### 3. ¿Por qué registros y no variables globales?

Si dos tareas comparten una variable global para pasar argumentos, un context switch en medio de la operación puede sobrescribir el valor de una tarea con el de otra (condición de carrera). Los registros, en cambio, se guardan y restauran por tarea dentro de `Context` (ver Proyecto 6) — cada tarea tiene su propia copia.

### 4. INT 0x80: interrupción de software

A diferencia de INT 32 (timer, generada por hardware) o INT 33 (teclado, generada por hardware), `INT 0x80` la dispara el PROPIO programa con la instrucción `int 0x80`. El CPU la trata igual: busca el handler en el IDT y salta ahí.

---

## 📝 Código Final

### `src/interrupts.rs`

```rust
#[repr(u64)]
pub enum SyscallNumber {
    WriteVGA = 1,
    ReadKeyboard = 2,
    Sleep = 3,
}

// Registrado en el IDT:
// idt[0x80].set_handler_fn(syscall_handler);

extern "x86-interrupt" fn syscall_handler(_stack_frame: InterruptStackFrame) {
    let syscall_num: u64;
    let arg1: u64;
    let arg2: u64;
    unsafe {
        asm!(
            "mov {0}, rax",
            "mov {1}, rdi",
            "mov {2}, rsi",
            out(reg) syscall_num,
            out(reg) arg1,
            out(reg) arg2,
        );
    }

    match syscall_num {
        1 => {
            // WriteVGA(posicion, caracter)
            let vga = 0xb8000 as *mut u16;
            unsafe {
                *vga.add(arg1 as usize) = ((0x0F as u16) << 8) | (arg2 as u16);
            }
        }
        _ => {}
    }
}
```

### `src/main.rs` (invocación de prueba)

```rust
unsafe {
    asm!(
        "mov rax, 1",
        "mov rdi, 5",
        "mov rsi, {0}",
        "int 0x80",
        in(reg) 0x0F58u64,   // 'X' con atributo blanco
    );
}
```

---

## 🔄 Flujo de Ejecución

```
main.rs ejecuta asm!(..., "int 0x80")
  ↓
CPU busca IDT[0x80] → syscall_handler
  ↓
syscall_handler lee rax=1, rdi=5, rsi=0x0F58
  ↓
match 1 => escribe en VGA posición 5
  ↓
CPU retorna a la instrucción siguiente en main.rs
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|---------|
| `INT 0x80` | Interrupción de software | Cruza la frontera user→kernel |
| `SyscallNumber` | Enum de syscalls | Identifica qué servicio se pide |
| Registros (rax/rdi/rsi) | Paso de argumentos | Cada tarea tiene su copia (sin condición de carrera) |
| `syscall_handler` | Función en el kernel | Ejecuta la acción real, protegida |

---

## 🚀 Próximos Pasos (Proyecto 9+)

- Agregar más syscalls (ReadKeyboard, Sleep reales)
- Retornar resultado en `rax` al programa que llamó
- Validar argumentos (evitar que una tarea escriba fuera del VGA)
- Proyecto 9: File System Basics

---

**🎉 Proyecto 8: Completado. Ahora el kernel expone servicios protegidos vía syscalls.**
