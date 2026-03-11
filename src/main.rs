#![no_std]
#![no_main]

mod vga_buffer;

use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
     println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // vga_buffer::print_something();

    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers {} {}", 12, 2.34).unwrap();

    println!("Hello world{}", "!");
    
    panic!("testing panic");
    loop { }
}
