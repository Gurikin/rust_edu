#![no_std]

mod types;
mod ntstatus;
mod dbg;

use types::*;
use ntstatus::*;
use dbg::*;

#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!();
}

#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "system" fn driver_entry(_driver: PVOID, _path: PVOID) -> NTSTATUS {
    kd_print!("Hello, Kernel's World!\n");
    STATUS_SUCCESS
}