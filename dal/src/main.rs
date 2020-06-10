mod dal;

fn main() {
    // init db client with storage dir and listen addr
    let mut client = dal::DAL::new("data", "localhost:9999");

    // begin handling requests from clients
    client.handle()
}
