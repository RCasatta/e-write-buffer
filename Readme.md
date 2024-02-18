[![MIT license](https://img.shields.io/github/license/RCasatta/e-write-buffer)](https://github.com/RCasatta/e-write-buffer/blob/master/LICENSE)
[![Crates](https://img.shields.io/crates/v/e-write-buffer.svg)](https://crates.io/crates/e-write-buffer)
[![Released API docs](https://docs.rs/e-write-buffer/badge.svg)](https://docs.rs/e-write-buffer)

A `no_std`, no allocation, `core::fmt::Write`able buffer.

Usage:

```rs
use e_write_buffer::WriteBuffer;
use std::fmt::Write as _;

fn main() {
    let mut buffer: WriteBuffer<20> = WriteBuffer::new();
    let x = 12;
    write!(buffer, "{}", x).unwrap();
    assert_eq!(buffer.as_str(), "12");
}
```