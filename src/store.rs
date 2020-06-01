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
    }

    pub fn get(&self, key: &str){
    }
}