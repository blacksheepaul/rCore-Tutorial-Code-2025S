#![no_std]
#![no_main]

mod console;
mod lang_items;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

// 清除bss段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, RCore World!\n");
    sbi::shutdown();
}
