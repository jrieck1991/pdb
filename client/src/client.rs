use lib::net;
use std::net::TcpStream;
use std::str;

pub struct Client {
    client: net::Client,
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Client {
        // init client and connect to db
        let tcp_client = net::Client::new(addr);
        let tcp_stream = tcp_client.connect();

        Client {
            client: tcp_client,
            stream: tcp_stream,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        println!("calling get");

        // form get
        let req = net::serialize::Data {
            key: String::from(key).into_bytes(),
            value: String::from("value").into_bytes(),
            action: "get".to_string(),
        };

        // write request
        self.client.io.write(&mut self.stream, req);

        // read response
        match self.client.io.read(&mut self.stream) {
            Some(data) => Some(String::from(str::from_utf8(&data.value).unwrap())),
            None => None,
        }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        println!("calling put");

        // form put
        let req = net::serialize::Data {
            key: String::from(key).into_bytes(),
            value: String::from(value).into_bytes(),
            action: "put".to_string(),
        };

        // write request
        self.client.io.write(&mut self.stream, req);
    }
}
