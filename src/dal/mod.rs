mod store;
use crate::net;

pub struct DAL {
    listen_path: String,
    storage: store::Client,
}

impl DAL {
    pub fn new(storage_path: &str, listen_path: &str) -> DAL {
        DAL {
            listen_path: listen_path.to_string(),
            storage: store::Client::new(storage_path),
        }
    }

    pub fn handle(&mut self) {

        // start listening on a unix domain socket
        let listen_socket = net::listen(self.listen_path.as_str());

        loop {
            // accept connection and form new socket
            let mut stream = net::accept(&listen_socket);

            // receive data from unix stream
            let data: net::serialize::Data = match net::read(&mut stream) {
                Some(data) => data,
                None => {
                    println!("client closed connection");
                    return
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
                            net::write(&mut stream, req);
                        }
                        None => {
                            println!("no match found");
                        }
                    };
                },
                _ => {
                    println!("no action match");
                    //return
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
