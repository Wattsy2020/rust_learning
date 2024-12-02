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

// todo: encapsulate this in a HttpConnectionHandler
// that takes a function that maps a HttpRequest to HttpResponse
pub fn handle_connection(mut stream: TcpStream) {
    println!("Connection established!");

    let buf_reader = BufReader::new(&stream);
    let request_contents = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty());

    let request = HttpRequest::from_lines(request_contents);
    if let Ok(request) = request {
        println!("Request: {request:#?}");

        let response = HttpResponse {
            version: request.version(),
            status: HttpStatus::Ok200,
            content: "Hello World!".to_string(),
        };
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
    // todo: return bad request on parse error
}
