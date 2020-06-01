use rocksdb::{DB, Options};

pub struct Client {
    db: DB,
}

impl Client {

    pub fn new(path: &str) -> Client {
        let d = DB::open_default(path).unwrap();

        Client {
            db: d,
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        self.db.put(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.db.get(key) {
            Ok(opt_value) => match opt_value {
                Some(value) => Some(value.to_vec()),
                None => None,
            } 
            Err(_e) => None,
        }
    }
}
