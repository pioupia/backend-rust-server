use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


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

        println!("Connection established !");

        // Process the new connection
        handle_request(stream);
    }
}

// Create a new function named 'handle_request' which take a mutable TcpStream argument.
fn handle_request(mut stream: TcpStream) {
    // We'll create a new Buffer React to read the content of the mut stream
    let buffer_reader = BufReader::new(&mut stream);

    // We'll create a new vector to collect theses lines of request
    let http_request: Vec<_> = buffer_reader
        .lines()
        // We iterate through the lines, we "define" a "res" variable, and unwrap it.
        // Same as before, the errors will stop the program, so its not very clean and for production
        .map(| res | res.unwrap())
        // The browser signals the end of an HTTP request by sending two newline characters in a row.
        // So, we iterate through the lines, and show when there is an empty line.
        .take_while(| res | !res.is_empty())
        // Then, we collect theses lines into a vector.
        .collect();

    // Print in the console the lines of the request.
    println!("Request: {:#?}", http_request);
}