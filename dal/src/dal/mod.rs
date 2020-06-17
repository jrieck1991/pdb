mod store;
use lib::net;

pub struct DAL {
    storage: store::Client,
    tcp_server: net::Server,
}

impl DAL {
    pub fn new(storage_path: &str, listen_addr: &str) -> DAL {
        DAL {
            storage: store::Client::new(storage_path),
            tcp_server: net::Server::new(listen_addr),
        }
    }

    pub fn handle(&mut self) {
        loop {
            // accept new connection and create stream
            let mut stream = self.tcp_server.accept();

            // receive data from stream
            let data: net::serialize::Data = match self.tcp_server.io.read(&mut stream) {
                Some(data) => data,
                None => {
                    println!("client closed connection");
                    return;
                }
            };

            // match action
            match &data.action.as_str() {
                &"put" => {
                    println!("put request received");
                    self.put(&data.key, &data.value);
                }
                &"get" => {
                    println!("get request received");
                    match self.get(&data.key) {
                        Some(value) => {
                            // form data to send result back to client
                            let req = net::serialize::Data {
                                action: "get".to_string(),
                                key: data.key,
                                value: value,
                            };

                            // write request to stream
                            self.tcp_server.io.write(&mut stream, req);
                        }
                        None => {
                            println!("no match found");
                        }
                    };
                }
                _ => {
                    println!("no action match");
                }
            }
        }
    }

    fn put(&self, key: &[u8], value: &[u8]) {
        self.storage.put(&key, &value);
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.storage.get(&key) {
            Some(value) => Some(value),
            None => None,
        }
    }
}
