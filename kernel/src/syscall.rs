use crate::file::{feof, fopen, fread, fseek, ftell};
use crate::ERROR;
use crate::{filesystem, kprintln};
use crate::{keyboard, vga};
use crate::{time, FILESYSTEM, USERLAND};
use core::arch::asm;

#[no_mangle]
pub extern "C" fn system_call() -> u64 {
    let mut syscall_nr: i64;

    unsafe {
        asm!("
        ",
            out("rdi") syscall_nr
        );
    }

    match syscall_nr {
        1 => return syscall_write(),
        2 => return syscall_getpid(),
        3 => return syscall_plot_pixel(),
        4 => return syscall_malloc(),
        5 => return syscall_fopen(),
        6 => return syscall_fread(),
        7 => return syscall_fseek(),
        8 => return syscall_ftell(),
        9 => return syscall_feof(),
        10 => return syscall_plot_framebuffer(),
        11 => return syscall_switch_vga_mode(),
        12 => return syscall_get_keystate(),
        13 => return syscall_get_time(),
        14 => return syscall_stat(),
        15 => return syscall_chdir(),
        16 => return syscall_getcwd(),
        _ => {
            ERROR!("Undefined system call triggered: {}", syscall_nr);
            return 0xdeadbeef;
        }
    }
}

fn syscall_feof() -> u64 {
    return feof();
}

fn syscall_ftell() -> u64 {
    return ftell() as u64;
}

fn syscall_fseek() -> u64 {
    let mut offset: usize;
    let mut origin: usize;

    unsafe {
        asm!("",
            out("r8") offset,
            out("r9") origin,
        );
    }

    return fseek(offset, origin);
}

fn syscall_fread() -> u64 {
    let mut ptr: u64;
    let mut num_bytes: usize;

    unsafe {
        asm!("",
            out("r8") ptr,
            out("r9") num_bytes,
        );
    }

    return fread(ptr as *mut u8, num_bytes);
}

fn syscall_fopen() -> u64 {
    return fopen();
}

fn syscall_malloc() -> u64 {
    let mut size: usize;

    unsafe {
        asm!("",
            out("r8") size
        );
    }

    return USERLAND.lock().process_malloc(size);
}

fn syscall_plot_pixel() -> u64 {
    let mut x: u32;
    let mut y: u32;
    let mut color: u32;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") x,
            out("r9") y,
            out("r10") color
        );
    }

    vga::vga_plot_pixel(x, y, color as u8);
    vga::vga_flip();

    return 0;
}

fn syscall_getpid() -> u64 {
    USERLAND.lock().get_current_process_id() as u64
}

fn syscall_write() -> u64 {
    let mut filedescriptor: i64;
    let mut payload: i64;
    let mut len: i64;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") filedescriptor,
            out("r9") payload,
            out("r10") len
        );

        match core::str::from_utf8(core::slice::from_raw_parts(
            payload as *const u8,
            len as usize,
        )) {
            Ok(msg) => match filedescriptor {
                // stdout
                1 => {
                    kprintln!("{}", msg)
                }
                _ => ERROR!("Undefined filedescriptor!"),
            },
            Err(_) => ERROR!("\nCouldnt reconstruct string!\n"),
        }
    }

    return 0;
}

fn syscall_plot_framebuffer() -> u64 {
    let mut framebuffer: u64;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") framebuffer,
        );
    }

    vga::vga_plot_framebuffer(framebuffer as *const u8);
    vga::vga_flip();

    return 0;
}

fn syscall_switch_vga_mode() -> u64 {
    let mut vga_on: u64;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") vga_on,
        );
    }

    if vga_on != 0 {
        vga::vga_enter();
        vga::vga_clear_screen();
    } else {
        vga::vga_exit();
    }

    return 0;
}

fn syscall_get_keystate() -> u64 {
    let mut key: usize;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") key,
        );
    }

    let keystate;

    unsafe {
        keystate = keyboard::KEYSTATES[key];
        keyboard::KEYSTATES[key] = false;
    }

    return keystate as u64;
}

fn syscall_get_time() -> u64 {
    let sec: *mut u32;
    let usec: *mut u32;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") sec,
            out("r9") usec,
        );
    }

    unsafe {
        (*sec, *usec) = time::get_time();
    }

    return 1;
}

fn syscall_stat() -> u64 {
    let mut pathname: *const u64;
    let mut statbuf: *mut u64;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") pathname,
            out("r9") statbuf,
        );
    }

    return FILESYSTEM.lock().stat(pathname, statbuf);
}

fn syscall_chdir() -> u64 {
    let mut pathname: *const u64;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") pathname,
        );
    }

    // get string from pathname pointer
    match unsafe { core::str::from_utf8(core::slice::from_raw_parts(pathname as *const u8, 256)) } {
        Ok(pathname) => {
            return USERLAND
                .lock()
                .get_current_process()
                .set_working_directory(pathname)
        }
        Err(_) => return u64::MAX,
    }
}

fn syscall_getcwd() -> u64 {
    let mut buf: *mut u64;
    let mut size: u64;

    unsafe {
        asm!("",
            out("r8") buf,
            out("r9") size,
        );
    }

    let cwd = USERLAND
        .lock()
        .get_current_process()
        .get_working_directory();

    // copy cwd to buf
    let cwd_bytes = cwd.as_bytes();
    let cwd_len = cwd_bytes.len();
    let buf_slice = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, size as usize) };
    for i in 0..core::cmp::min(cwd_len, size as usize) {
        buf_slice[i] = cwd_bytes[i];
    }

    return cwd_len as u64;
}
