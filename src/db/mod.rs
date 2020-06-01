#[cfg_attr(target_vendor = "fortanix", path = "enclave.rs")]
#[cfg_attr(target_os = "macos", path = "enclave_mock.rs")]
mod enclave;
mod encrypt;
mod store;

pub struct DB {
    storage: store::Client,
}

impl DB {
    pub fn new(storage_path: &str) -> DB {
        DB {
            storage: store::Client::new(storage_path),
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        // TODO arbitrary label to apply with sealing key, store
        let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // get mrenclave sealing key and seal data
        let (seal_key, _seal_data) = enclave::seal_key(label);

        // encrypt key
        // TODO: must be increments of 16 bytes
        let e_key = encrypt::encrypt(&seal_key, &mut key.to_vec());

        // encrypt value
        // TODO: must be increments of 16 bytes
        let e_value = encrypt::encrypt(&seal_key, &mut value.to_vec());

        // delete sealing key from memory
        drop(seal_key);

        // put into storage
        self.storage.put(&e_key, &e_value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // TODO arbitrary label to apply with sealing key, store
        let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // get mrenclave sealing key and seal data
        let (seal_key, seal_data) = enclave::seal_key(label);

        // encrypt key
        // TODO: must be increments of 16 bytes
        let e_key = encrypt::encrypt(&seal_key, &mut key.to_vec());

        // delete sealing key from memory
        drop(seal_key);

        // get encrypted key
        let value = match self.storage.get(&e_key) {
            Some(value) => value,
            None => return None,
        };

        // get unseal key
        let unseal_key = enclave::unseal_key(label, seal_data).unwrap();

        // use unseal key to decrypt
        let decrypted_value = encrypt::decrypt(&unseal_key, &mut value.to_vec());

        // delete unsealing key from memory
        drop(unseal_key);

        Some(decrypted_value)
    }
}
