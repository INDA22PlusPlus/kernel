use core::fmt::Arguments;
use core::fmt::Write;

use super::serial::{outb, outw};

/// Prints a 32-bit hexadecimal number to QEMU serial stdout
pub fn qemu_print_hex(value: u32) {
    let lookup: [u8; 16] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70,
    ];

    let mut v: u32 = value;

    qemu_print("0x");
    let mut i = 0;
    while i < 8 {
        let l: usize = ((v & 0xF0000000) >> 28) as usize;
        outb(0x3F8, lookup[l]);
        v <<= 4;
        i += 1;
    }
    qemu_print("\n");
}

/// Prints a string without newline to QEMU serial stdout
pub fn qemu_print(string: &str) {
    for b in string.bytes() {
        outb(0x3F8, b);
    }
}
pub fn qemu_fmt_println(string: &str, fmt: core::fmt::Arguments) {
    let mut writer = SerialWriter::new();

    write!(writer, "{}\n", fmt);
}

/// Prints a string with newline to QEMU serial stdout
pub fn qemu_println(string: &str) {
    qemu_print(string);
    qemu_print("\n");
}

/// Dummy struct to allow write! macros and the like to function
pub struct SerialWriter {}

impl SerialWriter {
    pub fn new() -> Self {
        SerialWriter {}
    }
}

// the advantage over VGAWriter is that newlines function
impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            outb(0x3F8, b);
        }
        Ok(())
    }
}
