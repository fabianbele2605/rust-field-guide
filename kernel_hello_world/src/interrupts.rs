use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Lazy;
use core::arch::asm;
use crate::task;
use crate::TASK_MANAGER;

const SCAN_CODE_TABLE: [u8; 256] = [
    // 0x00 - 0x0F
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', b'\x08', b'\t',
    // 0x10 - 0x1F
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', 0, b'a', b's',
    // 0x20 - 0x2F
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, b'\\', b'z', b'x', b'c', b'v',
    // 0x30 - 0x3F
    b'b', b'n', b'm', b',', b'.', b'/', 0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0,
    // 0x40 - 0x4F
    0, 0, 0, 0, 0, 0, 0, b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1',
    // 0x50 - 0x5F
    b'2', b'3', b'0', b'.', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x60 - 0x6F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x70 - 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x80 - 0x8F (A partir de aquí son los 'Release Codes' cuando se suelta una tecla)
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x90 - 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0xA0 - 0xAF
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0xB0 - 0xBF
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0xC0 - 0xCF
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0xD0 - 0xDF
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0xE0 - 0xEF
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0xF0 - 0xFF
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];


static CURSOR_X: spin::Mutex<usize> = spin::Mutex::new(0);
static CURSOR_Y: spin::Mutex<usize> = spin::Mutex::new(0);

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt[33].set_handler_fn(keyboard_handler);
    idt[32].set_handler_fn(timer_handler);
    idt[0].set_handler_fn(divide_by_zero_handler);
    idt[14].set_handler_fn(page_fault_handler);
    idt[13].set_handler_fn(general_protection_fault_handler);
    idt[0x80].set_handler_fn(syscall_handler);
    idt
});

#[repr(u64)]
pub enum SyscallNumber {
    WriteVGA = 1,
    ReadKeyboard = 2,
    Sleep = 3,
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // Breakpoint handler (en bare metal no hay println)
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let scan_code: u8;
    unsafe {
        asm!("in al, 0x60", out("al") scan_code);
    }

    let ascii = SCAN_CODE_TABLE[scan_code as usize];

    if ascii != 0 {  // Si es un caracter valido
        let vga = 0xb8000 as *mut u16;
        let color = 0x0F;   // Blanco
        
        let mut x = CURSOR_X.lock();
        let mut y = CURSOR_Y.lock();

        unsafe {
            // Calcular posicion en memoria: y * 80 + x
            let pos = *y * 80 + *x;
            *vga.add(pos) = ((color as u16) << 8) | (ascii as u16);
        }

        *x += 1;   // incrementar cursor
        if *x >= 80  {  // Si llega al final de linea
            *x = 0;
            *y += 1;
        }
    }
}

extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: InterruptStackFrame) {
    // Manejador para INT 0
}

extern "x86-interrupt" fn page_fault_handler(_stack_frame: InterruptStackFrame) {
    // Page Fault handler (INT 14)
}

extern "x86-interrupt" fn general_protection_fault_handler(_stack_frame: InterruptStackFrame) {
    // General Protection Fault (INT 13)
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    let mut tm = TASK_MANAGER.lock();

    // 1. Guardar contexto de tarea actual
    if let Some(current) = tm.current_task() {
        save_current_context(&mut current.context);
    }

    // 2. Scheduler elige siguiente tarea
    tm.schedule_next();

    // 3. Restaurar contexto de nueva tarea
    if let Some(next) = tm.current_task() {
        restore_current_context(&next.context);
    }
}

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
            out(reg) arg2
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

pub fn save_current_context(context: &mut task::Context) {
    unsafe {
        asm!(
            "mov {0},   rax",
            "mov {1},   rbx",
            "mov {2},   rcx",
            "mov {3},   rdx",
            "mov {4},   rsi",
            "mov {5},   rdi",
            "mov {6},   rbp",
            "mov {7},   rsp",
            out(reg)    context.rax,
            out(reg)    context.rbx,
            out(reg)    context.rcx,
            out(reg)    context.rdx,
            out(reg)    context.rsi,
            out(reg)    context.rdi,
            out(reg)    context.rbp,
            out(reg)    context.rsp,
        );
    }
}

pub fn restore_current_context(context: &task::Context) {
    unsafe {
        asm! (
            "mov rax,   {0}",
            "mov rbx,   {1}",
            "mov rcx,   {2}",
            "mov rdx,   {3}",
            "mov rsi,   {4}",
            "mov rdi,   {5}",
            "mov rbp,   {6}",
            "mov rsp,   {7}",
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