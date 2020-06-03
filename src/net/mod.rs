use socket2::{Domain, SockAddr, Socket, Type};
use std::io::{Read, Write};

use std::os::unix::net::UnixStream;

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
    // init unix domain socket
    let socket = Socket::new(Domain::unix(), Type::stream(), None).unwrap();

    // form peer address with given path
    let peer_addr = SockAddr::unix(path).unwrap();

    // connect to peer
    socket.connect(&peer_addr).unwrap();

    // convert to unix stream
    socket.into_unix_stream()
}

pub fn listen(path: &str) -> Socket {
    // init unix domain socket
    let socket = Socket::new(Domain::unix(), Type::stream(), None).unwrap();

    // form listen address
    let listen_addr = SockAddr::unix(path).unwrap();

    // bind to socket
    socket.bind(&listen_addr).unwrap();

    // start listening
    socket.listen(10).unwrap();

    socket
}

pub fn accept(socket: &Socket) -> UnixStream {
    // block waiting for connection
    let (conn, _addr) = socket.accept().unwrap();

    // convert to unix stream
    conn.into_unix_stream()
}
