use std::io::{Read, Write};

use std::os::unix::net::{UnixStream, UnixListener};

pub mod serialize;

pub fn write(stream: &mut UnixStream, data: serialize::Data) {
    let encoded: Vec<u8> = serialize::serialize(data);
    stream.write_all(&encoded).unwrap();
}

pub fn read(stream: &mut UnixStream) -> Option<serialize::Data> {
    // init buffer of max data size
    let mut buf = [0; 1024];

    // receive data
    let num_bytes_read = stream.read(&mut buf[..]).unwrap();
    println!("number of bytes read: {}", num_bytes_read);

    // detect closed connection
    if num_bytes_read == 0 {
        return None;
    };

    // deserialize bytes
    let data: serialize::Data = serialize::deserialize(buf.to_vec());

    Some(data)
}

pub fn connect(path: &str) -> UnixStream {
    UnixStream::connect(path).unwrap()
}

pub fn listen(path: &str) -> UnixListener {
    UnixListener::bind(path).unwrap()
}

pub fn accept(listener: &UnixListener) -> UnixStream {
    // block waiting for connection
    let (stream, _addr) = listener.accept().unwrap();
    stream
}
