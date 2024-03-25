#![feature(panic_info_message)]
#![no_main]
#![no_std]
#[macro_use]

mod console;
mod lang_items;
mod sbi;
use crate::sbi::sleep;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello world!"); 
    sleep(0,5);
    panic!("shut down machine");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}
