use crate::tooling::qemu_io::*;
use crate::tooling::serial::*;
use core::ptr;
use crate::utils::qemu_io::qemu_print_nln;

//Enum corresponding to the default color palette
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ColorCode {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    White = 0x7,
    Gray = 0x8,
    BrightBlue = 0x9,
    BrightGreen = 0xa,
    BrightCyan = 0xb,
    BrightRed = 0xc,
    BrightMagenta = 0xd,
    Yellow = 0xe,
    BrightWhite = 0xf,
}

// pub fn ColorCode_to_u8(color: ColorCode) -> u8 {
//     match color {
//         ColorCode::Black => 0x0,
//         ColorCode::Blue => 0x1,
//         ColorCode::Green_ => 0x2,
//         ColorCode::Cyan => 0x3,
//         ColorCode::Red_=> 0x4,
//         ColorCode::Magenta => 0x5,
//         ColorCode::Brown_=> 0x6,
//         ColorCode::White => 0x7,
//         ColorCode::Gray_=> 0x8,
//         ColorCode::BrightBlue => 0x9,
//         ColorCode::BrightGreen_=> 0xa,
//         ColorCode::BrightCyan => 0xb,
//         ColorCode::BrightRed_=> 0xc,
//         ColorCode::BrightMagenta => 0xd,
//         ColorCode::Yellow_=> 0xe,
//         ColorCode::BrightWhite => 0xf,
//     }
// }

pub fn u8_to_ColorCode(num: u8) -> ColorCode {
    qemu_print_num(num as u64);
    qemu_print_nln();
    match num {
        0x0 => ColorCode::Black,
        0x1 => ColorCode::Blue,
        0x2 => ColorCode::Green,
        0x3 => ColorCode::Cyan,
        0x4 => ColorCode::Red,
        0x5 => ColorCode::Magenta,
        0x6 => ColorCode::Brown,
        0x7 => ColorCode::White,
        0x8 => ColorCode::Gray,
        0x9 => ColorCode::BrightBlue,
        0xa => ColorCode::BrightGreen,
        0xb => ColorCode::BrightCyan,
        0xc => ColorCode::BrightRed,
        0xd => ColorCode::BrightMagenta,
        0xe => ColorCode::Yellow,
        0xf => ColorCode::BrightWhite,
        // TODO: Fix with panic
        _ => {
            qemu_print("Wrong Color: ");
            qemu_print_num(num as u64);
            qemu_print_nln();
            ColorCode::BrightWhite
        }
    }
}

// // TODO: Make it independent of constant size, kalloc
pub fn u8_buf_to_ColorCode(p: *mut u8) -> *mut ColorCode {
    let mut buf: [ColorCode; 256] = [ColorCode::Black; 256];
    let (_width, _height) = (16, 16);

    for y in 0.._height {
        for x in 0.._width {
            buf[_height * y + x] = u8_to_ColorCode(unsafe { *p.offset((_height * y + x) as isize) });
        }
    }

    buf.as_mut_ptr()
}

//This is the default VGA color palette
const DEFAULT_COLOR_MAPPING: [(u8, u8, u8); 16] = [
    (0, 0, 0),
    (0, 0, 42),
    (0, 42, 0),
    (0, 42, 42),
    (42, 0, 0),
    (42, 0, 42),
    (42, 42, 0),
    (42, 42, 42),
    (0, 0, 21),
    (0, 0, 63),
    (0, 42, 21),
    (0, 42, 63),
    (42, 0, 21),
    (42, 0, 63),
    (42, 42, 21),
    (42, 42, 63),
];

//Custom color format, for possibly chagning the color palette
#[derive(Clone, Copy)]
pub struct CustomColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl CustomColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        assert!(r < 64 && g < 64 && b < 64);
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
}

//Contains the color palatte information, object of planar_writer
pub struct ColorPalette {
    color_entries: [CustomColor; 16],
}

impl ColorPalette {
    pub fn new() -> Self {
        let mut tmp_init = ColorPalette {
            color_entries: [CustomColor {
                red: 0,
                green: 0,
                blue: 0,
            }; 16],
        };
        tmp_init.reset();
        return tmp_init;
    }

    //Reset the palette to the default state
    pub fn reset(&mut self) {
        for i in 0..16 {
            let v = DEFAULT_COLOR_MAPPING[i];
            self.color_entries[i] = CustomColor::new(v.0, v.1, v.2);
        }
    }

    //Update the internal VGA state with the palette object
    pub fn update_vga_dac_state(&self) {
        qemu_print("masking dac\n");
        //outb(0x3c6, 0xff);
        qemu_print("Updating...\n");

        let r = inb(0x3c7);
        if (r == 3) {
            qemu_print("Ready to accept writes\n");
        } else {
            qemu_print("Ready to accept reads\n");
        }

        outb(0x3c8, 0);
        for i in 0..16 {
            //outb(0x3c8, i);
            qemu_print_num(i as u64);
            let r = self.color_entries[i as usize].red;
            let g = self.color_entries[i as usize].green;
            let b = self.color_entries[i as usize].blue;

            outb(0x3c9, r);
            outb(0x3c9, g);
            outb(0x3c9, b);
            /*
            unsafe {
                outb(
                    0x3c9,
                    *((ptr::addr_of!(self.color_entries[i as usize]) as u64 + 8) as *mut u8),
                );
            }
            outb(0x3c9, g);
            outb(0x3c9, b);
            */
        }
    }

    //Write to the color palette
    pub fn update_color_entry(&mut self, entry: u8, color: CustomColor) -> &Self {
        self.color_entries[entry as usize] = color;
        return self;
    }

    //Print out the internal VGA state
    pub fn read_vga_dac_state() {
        qemu_println("------Writing out color entries------");

        for i in 0..16 {
            outb(0x3c7, i);

            let r = inb(0x3c9);
            let g = inb(0x3c9);
            let b = inb(0x3c9);
            qemu_print("");
            qemu_print_hex(r.into());
            qemu_print_hex(g.into());
            qemu_print_hex(b.into());
            qemu_print("\n");
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Rect {
    width: u8,
    height: u8,
    mid: (u8, u8),
}

pub fn graphics_error() {
    qemu_println("Oh no!, we got a graphics error!");
}
