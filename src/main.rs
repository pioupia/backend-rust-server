pub mod server;
pub mod types;
pub mod thread_pool;

use crate::server::Server;


fn cb(str: String) {
    println!("{}", str);
}

fn main() {
    let app: Server = Server::new(None);
    app.listen(
        None, None, cb
    );
}