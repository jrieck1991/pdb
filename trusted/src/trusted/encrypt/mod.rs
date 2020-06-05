use aes::block_cipher_trait::generic_array::GenericArray;
use aes::block_cipher_trait::BlockCipher;
use aes::Aes128;

// encrypt uses key to encrypt block with AES256
pub fn encrypt(key: &[u8; 16], data: &mut Vec<u8>) -> Vec<u8> {
    // create cipher
    let cipher = Aes128::new(GenericArray::from_slice(key));

    let mut result: Vec<u8> = Vec::new();

    for chunk in data.chunks_mut(16) {
        let mut block = GenericArray::clone_from_slice(chunk);

        // encrypt
        cipher.encrypt_block(&mut block);

        result.extend_from_slice(&block);
    }

    result
}

// decrypt uses key to decrypt block with AES256
pub fn decrypt(key: &[u8; 16], data: &mut Vec<u8>) -> Vec<u8> {
    // create cipher
    let cipher = Aes128::new(GenericArray::from_slice(key));

    let mut result: Vec<u8> = Vec::new();

    for mut chunk in data.chunks_mut(16) {
        let mut block = GenericArray::clone_from_slice(&mut chunk);

        // decrypt
        cipher.decrypt_block(&mut block);

        result.extend_from_slice(&block)
    }

    result
}
