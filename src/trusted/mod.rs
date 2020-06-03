mod encrypt;
#[cfg_attr(target_vendor = "fortanix", path = "seal/mod.rs")]
#[cfg_attr(target_os = "macos", path = "seal/mock.rs")]
mod seal;
use crate::net;

pub struct Client {
    dal_addr: String,
}

impl Client {
    pub fn new(addr: &str) -> Client {
        Client {
            dal_addr: addr.to_string(),
        }
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        // connect to dal
        let mut stream = net::connect(&self.dal_addr);

        // TODO arbitrary label to apply with sealing key, store
        let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // get mrenclave sealing key and seal data
        let (seal_key, _seal_data) = seal::seal_key(label);

        // encrypt key
        // TODO: must be increments of 16 bytes
        let e_key = encrypt::encrypt(&seal_key, &mut key.to_vec());

        // encrypt value
        // TODO: must be increments of 16 bytes
        let e_value = encrypt::encrypt(&seal_key, &mut value.to_vec());

        // delete sealing key from memory
        drop(seal_key);

        // form request
        let req = net::serialize::Data {
            key: e_key,
            value: e_value,
            action: "put".to_string(),
        };

        // send to unix socket
        net::write(&mut stream, req);
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        // connect to dal
        let mut stream = net::connect(&self.dal_addr);

        // TODO arbitrary label to apply with sealing key, store
        let label: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // get mrenclave sealing key and seal data
        let (seal_key, seal_data) = seal::seal_key(label);

        // encrypt key
        // TODO: must be increments of 16 bytes
        let e_key = encrypt::encrypt(&seal_key, &mut key.to_vec());

        // delete sealing key from memory
        drop(seal_key);

        // form request
        let req = net::serialize::Data {
            key: e_key,
            value: [0; 1].to_vec(),
            action: "get".to_string(),
        };

        // send get key value pair request
        net::write(&mut stream, req);

        // read response
        let data = match net::read(&mut stream) {
            Some(data) => data,
            None => return None,
        };

        // get unseal key
        let unseal_key = seal::unseal_key(label, seal_data).unwrap();

        // use unseal key to decrypt
        let decrypted_value = encrypt::decrypt(&unseal_key, &mut data.value.to_vec());

        // delete unsealing key from memory
        drop(unseal_key);

        Some(decrypted_value)
    }
}
