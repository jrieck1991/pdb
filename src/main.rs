#[cfg_attr(target_vendor = "fortanix", path = "enclave.rs")]
#[cfg_attr(target_os = "macos", path = "enclave_mock.rs" )]
mod enclave;
mod encrypt;
mod store;

fn main() {

    // init db
    let db_client = store::Client::new("/tmp/store");

    // arbitrary label to apply with sealing key, store
    let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    // get mrenclave sealing key and seal data
    let (seal_key, seal_data) = enclave::seal_key(label);

    // TODO: will panic if plaintext ln is not a multiple of 16 bytes
    let mut plain_text = String::from("jdhgywiqlakdlokj").into_bytes();

    // encrypt data
    let mut encrypted = encrypt::encrypt(&seal_key, &mut plain_text);

    // delete key from memory to prevent leaks
    drop(seal_key);

    // store encrypted data and seal_data
    db_client.put(&encrypted, "top_secret_stuff".as_bytes());

    // get unseal key
    let unseal_key = enclave::unseal_key(label, seal_data).unwrap();

    // use unseal key to decrypt
    let plain_res = encrypt::decrypt(&unseal_key, &mut encrypted);

    println!("result: {}", String::from_utf8(plain_res).unwrap());
}
