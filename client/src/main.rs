mod client;

fn main() {

    let key = String::from("jklsdoqanzkertvc");
    let value = String::from("owejdnbcxbtlkyfd");

    // init client
    let mut client = client::Client::new("localhost:9998");

    // put records
    client.put(&key, &value);

    // get record
    let value = client.get(&key);

    println!("value: {:?}", value);
}
