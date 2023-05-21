pub mod types;
pub mod server;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpStream}};
use crate::types::HttpRequestStatus;
use phf::{phf_set};
use crate::server::Server;


const HTTP_METHODS_LIST: phf::Set<&'static str> = phf_set! {
        "GET",
        "HEAD",
        "POST",
        "PUT",
        "DELETE",
        "CONNECT",
        "OPTIONS",
        "TRACE",
        "PATCH"
};

fn cb(str: String) {
    println!("{}", str);
}

fn main() {
    let app: Server = Server::new();
    app.listen(
        None, None, cb
    );
}