extern crate alloc;
use alloc::boxed::Box;
use core::cmp;
use core::result::Result;
use fatfs;

use alloc::vec::Vec;

const DISK_IMG: &[u8] = include_bytes!("../../storage/disk.img");

#[repr(C, packed(2))]
#[derive(Clone, Copy)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_mode: u64,
    st_nlink: u64,
    st_uid: u64,
    st_gid: u64,
    st_rdev: u64,
    st_size: u64,
    st_blksize: u64,
    st_blocks: u64,
    st_atime: u64,
    st_mtime: u64,
    st_ctime: u64,
}

pub struct FileSystem<'a> {
    fs: Box<fatfs::FileSystem<Cursor<'a>, fatfs::NullTimeProvider, fatfs::LossyOemCpConverter>>,
    filehandles:
        Vec<fatfs::File<'a, Cursor<'a>, fatfs::NullTimeProvider, fatfs::LossyOemCpConverter>>,
}

impl<'a> FileSystem<'a> {
    pub fn new() -> FileSystem<'a> {
        let buf_stream = Cursor::new(DISK_IMG);
        let fs = Box::new(fatfs::FileSystem::new(buf_stream, fatfs::FsOptions::new()).unwrap());
        FileSystem {
            fs,
            filehandles: Vec::new(),
        }
    }

    pub fn stat(&mut self, pathname: *const u64, statbuf: *mut u64) -> u64 {
        let pathname = unsafe { core::slice::from_raw_parts(pathname as *const u8, 256) };
        let statbuf = unsafe { &mut *(statbuf as *mut Stat) };

        let (path, filename) = seperate_path_from_filename(core::str::from_utf8(pathname).unwrap());

        let root_dir = self.fs.root_dir();
        let dir = root_dir.open_dir(path).unwrap();

        for direntry in dir.iter() {
            let entry = direntry.unwrap();
            if entry.short_file_name_as_bytes() == filename.as_bytes() {
                statbuf.st_dev = 0;
                statbuf.st_ino = 0;
                statbuf.st_mode = 0;
                statbuf.st_nlink = 0;
                statbuf.st_uid = 0;
                statbuf.st_gid = 0;
                statbuf.st_rdev = 0;
                statbuf.st_size = entry.len();
                statbuf.st_blksize = 0;
                statbuf.st_blocks = 0;
                statbuf.st_atime = 0;
                statbuf.st_mtime = 0;
                statbuf.st_ctime = 0;
                return 0;
            }
        }

        return u64::MAX;
    }

    pub fn fopen(
        &'a mut self,
        filename: *const u64,
        mode: *const u64,
    ) -> &mut fatfs::File<'a, Cursor<'a>, fatfs::NullTimeProvider, fatfs::LossyOemCpConverter> {
        let filename = unsafe {
            core::str::from_utf8(core::slice::from_raw_parts(filename as *const u8, 256)).unwrap()
        };

        // TODO consider mode
        //let mode = unsafe { core::str::from_utf8(core::slice::from_raw_parts(mode as *const u8, 256)) };

        let root_dir = self.fs.root_dir();
        self.filehandles.push(root_dir.open_file(filename).unwrap());

        return self.filehandles.last_mut().unwrap();
    }

    /*pub fn fread(ptr: *mut u8, num_bytes: usize) -> u64 {
        unsafe {
            for i in 0..num_bytes {
                let dst = ptr.add(i);
                let src = file_start().add(FILE_POSITION + i);

                core::ptr::write_volatile(dst, *src);
            }

            FILE_POSITION += num_bytes;
        }
        num_bytes as u64
    }

    pub fn fseek(offset: usize, origin: usize) -> u64 {
        unsafe {
            let size = file_size();

            match origin {
                0 => FILE_POSITION = offset,
                1 => FILE_POSITION += offset,
                2 => FILE_POSITION = size - offset,
                _ => panic!("undefined fseek"),
            }
        }
        0
    }

    pub fn ftell() -> usize {
        unsafe { FILE_POSITION }
    }

    pub fn feof() -> u64 {
        unsafe {
            if FILE_POSITION >= file_size() {
                1
            } else {
                0
            }
        }
    }*/
}

fn seperate_path_from_filename(path: &str) -> (&str, &str) {
    let mut path = path;
    let mut filename = "";
    if let Some(pos) = path.rfind('/') {
        filename = &path[pos + 1..];
        path = &path[..pos];
    }
    (path, filename)
}

// see also https://github.com/rafalh/rust-fatfs/issues/55

pub struct Cursor<'a> {
    inner: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(inner: &'a [u8]) -> Cursor<'a> {
        Cursor { inner, pos: 0 }
    }
}

#[derive(Debug)]
pub enum DiskCursorIoError {
    UnexpectedEof,
    WriteZero,
}
impl fatfs::IoError for DiskCursorIoError {
    fn is_interrupted(&self) -> bool {
        false
    }

    fn new_unexpected_eof_error() -> Self {
        Self::UnexpectedEof
    }

    fn new_write_zero_error() -> Self {
        Self::WriteZero
    }
}

impl<'a> fatfs::IoBase for Cursor<'a> {
    type Error = DiskCursorIoError;
}

impl<'a> fatfs::Read for Cursor<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, DiskCursorIoError> {
        let num_of_bytes_to_read = cmp::min(buf.len(), self.inner.len() - self.pos);
        buf[..num_of_bytes_to_read]
            .copy_from_slice(&self.inner[self.pos..self.pos + num_of_bytes_to_read]);
        self.pos += num_of_bytes_to_read;
        Ok(num_of_bytes_to_read)
    }
}

impl<'a> fatfs::Write for Cursor<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, DiskCursorIoError> {
        // TODO implement write properly
        let num_bytes_to_write = cmp::min(buf.len(), self.inner.len() - self.pos);
        Ok(num_bytes_to_write)

        /*
        let num_bytes_to_write = cmp::min(buf.len(), self.inner.len() - self.pos);
        self.inner[self.pos..self.pos + num_bytes_to_write]
            .copy_from_slice(&buf[..num_bytes_to_write]);
        self.pos += num_bytes_to_write;
        Ok(num_bytes_to_write)
        */
    }

    fn flush(&mut self) -> Result<(), DiskCursorIoError> {
        // TODO implement flush to enable buffered writing
        Ok(())
    }
}

impl<'a> fatfs::Seek for Cursor<'a> {
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, DiskCursorIoError> {
        let (base_pos, offset) = match pos {
            fatfs::SeekFrom::Start(offset) => (0 as usize, offset as usize),
            fatfs::SeekFrom::End(offset) => (self.inner.len(), offset as usize),
            fatfs::SeekFrom::Current(offset) => (self.pos, offset as usize),
        };

        let new_pos = base_pos.checked_add(offset).unwrap();

        if new_pos < 0 || new_pos as usize > self.inner.len() {
            return Err(DiskCursorIoError::UnexpectedEof);
        }

        self.pos = new_pos as usize;
        Ok(self.pos as u64)
    }
}
