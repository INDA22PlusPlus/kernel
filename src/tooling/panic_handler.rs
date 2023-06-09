use crate::tooling::vga;
use crate::tooling::vga::VGAWriter;
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use super::qemu_io::{qemu_fmt_println, qemu_print, qemu_println};
//use core::option;

fn format_line_number(line: u32) -> &'static str {
    // maximum number of digits possible to print.
    // currently will not be able to print 5-digit line numbers
    // so make sure not to introduce any bugs on line 10000+ in any files guys...
    const BUFFER_SIZE: usize = 4;
    static mut BUFFER: [u8; BUFFER_SIZE + 1] = [0; BUFFER_SIZE + 1];

    if line == 0 {
        return "0";
    }

    let mut num = line;
    let mut idx = 0;

    while num > 0 && idx < BUFFER_SIZE {
        let digit = (num % 10) as u8;
        num /= 10;
        unsafe {
            BUFFER[BUFFER_SIZE - 1 - idx] = b'0' + digit;
        }
        idx += 1;
    }

    let start = BUFFER_SIZE - idx;
    unsafe {
        BUFFER[BUFFER_SIZE] = 0;
        let line_str = core::str::from_utf8_unchecked(&BUFFER[start..BUFFER_SIZE]);
        line_str
    }
}

/// Prints the file and line number where the panic occurred.
fn print_location(location: &core::panic::Location) {
    let file = location.file();
    let line = location.line();
    qemu_print(file);
    qemu_print(" ");
    qemu_print("Line: ");

    let line_str = format_line_number(line);
    qemu_println(&line_str)
}

/// Prints the panic message.
fn print_message(message: core::fmt::Arguments) {
    qemu_fmt_println("{}", message);
}

pub fn stack_trace() {
    let mut rbp: *mut u64;
    let mut saved_rbp: *mut u64;
    let mut saved_rip: u64;
    let mut should_quit: u64;
    unsafe {
        loop {
            // ; saved rbp is pointed to by rbp, which is stored in rbx
            // ; rip is 8 bytes above saved rbp
            asm!("
                    mov rbx, rbp
                    mov {0}, rbp
                    cmp rbp, 0
                    je 1f
                    mov {1}, [rbx] 
                    mov rbp, [rbx]  
                    sub rbx, 8     
                    mov {3}, [rbx]
                    mov {2}, 1
                    jmp 2f
                    1:
                        mov {2}, 0
                    2:
                ", out(reg) rbp, out(reg) saved_rbp, out(reg) should_quit, out(reg) saved_rip);
            //asm!("
            //    mov {0}, rbp
            //    mov rsp, rbp
            //
            //    pop rbp
            //    mov {1}, rbp
            // ", out(reg) ebp, out(reg) saved_ebp);
            qemu_fmt_println(
                "{}",
                format_args!(
                    "RBP = {:#x}, SAVED RBP = {:#x}, CALLER RIP = {:#x} TOP_FRAME = {}",
                    rbp as u64, saved_rbp as u64, saved_rip, should_quit
                ),
            ); // Some(print_message(format_args!("EBP = {:#x}, SAVED EBP = {:#x}, TOP_FRAME = {}\n", ebp as u64, saved_ebp as u64, should_quit), writer));
            qemu_print("\n");
            if should_quit == 1 {
                qemu_print("=========================================================\n\n");

                break;
            }
        }
    }
}

fn dump_current_frame() {}

/// This function is called on panic.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    qemu_println("\n\n=========================================================");

    qemu_println("PANIC!");

    if let Some(location) = info.location() {
        print_location(location);
    }

    if let Some(message) = info.message() {
        print_message(*message);
    }
    stack_trace();

    qemu_print("=========================================================\n\n");
    loop {}
}
