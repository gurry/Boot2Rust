#![no_std]
#![feature(intrinsics)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(link_args)]

use uefi::SimpleTextOutput;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_copy_implementations)]
pub mod uefi;

#[link_args = "/SUBSYSTEM:EFI_APPLICATION"]
extern "C" {}

#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}


pub fn efi_main(sys : uefi::SystemTable) {
    sys.console().write("Hello, World!\n\r");

    // loop {}
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

// #[no_mangle]
// pub extern "win64" fn __CxxFrameHandler3() -> ! {
// 	loop {}
// }
