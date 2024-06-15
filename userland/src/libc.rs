use core::{arch::asm, array};

//pid_t getppid(void);

pub fn getpid() -> u64 {
    let mut _pid = 0xdeadbeef;

    unsafe {
        asm!("
            push rdi
            mov rdi, 2

            push r11
            push rcx
        
            syscall
        
            pop rcx
            pop r11
            pop rdi
            ",
            out("rax") _pid,
            options(nostack)
        );
    }

    return _pid;
}

pub fn write(filedescriptor: i64, payload: *const u64, len: usize) {
    unsafe {
        asm!("
            push rdi
            mov rdi, 1

            push r11
            push rcx
        
            syscall
        
            pop rcx
            pop r11
            
            pop rdi
        ",
            in("r8") filedescriptor,
            in("r9") payload as u64,
            in("r10") len,
            options(nostack),
            clobber_abi("C")
        );
    }
}

pub fn draw_pixel(x: u32, y: u32, color: u8) {
    unsafe {
        asm!("
            push rdi
            mov rdi, 3

            push r11
            push rcx
        
            syscall
        
            pop rcx
            pop r11
            
            pop rdi
        ",
            in("r8") x,
            in("r9") y,
            in("r10") color as u64,
            options(nostack),
            clobber_abi("C")
        );
    }
}

pub fn malloc(size: usize) -> u64 {
    let mut address: u64 = 0;

    unsafe {
        asm!("
            push rdi
            mov rdi, 4

            push r11
            push rcx

            syscall

            pop rcx
            pop r11

            pop rdi
        ",
            in("r8") size,
            out("rax") address,
            options(nostack),
            clobber_abi("C")
        );
    }

    return address;
}

pub fn free(address: u64) {
    // TODO implement working free; problem is that LockedHeap::dealloc requires the size...
    // TODO Implement; not doing anything for now
}

pub fn open(filename: &str, mode: &str) -> u64 {
    // TODO Implement; not doing anything for now
}

pub fn close(handle: u64) {
    // TODO Implement; not doing anything for now
}

pub fn read(handle: u64, count: usize) -> array<u8> {
    // FIXME needed
}

pub fn write(handle: u64, buffer: array<u8>, count: usize) -> buffer {
    // FIXME needed
}

pub fn seek(handle: u64, offset: u64, origin: u64) -> usize {
    // FIXME needed
}

pub fn tell(handle: u64) -> usize {
    // FIXME needed
}

pub fn eof(handle: u64) -> usize {
    // FIXME needed
}

pub struct Printer {}

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(1, s.as_bytes().as_ptr() as *const u64, s.len());
        Ok(())
    }
}

#[macro_export]
macro_rules! printf {
    () => {    };
    ($($arg:tt)*) => {{
        let mut printer = crate::libc::Printer {};
        core::fmt::write(&mut printer, core::format_args!($($arg)*)).unwrap();
    }};
}
