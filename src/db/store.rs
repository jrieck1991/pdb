use rocksdb::{Options, DB};

pub struct Client {
    storage: DB,
}

impl Client {
    pub fn new(path: &str) -> Client {
        let db = DB::open_default(path).unwrap();

        Client { storage: db }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        self.storage.put(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.storage.get(key) {
            Ok(opt_value) => match opt_value {
                Some(value) => Some(value.to_vec()),
                None => None,
            },
            Err(_e) => None,
        }
    }
}
