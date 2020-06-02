use socket2::{Domain, SockAddr, Socket, Type};
use std::io::{Read, Write};

use std::os::unix::net::UnixStream;

pub mod serialize;

pub struct Unix {
    stream: UnixStream,
}

impl Unix {
    pub fn new(action: &str, path: &str) -> Unix {
        if action == "listen" {
            let s = listen(path);
            Unix { stream: s }
        } else {
            let s = connect(path);
            Unix { stream: s }
        }
    }
    pub fn write(&mut self, data: serialize::Data) {
        let encoded: Vec<u8> = serialize::serialize(data);
        self.stream.write_all(&encoded);
    }

    pub fn read(&mut self) -> serialize::Data {
        // init buffer of max data size
        let mut buf = [0; 10000];

        // receive data
        let num_bytes_read = self.stream.read(&mut buf[..]).unwrap();
        println!("number of bytes read: {}", num_bytes_read);

        // deserialize bytes
        let data: serialize::Data = serialize::deserialize(buf.to_vec());
        data
    }
}

fn connect(path: &str) -> UnixStream {
    // init unix domain socket
    let socket = Socket::new(Domain::unix(), Type::stream(), None).unwrap();

    // form peer address with given path
    let peer_addr = SockAddr::unix(path).unwrap();

    // connect to peer
    socket.connect(&peer_addr).unwrap();

    // convert to unix stream
    socket.into_unix_stream()
}

fn listen(path: &str) -> UnixStream {
    // init unix domain socket
    let socket = Socket::new(Domain::unix(), Type::stream(), None).unwrap();

    // form listen address
    let listen_addr = SockAddr::unix(path).unwrap();

    // bind to socket
    socket.bind(&listen_addr).unwrap();

    // start listening
    socket.listen(10).unwrap();

    // block waiting for connection
    let (conn, _addr) = socket.accept().unwrap();

    // convert to unix stream
    conn.into_unix_stream()
}
