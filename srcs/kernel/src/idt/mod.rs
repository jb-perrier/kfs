use handler::set_interrupt_handler;
use irq::*;
use isr::*;

use crate::{
    asm,
    error::KernelError,
    kernel::{kernel, kernel_option},
    process::ProcessState,
    text,
};

pub mod handler;
pub mod irq;
pub mod isr;
pub mod registers;

const FLAG_INTERRUPT_GATE: u8 = 0x8E;
const IDT_SIZE: usize = 256;
static mut IDT: [IDTEntry; IDT_SIZE] = [IDTEntry::new(0, 0, 0); IDT_SIZE];
static mut IDT_POINTER: IDTPointer = IDTPointer { limit: 0, base: 0 };

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IDTEntry {
    base_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    base_high: u16,
}

impl IDTEntry {
    pub const fn new(offset: usize, selector: u16, flags: u8) -> IDTEntry {
        IDTEntry {
            base_low: (offset & 0xFFFF) as u16,
            selector,
            zero: 0,
            flags, // | 0x60 for user mode, sets privilege level to 3
            base_high: ((offset >> 16) & 0xFFFF) as u16,
        }
    }
}

#[repr(C, packed)]
pub struct IDTPointer {
    limit: u16,
    base: u32,
}

pub fn init() -> Result<(), KernelError> {
    unsafe {
        IDT_POINTER.limit = (IDT.len() * core::mem::size_of::<IDTEntry>() - 1) as u16;
        IDT_POINTER.base = IDT.as_ptr() as u32;
    }

    remap_irq_table();

    set_entry(0, _isr0, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(1, _isr1, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(2, _isr2, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(3, _isr3, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(4, _isr4, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(5, _isr5, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(6, _isr6, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(7, _isr7, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(8, _isr8, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(9, _isr9, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(10, _isr10, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(11, _isr11, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(12, _isr12, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(13, _isr13, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(14, _isr14, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(15, _isr15, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(16, _isr16, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(17, _isr17, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(18, _isr18, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(19, _isr19, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(20, _isr20, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(21, _isr21, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(22, _isr22, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(23, _isr23, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(24, _isr24, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(25, _isr25, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(26, _isr26, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(27, _isr27, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(28, _isr28, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(29, _isr29, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(30, _isr30, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(31, _isr31, 0x08, FLAG_INTERRUPT_GATE);

    set_entry(32, _irq0, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(33, _irq1, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(34, _irq2, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(35, _irq3, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(36, _irq4, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(37, _irq5, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(38, _irq6, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(39, _irq7, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(40, _irq8, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(41, _irq9, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(42, _irq10, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(43, _irq11, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(44, _irq12, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(45, _irq13, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(46, _irq14, 0x08, FLAG_INTERRUPT_GATE);
    set_entry(47, _irq15, 0x08, FLAG_INTERRUPT_GATE);

    asm::idt_flush(&raw const IDT_POINTER);

    set_interrupt_handler(32, |regs| {
        if kernel_option().is_none() {
            return 0;
        }
        if !kernel().scheduler.running {
            return 0;
        }
        let Some(current_process) = kernel().get_current_process() else {
            return 0;
        };

        {
            // text::write_format!("HandlerRegisters: {:#?}\n", regs);
            let eip = regs.interrupt.eip;
            // text::write_format!("EIP: {:#x}\n", eip);
        }
        // text::write_str("Switching task, old esp: ");
        // text::write_num_hex!(regs.esp);
        // text::write_str("\n");
        current_process.stack_ptr = regs.esp as usize as *mut u8;

        let Some(next_process) = kernel().get_next_scheduled_process() else {
            return 0;
        };

        if next_process.state == ProcessState::Start {
            next_process.state = ProcessState::Running;
            text::write_format!("Starting process: {}\n", next_process.pid);
            let eip = next_process.func as *const () as u32;
            let eflags = 0x202;
            let ebp = next_process.stack_top.addr() as u32;
            let esp = next_process.stack_top.addr() as u32;
            asm::jump_in_new_process(esp, ebp, eip, eflags);
        }

        // if next_process.pid == current_process.pid {
        //     return 0;
        // }

        if next_process.state == ProcessState::Running {
            // text::write_str("New esp: ");
            // text::write_num_hex!(next_process.stack_ptr.addr() as u32);
            // text::write_str("\n");
            // text::write_format!("Swithing from {} to {}\n", current_process.pid, next_process.pid);
            // unsafe {
            //     let esp = next_process.stack_ptr.addr() as u32;
            //     // core::arch::asm! {
            //     //     "mov esp, {0}",
            //     //     "add esp, 4",
            //     //     "popad",
            //     //     "add esp, 8",
            //     //     "sti",
            //     //     "iret",
            //     //     in(reg) esp,
            //     //     options(noreturn),
            //     // }
            //     return esp;
            // }

            let old_esp = regs.esp;
            // text::write_format!("ESP saved: {:#x}", old_esp);
            let new_esp = next_process.stack_ptr.addr() as u32;
            // text::write_format!(" restored: {:#x}\n", new_esp);
            return new_esp;
        }
        0
    });
    Ok(())
}

fn set_entry(index: usize, base: unsafe extern "C" fn(), selector: u16, flags: u8) {
    unsafe {
        IDT[index] = IDTEntry::new(base as usize, selector, flags);
    }
}

fn get_interrupt_name(int_no: u32, err_code: u32) -> Option<&'static str> {
    match int_no {
        0 => Some("Division by zero"),
        1 => Some("Debug"),
        2 => Some("Non-maskable interrupt"),
        3 => Some("Breakpoint"),
        4 => Some("Overflow"),
        5 => Some("Bound range exceeded"),
        6 => Some("Invalid opcode"),
        7 => Some("Device not available"),
        8 => Some("Double fault"),
        9 => Some("Coprocessor segment overrun"),
        10 => Some("Invalid TSS"),
        11 => Some("Segment not present"),
        12 => Some("Stack-segment fault"),
        13 => Some("General protection fault"),
        14 => Some("Page fault"),
        16 => Some("x87 FPU floating-point error"),
        17 => Some("Alignment check"),
        18 => Some("Machine check"),
        19 => Some("SIMD floating-point exception"),
        20 => Some("Virtualization exception"),
        21 => Some("Control protection exception"),
        _ => return None, // "Unknown interrupt"
    }
}
