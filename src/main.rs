pub mod server;
pub mod types;

use phf::{phf_set};
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