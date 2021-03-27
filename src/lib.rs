#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

//! A no-std `Write`able buffer, to use like
//!
//! ```
//! use e_write_buffer::WriteBuffer;
//! use std::fmt::Write;
//! let mut buffer: WriteBuffer<20> = WriteBuffer::new();
//! let x = 12;
//! write!(buffer, "{}", x).expect("Can't write");
//! assert_eq!(buffer.as_str().unwrap(), "12");
//! ```

/// A write buffer
#[derive(Debug)]
pub struct WriteBuffer<const N: usize> {
    buf: [u8; N],
    offset: usize,
}

impl<const N: usize> WriteBuffer<N> {
    /// Creates a write buffer
    pub fn new() -> Self {
        let buf = [0u8; N];
        WriteBuffer { buf, offset: 0 }
    }

    /// Returns the valid portion of the buffer as slice
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.offset]
    }

    /// Reset the buffer to be reused
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Converts the buffer in `&str`, returnig None if some bytes doesn't resemble valid utf-8
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(self.as_slice()).ok()
    }
}

impl<const N: usize> core::fmt::Write for WriteBuffer<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::WriteBuffer;
    use core::fmt::Write;

    #[test]
    fn test_write_wrapper() {
        let x = 123;

        let mut buffer: WriteBuffer<20> = WriteBuffer::new();
        write!(buffer, "{}", x).unwrap();
        assert_eq!(&buffer.as_slice()[0..3], b"123");

        buffer.reset();
        let x = 2.242424;
        write!(buffer, "{:.2}", x).unwrap();
        assert_eq!(buffer.as_slice(), b"2.24");

        buffer.reset();
        let x = 20;
        write!(
            buffer,
            "Longer than {} characters sentence",
            x
        )
            .unwrap_err();

        buffer.reset();
        write!(buffer, "{}", "1").unwrap();
        write!(buffer, "{}", "2").unwrap();
        assert_eq!(buffer.as_slice(), b"12");
    }
}
