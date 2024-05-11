use crate::types::*;

// Imports the `DbgPrint` function from `ntoskrnl.exe`, and set as a global function (pub),
// that can be called from anywhere module in the crate.
#[link(name = "ntoskrnl")]
extern "C" {
    #[allow(dead_code)]
    pub fn DbgPrint(format: PCSTR, ...) -> NTSTATUS;
}

// Macro that calls the `DbgPrint` function, and prepends the string with the name of the driver,
// and a null terminator. The `unsafe` is used because the `DbgPrint` function is written in a different language,
// and Rust cannot guarantee safety.
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! kd_print {
    ($string: expr) => {
        unsafe {
            $crate::DbgPrint(concat!("[hello_win_kernel.sys]", $string, "\0").as_ptr())
        }
    };

    ($string: expr, $($x:tt)*) => {
        unsafe {
            $crate::DbgPrint(concat!("[hello_win_kernel.sys]", $string, "\0").as_ptr(), $($x)*)
        }
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! kd_print {
    ($string: expr) => {};
    ($string: expr, $($x:tt)*) => {};
}