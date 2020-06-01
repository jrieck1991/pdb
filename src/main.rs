mod db;

fn main() {
    // init db client
    let client = db::DB::new("/tmp/storage_path");

    // TODO: will panic if key not in chunks of 16
    let key = String::from("jdhgywiqlakdlokj").into_bytes();

    // TODO: will panic if value not in chunks of 16
    let value = String::from("jdhgydjalakdloke").into_bytes();

    // store data
    client.put(&key, &value);

    // store encrypted data and seal_data
    match client.get(&key) {
        Some(val) => println!("value: {}", String::from_utf8(val).unwrap()),
        None => println!("no item found for key"),
    };
}
