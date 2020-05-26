mod enclave;
mod encrypt;

fn main() {
    // get mrenclave sealing key
    let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let (seal_key, _seal_data) = enclave::seal_key(label);

    // TODO: will panic if plaintext ln is not a multiple of 16 bytes
    let mut plain_text = String::from("jdhgywiqlakdlokj").into_bytes();

    // encrypt data
    let mut _encrypted = encrypt::encrypt(&seal_key, &mut plain_text);

    // delete key from memory to prevent leaks
    // save data

    // use same key to decrypt
    //let mut plain = encrypt::decrypt(&key, &mut encrypted);

    // unseal
}
