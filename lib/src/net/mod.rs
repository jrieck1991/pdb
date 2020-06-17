use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub mod serialize;

pub struct Server {
    listener: TcpListener,
    pub io: IO,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        // bind to addr
        let l = TcpListener::bind(&addr).unwrap();

        Server {
            listener: l,
            io: IO::new(),
        }
    }

    pub fn accept(&self) -> TcpStream {
        let (stream, _addr) = self.listener.accept().unwrap();
        stream
    }
}

pub struct Client {
    connect_addr: String,
    pub io: IO,
}

impl Client {
    pub fn new(addr: &str) -> Client {
        Client {
            connect_addr: addr.to_string(),
            io: IO::new(),
        }
    }

    pub fn connect(&self) -> TcpStream {
        TcpStream::connect(self.connect_addr.as_str()).unwrap()
    }
}

pub struct IO {}

impl IO {
    pub fn new() -> IO {
        IO {}
    }

    pub fn read(&self, stream: &mut TcpStream) -> Option<serialize::Data> {
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

    pub fn write(&self, stream: &mut TcpStream, data: serialize::Data) {
        let encoded: Vec<u8> = serialize::serialize(data);
        stream.write_all(&encoded).unwrap();
    }
}
