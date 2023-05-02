pub mod types;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use crate::types::HttpRequestStatus;
use phf::{phf_set};


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

fn main() {
    // Bind the local port 8000, then, precise "unwrap" to close the server when
    // the program got an error.
    // TODO: manage the errors to avoid the webserver closing with errors
    let listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(num) => num,
        Err(_) => {
            println!("Address already in used.");
            std::process::exit(1);
        }
    };

    // Iterate through the listener.incoming() stream iterator.
    for stream in listener.incoming() {
        // Get the stream without errors
        // Or close the program.
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("Connection established !");

        // Process the new connection, and pass a reference to the stream
        handle_request(&stream);
    }
}

// Create a new function named 'handle_request' which take a mutable TcpStream argument.
fn handle_request(stream: &TcpStream) {
    // We'll create a new Buffer React to read the content of the mut stream
    let buffer_reader = BufReader::new(stream);

    // We'll create a new vector to collect theses lines of request
    let http_request: Vec<_> = buffer_reader
        .lines()
        // We iterate through the lines, we "define" a "res" variable, and unwrap it.
        // Same as before, the errors will stop the program, so its not very clean and for production
        .map(|res| match res {
            Ok(s) => s,
            Err(e) => {
                println!("An error has occurred during a request: {}", e);

                send_response(stream, &String::from("HTTP/1.1 500 Internal Server Error"));

                panic!("An error has occurred during the request");
            }
        })
        // The browser signals the end of an HTTP request by sending two newline characters in a row.
        // So, we iterate through the lines, and show when there is an empty line.
        .take_while(|res| !res.is_empty())
        // Then, we collect theses lines into a vector.
        .collect();

    let mut http_request_iterator = http_request.iter();

    // Take the first request line
    let first_request_line = match http_request_iterator.next() {
        Some(t) => t,
        None => {
            println!("An error has occurred during the parse of the request");

            send_response(stream, &String::from("HTTP/1.1 500 Internal Server Error"));

            panic!("An error has occurred during the parsing of the request");
        }
    };

    // Parsing the status line to get the informations about it.
    let http_request_content = parse_status_line(first_request_line);

    // Create a variable for the status line
    let status_line = format!("HTTP/{} 200 OK", http_request_content.http_version);

    // Read the content of the index file to string
    let content;

    if http_request_content.path == "/" {
        // Return dynamically the index page
        content = match fs::read_to_string("./src/pages/index.html") {
            Ok(s) => s,
            Err(e) => {
                println!("An error has occurred when searching for the correct file: {}", e);

                send_response(stream, &String::from("HTTP/1.1 404 Not Found"));

                panic!("An error has occurred during the search of the file");
            }
        };
    } else {
        // Return dynamically the HTML pages
        content = match fs::read_to_string(
            format!("./src/pages/{}.html", http_request_content.path)
        ) {
            Ok(s) => s,
            Err(e) => {
                println!("An error has occurred when searching for the correct file: {}", e);

                send_response(stream, &String::from("HTTP/1.1 404 Not Found"));

                panic!("An error has occurred during the search of the file");
            }
        };
    }

    // Take its len
    let content_len = content.len();

    // Create the response by formatting the string.
    let response =
        format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{content}");

    // Return the response as byte slice, and unwrap it to avoid errors
    send_response(stream, &response);

    // Print in the console the lines of the request.
    println!("Request: {:#?}", http_request);
    // Print in the console the response for this request.
    println!("Response: {:#?}", response);
}

fn send_response(mut stream: &TcpStream, response: &String) {
    match stream.write_all(response.as_bytes()) {
        Ok(s) => s,
        Err(e) => {
            println!("An error has occurred when sending a response:\nError: {0}\nResponse: {1}", e, response);
        }
    }
}

fn parse_status_line(status_line: &String) -> HttpRequestStatus {
    // Split the status line into piece of text without spaces.
    let status_line_parts: Vec<_> = status_line.split_whitespace().collect();

    // If the status line has more than 3 spaces, we cant accept the request (sorry)
    if status_line_parts.len() != 3 {
        panic!("The HTTP Request status is invalid")
    }

    // If the first argument (the method) is not valid, we deny the request
    if !HTTP_METHODS_LIST.contains(status_line_parts[0]) {
        panic!("The method is not correct.")
    }

    // If the first char of the second argument is not a '/', we know that the path is not valid.
    if !status_line_parts[1].chars().nth(0).eq(&Option::from('/')) {
        panic!("The path is invalid.")
    }

    // We know that the HTTP version should take 8 chars
    // 'HTTP/x.x' like HTTP/2.0
    // If not, the HTTP version is invalid
    if status_line_parts[2].len() != 8 {
        panic!("The HTTP version is invalid.");
    }

    // We separate the http_version from the HTTP string (by slicing the string)
    // We're removing the "HTTP/" part.
    let http_version = &status_line_parts[2][5..8];

    // If the HTTP version is not 1.1 or 1.2, we know that we can't do anything for that (sorry).
    // So, the version is not supported or doesn't exist.
    if http_version != "1.1" && http_version != "1.2" {
        panic!("The HTTP version is invalid/not supported.")
    }

    // TODO: separate the path and the query parameters
    let http_request_content = HttpRequestStatus {
        method: status_line_parts[0].to_string(),
        http_version: http_version.parse().expect("The HTTP version should be a float32."),
        path: status_line_parts[1].to_string(),
    };

    println!("{{ method: {0}, http_version: {1}, path: {2} }}",
             http_request_content.method, http_request_content.http_version, http_request_content.path);

    return http_request_content;
}