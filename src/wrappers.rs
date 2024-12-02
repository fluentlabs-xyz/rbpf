use crate::aligned_memory::IoWrite;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;

pub struct VecWriter {
    pub buf: Vec<u8>,
}

impl fmt::Write for VecWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buf.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

impl IoWrite for VecWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Box<dyn core::error::Error>> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        Ok(())
    }
}
