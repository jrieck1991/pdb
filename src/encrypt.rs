use aes::Aes256;
use aes::block_cipher_trait::BlockCipher;
use aes::block_cipher_trait::generic_array::GenericArray;

// encrypt uses key to encrypt block with AES256
pub fn encrypt(key: [u8; 16], mut block: [u8; 16]) {

    // create cipher
    let cipher = Aes256::new(GenericArray::from_slice(&key));

    let block = GenericArray::from_mut_slice(&mut block);

    // encrypt
    cipher.encrypt_block(block);
}

// decrypt uses key to decrypt block with AES256
pub fn decrypt(key: [u8; 16], mut block: [u8; 16]) {

    // create cipher
    let cipher = Aes256::new(GenericArray::from_slice(&key));

    let block = GenericArray::from_mut_slice(&mut block);

    // encrypt
    cipher.decrypt_block(block);
}
