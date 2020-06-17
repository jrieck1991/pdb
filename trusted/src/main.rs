mod trusted;

fn main() {
    // init client and connect to database
    let mut client = trusted::Client::new("localhost:9998", "localhost:9999");

    // handle incoming requests
    client.handle();
}
