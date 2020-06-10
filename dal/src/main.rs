mod dal;

// accept data from unix socket
// either put or get

fn main() {
    // init db client with storage dir and unix socket listen path
    let mut client = dal::DAL::new("data", "localhost:9999");

    // begin handling requests from clients
    client.handle()
}
