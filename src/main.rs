#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod tooling;

use tooling::vga::write_str_at;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    write_str_at("Hello World!", 0, 0, 0xb);
    panicking_function(); 
    loop {}
}

fn panicking_function() -> ! {
    //write_str_at("Panicking function call", 0, 0, 0xb);
    //tooling::panic_handler::stack_trace();
    panic!("This is a test panic.");

    loop {}
}
