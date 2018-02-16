#![no_std]
#![feature(intrinsics)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(compiler_builtins_lib)]

use uefi::SimpleTextOutput;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_copy_implementations)]
pub mod uefi;

#[allow(unused_attributes)] // The below attribute is needed to specify the entry point. Hence suppressing the warning
#[link_args = "/ENTRY:efi_start"]
extern "C" {}

extern crate rlibc;
extern crate compiler_builtins;

#[no_mangle]
#[lang="panic_fmt"]
pub extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

pub fn efi_main(sys : uefi::SystemTable) {
    sys.console().write("Hello, World!\n\r");

    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
	loop {}
}

#[no_mangle]
pub fn breakpoint() -> ! {
	loop {}
}

#[no_mangle]
pub fn __chkstk() -> ! {
	loop {}
}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    // we can't really do much in this benighted hellhole of
    // an environment without bringing in more libraries.
    // We can make syscalls, segfault, and set the exit code.
    // To be sure that this actually ran, let's set the exit code.
    42
}

fn main() {
    // we can't really do much in this benighted hellhole of
    // an environment without bringing in more libraries.
    // We can make syscalls, segfault, and set the exit code.
    // To be sure that this actually ran, let's set the exit code.
}
// #[no_mangle]
// pub extern "win64" fn __CxxFrameHandler3() -> ! {
// 	loop {}
// }
