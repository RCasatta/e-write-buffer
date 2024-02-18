use e_write_buffer::WriteBuffer;
use std::fmt::Write as _;

fn main() {
    let mut buffer: WriteBuffer<20> = WriteBuffer::new();
    let x = 12;
    write!(buffer, "{}", x).unwrap();
    assert_eq!(buffer.as_str(), "12");
}
