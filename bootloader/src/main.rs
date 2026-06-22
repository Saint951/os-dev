#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image_handle: *mut core::ffi::c_void, _system_table: *mut core::ffi::c_void) -> usize {
    0
}