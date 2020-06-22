mod store;
use lib::net;

pub struct Client {
    tcp_server: net::Server,
    store: store::Store,
}

impl Client {
    pub fn new(listen_addr: &str, connect_addr: &str) -> Client {
        Client {
            tcp_server: net::Server::new(listen_addr),
            store: store::Store::new(connect_addr),
        }
    }

    pub fn handle(&mut self) {
        loop {
            // wait for new connection
            let mut stream = self.tcp_server.accept();
            println!("new conn");

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
                    self.store.put(&data.key, &data.value);
                }
                &"get" => {
                    println!("get request received");
                    match self.store.get(&data.key) {
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
}
