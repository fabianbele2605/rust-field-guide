// Importes

use bitflags::bitflags;
use core::arch::asm;

// PageTableEntry (PTE)

#[derive(Clone, Copy)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    // Crear entrada vacía
    pub fn new() -> Self {
        PageTableEntry(0)
    }

    // Leer si está presente (bit 0)
    pub fn is_present(&self) -> bool {
        (self.0 & 1) != 0
    }

    // Leer direccion fisica (bits 12-51)
    pub fn address(&self) -> u64 {
        self.0 & 0x000F_FFFF_FFFF_F000
    }

    // Escribir flags + direccion
    pub fn set(&mut self, addr: u64, flags: PageTableFlags) {
        self.0 = (addr & 0x000F_FFFF_FFFF_F000) | flags.bits();
    }
}

// Flags

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

// PageTable

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

pub fn walk_page_table(virtual_addr: u64) -> Option<u64> {

    // Extrar indices de la direccion virtual

    // Como extraer

    let pml4_idx = ((virtual_addr >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virtual_addr >> 30) & 0x1FF) as usize;
    let pd_idx   = ((virtual_addr >> 21) & 0x1FF) as usize;
    let pt_idx   = ((virtual_addr >> 12) & 0x1FF) as usize;  
    let offset   = virtual_addr & 0xFFF;            

    // Leer CR3 (tabla PML4 actual)

    let pml4_addr: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) pml4_addr);
    }
    let pml4 = unsafe { &*(pml4_addr as *const PageTable )};

    // Caminar por 4 tablas

    // 1. Lee de PML4
    let pdpt_entry = pml4.get(pml4_idx);
    if !pdpt_entry.is_present() {
        return None;    // Pagina no existe
    }
    let pdpt = unsafe { &*(pdpt_entry.address() as *const PageTable) };

    // 2. Lee de PDPT
    let pd_entry = pdpt.get(pdpt_idx);
    if !pd_entry.is_present() {
        return None;
    }
    let pd = unsafe { &*(pd_entry.address() as *const PageTable) };

    // 3. Lee de PD
    let pt_entry = pd.get(pd_idx);
    if !pt_entry.is_present() {
        return None;
    }
    let pt = unsafe { &*(pt_entry.address() as *const PageTable) };

    // 4. Lee PT
    let page_entry = pt.get(pt_idx);
    if !page_entry.is_present() {
        return None;
    }

    // Retornar direccion fisica

    let physical_addr = page_entry.address() + offset;
    Some(physical_addr)
}

pub fn setup_paging() {
    // 1. Leer CR3
    let mut cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    // 2. Convertir CR3 → referencia mutable PageTable
    // Hint: En walk_page_table() usaste &*(...), pero aqui es &mut
    let pml4 = unsafe { &mut *(cr3 as *mut PageTable) };

    // 3. TODO: Mapear entradas (proxima fase)
    // Por ahora solo estructura
}

pub fn identity_map_page(virtual_addr: u64) {
    // Extraer indices de 4 niveles
    let pml4_idx = ((virtual_addr >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virtual_addr >> 30) & 0x1FF) as usize;
    let pd_idx = ((virtual_addr >> 21) & 0x1FF) as usize;
    let pt_idx = ((virtual_addr >> 12) & 0x1FF) as usize;

    // Leer CR3
    let mut cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    // Obtener PML4 (ya existe, bootloader lo crea)
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

    // Nivel 4: PT → mapear dirección (identity mapping: virtual = física)
    pt.set_entry(pt_idx, virtual_addr, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
}