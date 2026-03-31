#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_kernel::test_runner)]
#![reexport_test_harness_main = "invoke_all_tests"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    
    os_kernel::init();
    
    fn stack_overflow () {
        stack_overflow();
    }

    // stack_overflow();

    #[cfg(test)]
    invoke_all_tests();
    
    println!("it did not crash");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
     println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_kernel::test_panic_handler(info)
}