#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate alloc;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use core::{arch::asm, panic::PanicInfo};

mod gdt;
mod interrupt;
mod keyboard;
mod kprint;
mod logging;
mod process;
mod syscall;
mod time;
mod userland;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logging::log("Kernel Panic!");

    kprintln!("{}", info);

    loop {}
}

fn init_heap() {
    extern "C" {
        static mut KERNEL_END: *const u64;
    }

    unsafe {
        let heap_start = KERNEL_END;
        kprintln!("heap start: {:x}\n", heap_start as u64);
        //let heap_end = …;

        //let heap_size = heap_end - heap_start;

        //ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    gdt::init_gdt();
    interrupt::init_idt();

    init_heap();

    clear_console!();
    kprintln!("successfull boot!");
    kprintln!("Hellö Wörld!");

    // Trigger exception
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    let userland: userland::Userland = userland::Userland::new();

    userland.switch_to_userland();

    //panic!("this is a terrible mistake!");

    loop {}
}
