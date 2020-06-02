mod store;
use crate::socket;

pub struct DB {
    stream: socket::Unix,
    storage: store::Client,
}

impl DB {
    pub fn new(storage_path: &str, listen_path: &str) -> DB {
        DB {
            stream: socket::Unix::new("listen", listen_path),
            storage: store::Client::new(storage_path),
        }
    }

    pub fn handle(&mut self) {
        loop {
            // receive data from unix stream
            let data: socket::serialize::Data = self.stream.read();

            // match action
            match &data.action.as_str() {
                &"put" => self.put(&data.key, &data.value),
                &"get" => {
                    let value = match self.get(&data.key) {
                        Some(value) => {
                            // form data to send result back to client
                            let req = socket::serialize::Data {
                                action: "get".to_string(),
                                key: data.key,
                                value: value,
                            };

                            // send to stream
                            self.stream.write(req)
                        }
                        None => {
                            println!("no match found");
                        }
                    };
                }
                _ => println!("no action: {}", data.action),
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
