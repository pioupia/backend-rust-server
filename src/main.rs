pub mod server;
pub mod types;
pub mod thread_pool;

use crate::server::Server;


fn cb(str: String) {
    println!("The server is up on: http://{}", str);
}

fn main() {
    let mut app: Server = Server::new(None);
    app.listen(
        None, None, cb
    );
}