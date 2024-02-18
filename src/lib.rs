#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

//! A no-std `Write`able buffer, to use like
//!
//! ```
//! use e_write_buffer::WriteBuffer;
//! use std::fmt::Write;
//! let mut buffer: WriteBuffer<20> = WriteBuffer::new();
//! let x = 12;
//! write!(buffer, "{}", x).unwrap();
//! assert_eq!(buffer.as_str(), "12");
//! ```

use core::fmt::{self, Display, Formatter};

/// A write buffer
#[derive(Debug)]
pub struct WriteBuffer<const N: usize> {
    buffer: [u8; N],
    cursor: usize,
}

impl<const N: usize> WriteBuffer<N> {
    /// Creates a write buffer
    pub fn new() -> Self {
        let buf = [0u8; N];
        WriteBuffer {
            buffer: buf,
            cursor: 0,
        }
    }

    /// Returns a slice containing the already written bytes in the buffer
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.cursor]
    }

    /// Returns a mutable slice containing the already written bytes in the
    /// buffer
    ///
    /// Dev Note: This should _not_ be `pub`, since otherwise the user might
    /// mess with the bytes, violating the guarantee that the safety of
    /// [`as_str`] and [`as_str_mut`] depend on!
    fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.cursor]
    }

    /// Reset the buffer such that it can be reused.
    ///
    /// Note: This does _not_ overwrite any data in memory, it only sets the
    /// internal cursor back to the start of the buffer.
    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    /// Converts the buffer into `&str`.
    pub fn as_str(&self) -> &str {
        // SAFETY: The only way to write into `self.buf` is via
        // `Write::write_str`. Therefore it is always guaranteed that the buffer
        // contains valid UTF-8.
        unsafe { core::str::from_utf8_unchecked(self.as_slice()) }
    }

    /// Converts the buffer into `&mut str`.
    pub fn as_str_mut(&mut self) -> &mut str {
        // SAFETY: The only way to write into `self.buf` is via
        // `Write::write_str`. Therefore it is always guaranteed that the buffer
        // contains valid UTF-8.
        unsafe { core::str::from_utf8_unchecked_mut(self.as_slice_mut()) }
    }

    /// Returns how many bytes in the buffer have already been written.
    pub fn len(&self) -> usize {
        self.cursor
    }

    /// Returns true if zero bytes in the buffer are written.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns how many bytes in the buffer remain for writing.
    pub fn remaining(&self) -> usize {
        N - self.len()
    }

    /// Returns true if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.remaining() == 0
    }
}

impl<const N: usize> Default for WriteBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Write for WriteBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // New cursor after write
        let new_cursor = self.cursor + bytes.len();

        // If we would exceed the capacity of the buffer, we fail
        if new_cursor > N {
            return Err(fmt::Error);
        }

        // Efficiently copy the bytes into the bufffer
        self.buffer[self.cursor..new_cursor].copy_from_slice(bytes);

        // Update the cursor
        self.cursor = new_cursor;

        Ok(())
    }
}

impl<const N: usize> Display for WriteBuffer<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
        write!(buffer, "Longer than {} characters sentence", x).unwrap_err();

        buffer.reset();
        write!(buffer, "{}", "1").unwrap();
        write!(buffer, "{}", "2").unwrap();
        assert_eq!(buffer.as_slice(), b"12");
    }

    #[test]
    fn test_display() {
        let x = 123;

        let mut buffer: WriteBuffer<20> = WriteBuffer::new();
        write!(buffer, "{}", x).unwrap();
        assert_eq!("123", format!("{}", buffer));
    }

    #[test]
    fn test_as_str_mut() {
        let mut buffer: WriteBuffer<20> = WriteBuffer::new();
        write!(buffer, "hello world").unwrap();
        buffer.as_str_mut().make_ascii_uppercase();

        assert_eq!(buffer.as_str(), "HELLO WORLD");
    }

    #[test]
    fn test_is_empty_is_full_and_overflow() {
        let mut buffer: WriteBuffer<10> = WriteBuffer::new();
        assert!(buffer.is_empty());
        write!(buffer, "0123456789").unwrap();
        assert!(buffer.is_full());
        assert_eq!(write!(buffer, "!"), Err(core::fmt::Error));
    }
}
