use lib::net;

fn main() {

    let key = String::from("jklsdoqanzkertvc");
    let value = String::from("owejdnbcxbtlkyfd");

    // init client and connect to db
    let client = net::Client::new("localhost:9998");
    let mut stream = client.connect();

    // form put
    let put_req = net::serialize::Data {
        key: key.into_bytes(),
        value: value.into_bytes(),
        action: "put".to_string(),
    };

    // write request
    client.io.write(&mut stream, put_req);
}
