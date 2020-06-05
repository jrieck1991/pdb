mod trusted;

// encrypt data received from TLS
// send either get or put request with data to unix socket

fn main() {
    // init client and connect to database
    let mut client = trusted::Client::new("db.sock");

    // TODO: will panic if key not in chunks of 16
    let key = String::from("jdhgywiqlakdlokj").into_bytes();

    // TODO: will panic if value not in chunks of 16
    let value = String::from("jdhgydjalakdloke").into_bytes();

    // store data
    println!("put value");
    client.put(&key, &value);

    // get value
    println!("get value");
    match client.get(&key) {
        Some(value) => println!("value: {:?}", value),
        None => println!("no match found for key: {:?}", key),
    };
}
