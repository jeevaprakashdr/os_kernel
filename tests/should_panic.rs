#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os_kernel::{exit_qemu, serial_println, serial_print};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[tests did not panic]");
    exit_qemu(os_kernel::QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(os_kernel::QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}