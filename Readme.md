A no-std `Write`able buffer, to use like

 ```
use e_write_buffer::WriteBuffer;
use std::fmt::Write;
let mut buffer: WriteBuffer<20> = WriteBuffer::new();
let x = 12;
write!(buffer, "{}", x).expect("Can't write");
assert_eq!(buffer.as_str().unwrap(), "12");
```