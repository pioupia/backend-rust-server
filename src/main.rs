use std::net::TcpListener;


fn main() {
    // Bind the local port 8000, then, precise "unwrap" to close the server when
    // the program got an error.
    // TODO: manage the errors to avoid the webserver closing with errors
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // Iterate through the listener.incoming() stream iterator.
    for stream in listener.incoming() {
        // Get the stream without errors
        // Or close the program.
        let stream = stream.unwrap();

        println!("Connection established !")
    }
}
