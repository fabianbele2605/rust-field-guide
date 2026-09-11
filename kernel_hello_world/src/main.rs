#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(); 

use core::cell::Cell;
use core::arch::asm;

pub struct BumpAllocator {
    heap: [u8; 0x1000],     // 4KB heap
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

use core::alloc::{GlobalAlloc, Layout};

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Bump allocator simple: solo suma el offset
        let mut alloc = self.heap.as_ptr() as usize;
        alloc += self.offset.get();
        self.offset.set(self.offset.get() + layout.size());
        alloc as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator no desasigna (simple)
    }
}

unsafe impl Sync for BumpAllocator {}

mod interrupts;
mod task;
mod paging;
mod filesystem;

use spin::Mutex;
use task::TaskManager;
use filesystem::FileSystem;

static TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
static FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new());

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Dirección física de VGA buffer
    let vga = 0xb8000 as *mut u16;
    
    // Inicializar tabla de interrupts
    interrupts::init_idt();

    // Mapear kernel code (0x400000 - 0x500000)
    for addr in (0x400000..0x500000).step_by(0x1000) {
        paging::identity_map_page(addr);
    }

    // Mapear VGA buffer
    paging::identity_map_page(0xb8000);

    // Crear 3 tareas de prueba
    {
        let mut tm = TASK_MANAGER.lock();
        tm.create_task();
        tm.create_task();
        tm.create_task();
    }

    // Probar filesystem
    {
        let mut fs = FILESYSTEM.lock();
        fs.create_file(b"hello.txt", b"Hola Kernel!");

        if let Some(data) = fs.read_file(b"hello.txt") {
            // Escribir primer byte leído en VGA, para comprobar visualmente
            unsafe {
                *vga.add(20) = ((0x0F as u16) << 8) | (data[0] as u16);
            }
        }
    }
    
    // Escribir "Hello Kernel!" en primera línea
    for i in 0..13 {
        let chars = b"Hello Kernel!";
        unsafe {
            *vga.add(i) = ((0x0F as u16) << 8) | (chars[i] as u16);
        }
    }
    
    // Llenar resto de línea con espacios verdes
    for i in 13..80 {
        unsafe {
            *vga.add(i) = ((0x0A as u16) << 8) | (b' ' as u16);
        }
    }

    // Obtener direccion de PML4 (CR3 actual)
    let mut cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    // Escribir CR3 con la misma direccion (ahora con paging activo)
    unsafe {
        asm!("mov cr3, {}", in(reg) cr3);
    }

    // Activar paging: establecer bit PE (bit 0) en CR0
    unsafe {
        asm!("mov rax, cr0");
        asm!("or rax, 1"); // Activar bit PE
        asm!("mov cr0, rax");
    }

    // Probar syscall: escribir 'X' en posicion 5 del VGA (via INT 0x80)
    unsafe {
        asm!(
            "mov rax, 1",           // syscall number = WriteVGA
            "mov rdi, 5",           // posicion
            "mov rsi, {0}",         // caracter (0x0F58 = blanco + 'X')
            "int 0x80",
            in(reg) 0x0F58u64,
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


