mod method;
mod request;
mod response;
mod status;
mod version;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub use method::HttpMethod;
pub use request::HttpRequest;
pub use response::HttpResponse;
pub use status::HttpStatus;
pub use version::HttpVersion;

// todo: convert this to use the structs
// todo: encapsulate this in a HttpConnectionHandler
// that takes a function that maps a HttpRequest to HttpResponse
pub fn handle_connection(mut stream: TcpStream) {
    println!("Connection established!");

    let buf_reader = BufReader::new(&stream);
    let request_contents: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {request_contents:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
