extern crate pdb;
use pdb::db;

// accept data from unix socket
// either put or get

fn main() {
    // init db client with storage dir and unix socket listen path
    let mut client = db::DB::new("/tmp/data", "/tmp/db.sock");

    // begin handling requests from clients
    client.handle()
}
