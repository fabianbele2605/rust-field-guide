# 📚 Proyecto 6.1: Virtual Memory Management (Paging) — Documentación Educativa

> Extensión de Proyecto 6 (Kernel Development), no confundir con el "Proyecto 7: TCP Client" de la guía original.

**Nivel:** 🔴 AVANZADO+++ | **Duración:** 1-2 semanas | **Estado:** ✅ COMPLETADO

---

## 🎯 Resumen: ¿Qué Construimos?

Un **sistema de memory management virtual** que traduce direcciones virtuales a físicas mediante:

1. **Page Table Structures** — Jerarquía de 4 niveles (PML4, PDPT, PD, PT)
2. **Page Mapping** — Mapear direcciones virtuales → físicas (identity mapping)
3. **MMU Activation** — Escribir CR3 y CR0 para activar paging en CPU

**Resultado:** Un kernel que usa **memoria virtual**. Cuando el CPU accede dirección virtual 0x1000, la MMU traduce automáticamente a dirección física (ej: 0x5000).

```
Sin Paging:
CPU → 0x1000 → RAM física 0x1000

Con Paging (lo que construimos):
CPU → 0x1000 (virtual) → MMU busca en page tables → RAM física 0x5000
```

---

## 🦀 Conceptos Rust Aprendidos

### 1. **Inline Assembly: Lectura de Registros del CPU**

```rust
let mut cr3: u64;
unsafe {
    asm!("mov {}, cr3", out(reg) cr3);
}
```

**¿Qué es?**
- `asm!()` = ejecutar instrucción x86-64 directamente desde Rust
- `"mov {}, cr3"` = instrucción: Lee registro CR3 (control register 3)
- `out(reg) cr3` = output constraint: escribe valor en variable Rust

**¿Por qué es importante?**
- CR3 es un **registro del CPU**, no variable Rust normal
- No hay forma segura de leerlo sin assembly
- Paging REQUIERE acceso directo a registros del CPU

**¿Cuándo usar unsafe asm?**
- Bare metal (kernels, bootloaders)
- Acceso a hardware específico
- Operaciones que no puede expresar Rust seguro

---

### 2. **Unsafe Pointers: Convertir Direcciones a Referencias**

```rust
let pml4 = unsafe { &mut *(cr3 as *mut PageTable) };
```

**Desglosemos:**
- `cr3` = dirección física en u64 (numero puro)
- `cr3 as *mut PageTable` = convierte número → puntero mutable
- `*(&mut * ...)` = dereference puntero → obtiene referencia
- `unsafe { }` = Rust dice "confío que sabes qué haces"

**¿Por qué unsafe?**
- Rust no puede verificar si cr3 apunta a memoria válida
- Rust no sabe si es realmente una PageTable
- El programador PROMETE que es correcto

**¿Qué pueden salir mal?**
```
1. cr3 no es dirección válida → segmentation fault
2. cr3 apunta a memoria no PageTable → comportamiento indefinido
3. Múltiples referencias mutables → data race
```

---

### 3. **Bitflags: Combinar Flags con OR**

```rust
paging::identity_map_page(addr);
```

Donde `identity_map_page()` usa:
```rust
PageTableFlags::PRESENT | PageTableFlags::WRITABLE
```

**¿Qué es?**
- `PageTableFlags::PRESENT` = bit 0 encendido (0000_0001)
- `PageTableFlags::WRITABLE` = bit 1 encendido (0000_0010)
- `|` = OR bitwise = combina ambos (0000_0011)

**¿Por qué?**
- Page table entries contienen múltiples flags
- Cada bit significa algo diferente (presente, escribible, etc)
- OR bitwise es la forma eficiente de combinarlos

---

### 4. **Loop Range y step_by()**

```rust
for addr in (0x400000..0x500000).step_by(0x1000) {
    paging::identity_map_page(addr);
}
```

**¿Qué hace?**
- `0x400000..0x500000` = rango desde A hasta B
- `.step_by(0x1000)` = incrementar de 4096 en 4096 (4KB)
- Mapea CADA página del kernel

**Iteración:**
```
iter 1: 0x400000
iter 2: 0x401000
iter 3: 0x402000
...
(hasta 0x500000)
```

---

## 🖥️ Conceptos de Sistemas Operativos

### 1. **Virtual Memory: Direcciones Virtuales vs Físicas**

**Sin Paging (bare metal simple):**
```
Programa:         CPU:           RAM:
addr 0x1000  →  0x1000  →  física 0x1000
```

El CPU accede directamente a la dirección física.

**Con Paging (lo que hicimos):**
```
Programa:         CPU:           MMU:            RAM:
addr 0x1000  →  0x1000 (virtual)  →  page_tables  →  0x5000 (física)
                                      consulta
```

El MMU traduce automáticamente.

**¿Por qué?**
- Protección: Proceso A no ve memoria de Proceso B
- Flexibilidad: Memoria fragmentada parece continua
- Sandbox: Kernel está aislado de tareas
- Swapping: Simular más RAM usando disco

---

### 2. **Page Table Hierarchy: 4 Niveles en x86-64**

```
Dirección Virtual (48-bit):
┌──────────┬──────────┬──────────┬──────────┬──────┐
│ PML4     │ PDPT     │ PD       │ PT       │ Offset│
│ 9 bits   │ 9 bits   │ 9 bits   │ 9 bits   │ 12 bits
└──────────┴──────────┴──────────┴──────────┴──────┘
     ↓        ↓         ↓         ↓
  PML4   →  PDPT   →  PD    →  PT  → Página física
```

**¿Por qué 4 niveles?**
- Cada nivel = 512 entradas (2^9)
- 1 nivel = 512 páginas = 2MB
- 2 niveles = 512² = 1GB
- 3 niveles = 512³ = 512GB
- 4 niveles = 512⁴ = **256TB** (suficiente para 64-bit)

**¿Cómo funciona la caminata?**

Para traducir dirección virtual `0x12345678`:

```
1. Leer CR3 → dirección de PML4
2. Extraer índice PML4 de 0x12345678 (bits 39-47) → idx = 0x092
3. PML4[92] → contiene dirección de PDPT
4. Leer PDPT usando esa dirección
5. Extraer índice PDPT (bits 30-38) → idx = 0x123
6. PDPT[123] → dirección de PD
7. Leer PD usando esa dirección
8. Extraer índice PD (bits 21-29) → idx = 0x045
9. PD[45] → dirección de PT
10. Leer PT usando esa dirección
11. Extraer índice PT (bits 12-20) → idx = 0x678
12. PT[678] → dirección física de página
13. Sumar offset (bits 0-11) = 0x678
14. Dirección final = página_física + offset
```

---

### 3. **Page Table Entry (PTE): 8 Bytes de Información**

```rust
pub struct PageTableEntry(u64);
```

Cada entrada es **8 bytes (u64)** con estructura:

```
Bits 0-11:   FLAGS
  Bit 0: PRESENT (¿existe la página?)
  Bit 1: WRITABLE (¿se puede escribir?)
  Bit 2: USER_ACCESSIBLE (¿puede acceder user-mode?)
  Bit 3: WRITE_THROUGH
  Bit 4: CACHE_DISABLE
  Bit 5: ACCESSED (CPU pone automáticamente)
  Bit 6: DIRTY (CPU pone automáticamente)
  Bit 7: HUGE_PAGE

Bits 12-51:  DIRECCIÓN FÍSICA
  Bits 12-51 = dirección de página física (40 bits)

Bits 52-63:  DISPONIBLE para software
  Puedes usar estos bits como quieras
```

**Ejemplo en binario:**
```
1000_0000_0000 1111_1111_1111_1111_1111_1111_1111_1000_0000_0011
             ↑                                           ↑ ↑
          dirección física                    WRITABLE PRESENT
```

---

### 4. **CR3: El Registro Raíz del Paging**

```rust
unsafe { asm!("mov cr3, {}", in(reg) cr3); }
```

**¿Qué es CR3?**
- Registro de control del CPU (64-bit)
- Contiene **dirección física de tabla PML4**
- Bits 0-11: flags
- Bits 12-51: dirección PML4

**¿Por qué es especial?**
- Cuando escribes en CR3, el CPU **activa paging**
- MMU consulta CR3 para cada traducción
- Cada proceso tiene diferente CR3 = diferentes espacios de direcciones

---

### 5. **CR0: Bit PE (Paging Enable)**

```rust
unsafe {
    asm!("mov rax, cr0");
    asm!("or rax, 1");      // Activar bit 0 (PE)
    asm!("mov cr0, rax");
}
```

**¿Qué es CR0?**
- Registro de control del CPU
- Bit 0 = PE (Paging Enable)
- Otros bits = cache, FPU, protected mode, etc.

**¿Qué pasa cuando activas PE?**
```
Antes: CPU usa direcciones físicas directas
Después: CPU SOLO acepta direcciones virtuales
         Todas traducidas por MMU usando CR3
```

**¿Qué pasa si CR3 no está configurado?**
→ **Page Fault (INT 14)** - El CPU no encuentra la página

---

### 6. **Identity Mapping: Dirección Virtual = Dirección Física**

```rust
paging::identity_map_page(0x400000);  // Mapea 0x400000 → 0x400000
paging::identity_map_page(0xb8000);   // Mapea 0xb8000 → 0xb8000
```

**¿Por qué identity mapping?**
- Simplest to implement (virtual = física)
- Kernel actual puede estar en cualquier dirección física
- Dirección virtual sigue siendo la misma

**Ventaja:** No necesitas recompilar si cambia dirección física.

---

## 📝 Código Final Completo

### `src/paging.rs` (Estructuras)

```rust
use bitflags::bitflags;
use core::arch::asm;

// PageTableEntry: 8 bytes que representan una entrada
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    pub fn new() -> Self {
        PageTableEntry(0)
    }

    pub fn is_present(&self) -> bool {
        (self.0 & 1) != 0
    }

    pub fn address(&self) -> u64 {
        self.0 & 0x000F_FFFF_FFFF_F000  // Bits 12-51
    }

    pub fn set(&mut self, addr: u64, flags: PageTableFlags) {
        self.0 = (addr & 0x000F_FFFF_FFFF_F000) | flags.bits();
    }
}

// Flags de Page Table Entry
bitflags! {
    pub struct PageTableFlags: u64 {
        const PRESENT           = 1 << 0;
        const WRITABLE          = 1 << 1;
        const USER_ACCESSIBLE   = 1 << 2;
        const WRITE_THROUGH     = 1 << 3;
        const CACHE_DISABLE     = 1 << 4;
        const ACCESSED          = 1 << 5;
        const DIRTY             = 1 << 6;
        const HUGE_PAGE         = 1 << 7;
    }
}

// PageTable: 512 entradas = 4KB
#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::new(); 512],
        }
    }

    pub fn get(&self, index: usize) -> PageTableEntry {
        self.entries[index]
    }

    pub fn set(&mut self, index: usize, entry: PageTableEntry) {
        self.entries[index] = entry;
    }

    pub fn set_entry(&mut self, index: usize, addr: u64, flags: PageTableFlags) {
        let mut entry = PageTableEntry::new();
        entry.set(addr, flags);
        self.set(index, entry);
    }
}

// Traducir dirección virtual → física
pub fn walk_page_table(virtual_addr: u64) -> Option<u64> {
    let pml4_idx = ((virtual_addr >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virtual_addr >> 30) & 0x1FF) as usize;
    let pd_idx = ((virtual_addr >> 21) & 0x1FF) as usize;
    let pt_idx = ((virtual_addr >> 12) & 0x1FF) as usize;
    let offset = virtual_addr & 0xFFF;

    let mut cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }
    let pml4 = unsafe { &*(cr3 as *const PageTable) };

    // Nivel 1: PML4 → PDPT
    let pdpt_entry = pml4.get(pml4_idx);
    if !pdpt_entry.is_present() {
        return None;
    }
    let pdpt = unsafe { &*(pdpt_entry.address() as *const PageTable) };

    // Nivel 2: PDPT → PD
    let pd_entry = pdpt.get(pdpt_idx);
    if !pd_entry.is_present() {
        return None;
    }
    let pd = unsafe { &*(pd_entry.address() as *const PageTable) };

    // Nivel 3: PD → PT
    let pt_entry = pd.get(pd_idx);
    if !pt_entry.is_present() {
        return None;
    }
    let pt = unsafe { &*(pt_entry.address() as *const PageTable) };

    // Nivel 4: PT → página física
    let page_entry = pt.get(pt_idx);
    if !page_entry.is_present() {
        return None;
    }

    let physical_addr = page_entry.address() + offset;
    Some(physical_addr)
}

// Mapear una página (identity mapping)
pub fn identity_map_page(virtual_addr: u64) {
    let pml4_idx = ((virtual_addr >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virtual_addr >> 30) & 0x1FF) as usize;
    let pd_idx = ((virtual_addr >> 21) & 0x1FF) as usize;
    let pt_idx = ((virtual_addr >> 12) & 0x1FF) as usize;

    let mut cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    let pml4 = unsafe { &mut *(cr3 as *mut PageTable) };

    // Nivel 1: PML4 → PDPT
    let pdpt_entry = pml4.get(pml4_idx);
    let pdpt_addr = pdpt_entry.address();
    let pdpt = unsafe { &mut *(pdpt_addr as *mut PageTable) };

    // Nivel 2: PDPT → PD
    let pd_entry = pdpt.get(pdpt_idx);
    let pd_addr = pd_entry.address();
    let pd = unsafe { &mut *(pd_addr as *mut PageTable) };

    // Nivel 3: PD → PT
    let pt_entry = pd.get(pd_idx);
    let pt_addr = pt_entry.address();
    let pt = unsafe { &mut *(pt_addr as *mut PageTable) };

    // Nivel 4: PT → mapear dirección
    pt.set_entry(pt_idx, virtual_addr, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
}
```

### `src/main.rs` (Activación)

```rust
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u16;
    
    interrupts::init_idt();

    // === MAPEO DE PÁGINAS ===
    
    // Mapear kernel code (0x400000 - 0x500000)
    for addr in (0x400000..0x500000).step_by(0x1000) {
        paging::identity_map_page(addr);
    }

    // Mapear VGA buffer
    paging::identity_map_page(0xb8000);

    // === ACTIVAR PAGING ===
    
    // Leer CR3 (ya tiene dirección de PML4 del bootloader)
    let mut cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    // Escribir CR3 (instala PML4)
    unsafe {
        asm!("mov cr3, {}", in(reg) cr3);
    }

    // Activar paging en CR0 (bit PE = bit 0)
    unsafe {
        asm!("mov rax, cr0");
        asm!("or rax, 1");        // Activar bit PE
        asm!("mov cr0, rax");
    }
    
    // Escribir "Paging OK!" en VGA
    let msg = b"Paging OK!";
    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            *vga.add(i) = ((0x0F as u16) << 8) | (byte as u16);
        }
    }

    loop {}
}
```

---

## 🔄 Flujo de Ejecución

```
_start()
  ↓
1. Inicializar IDT (interrupts)
  ↓
2. Loop: Mapear kernel pages (0x400000..0x500000)
   - Para cada dirección: identity_map_page()
   - Walk tables PML4→PDPT→PD→PT
   - Set_entry con PRESENT | WRITABLE
  ↓
3. Mapear VGA buffer (0xb8000)
  ↓
4. Leer CR3 (dirección PML4 del bootloader)
  ↓
5. Escribir CR3 (instala PML4)
  ↓
6. Activar PE en CR0
  ↓
7. CPU activa paging automáticamente
  ↓
8. Cualquier acceso a memoria pasa por MMU
  ↓
9. Loop infinito (kernel ejecutando)
```

---

## 📊 Resumen de Conceptos

| Concepto | Qué es | Por qué |
|----------|--------|--------|
| **PageTableEntry** | 8 bytes con dirección + flags | Representa mapeo virtual→física |
| **PageTable** | 512 entradas de 4KB | Un nivel de jerarquía |
| **Jerarquía 4 niveles** | PML4→PDPT→PD→PT | Cubrir 256TB de direcciones |
| **Identity mapping** | virtual = física | Simplest implementation |
| **CR3** | Dirección de PML4 | Raíz del sistema de paging |
| **CR0 PE** | Bit 0 para activar paging | Orden al CPU para usar MMU |
| **unsafe** | Promesa de Rust | Necesario para assembly + pointers |
| **Bitflags** | Combinar múltiples bits | Eficiente para flags de CPU |

---

## 🧠 Lecciones Aprendidas

### 1. **Paging es Indirección**
Todo en sistemas operativos es indirección. La clave es entender qué se mapea a qué.

### 2. **Unsafe es Necesario en Bare Metal**
No es "malo", es NECESARIO para acceso a hardware. Pero requiere entender invariantes.

### 3. **La Jerarquía Resuelve el Problema de Escala**
4 niveles = 256TB. Genial. Sin jerarquía = 2^48 entradas en una tabla = imposible.

### 4. **Identity Mapping es Ingeniero Perezoso**
Primer paso = mapear virtual = física. Luego puedes hacer cosas sofisticadas.

### 5. **El Bootloader nos Ayuda**
Bootloader ya crea PML4 básica. Solo tuvimos que llenarla.

---

## 🚀 Próximos Pasos (Proyecto 8+)

### Mejoras a Paging
- Mapeo no-identity (virtual diferente de física)
- Protección por proceso (cada proceso su CR3)
- Lazy mapping (crear páginas bajo demanda)
- Swapping (página en disco, traer a RAM)

### Nuevos Proyectos
- **Proyecto 8:** System Calls Interface
- **Proyecto 9:** File System Basics
- **Proyecto 10:** Network Stack

---

## 📖 Referencias Técnicas

- **AMD64 Architecture Manual** - Sección sobre paging
- **Intel x86-64 Reference** - CR3, CR0, page tables
- **OSDev Wiki** - https://wiki.osdev.org/Paging
- **Philipp Oppermann's OS Blog** - https://os.phil-opp.com/

---

## ✅ Checklist de Comprensión

Antes de continuar, asegúrate de entender:

- [ ] ¿Qué es dirección virtual vs física?
- [ ] ¿Por qué necesitamos page tables?
- [ ] ¿Cómo funciona jerarquía de 4 niveles?
- [ ] ¿Qué es CR3 y CR0?
- [ ] ¿Por qué unsafe es necesario?
- [ ] ¿Qué significa identity mapping?
- [ ] ¿Cómo se traduce 0x12345678?

Si respondiste "no" a alguna → re-lee esa sección.

---

**🎉 Proyecto 7: Completado. ¡Ahora tienes un kernel con virtual memory!**
