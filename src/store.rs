use rocksdb::{DB, Options};

pub struct Client {
    db: DB,
}

impl Client {

    pub fn new() -> Client {
        let path = "/tmp/store";
        let d = DB::open_default(path).unwrap();

        Client {
            db: d,
        }
    }

    pub fn put(&self, key: &str, value: &str) {
        self.db.put(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.db.get(key) {
            Ok(opt_value) => match opt_value {
                Some(value) => Some(String::from_utf8(value).unwrap()),
                None => None,
            } 
            Err(e) => None,
        }
    }
}