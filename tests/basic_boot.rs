#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_kernel::test_runner)]
#![reexport_test_harness_main = "invoke_all_tests"]

use core::panic::PanicInfo;
use os_kernel::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    invoke_all_tests();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_kernel::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}