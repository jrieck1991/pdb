mod enclave;

fn main() {
    // seal
    let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    println!("label {:?}", label);
    let (cipher_text, seal_data) = enclave::seal_key(label);
    println!(
        "cipher_text: {:?}\n seal_data: {:?}",
        cipher_text, seal_data
    );

    // unseal
    match enclave::unseal_key(cipher_text, seal_data) {
        Ok(label) => println!("label {:?}", label),
        Err(e) => println!("unseal_key error: {:?}", e),
    }
}
