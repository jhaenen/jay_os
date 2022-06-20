#![no_std]
#![no_main]

mod panic;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    loop { }
}