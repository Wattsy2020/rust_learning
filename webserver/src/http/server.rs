use crate::http::{HttpRequest, HttpResponse, HttpStatus, HttpVersion};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// Handles all connections to a TcpListener and sends responses based on the response_fn
pub struct Server<F: FnMut(HttpRequest) -> HttpResponse> {
    listener: TcpListener,
    response_fn: F,
}

impl<F: FnMut(HttpRequest) -> HttpResponse> Server<F> {
    pub fn new(listener: TcpListener, response_fn: F) -> Self {
        Server {
            listener,
            response_fn,
        }
    }

    /// Start listening and responding to messages
    pub fn serve(&mut self) {
        // Each stream is a connection between the client and the server
        // In TCP, for each request received from client we need to send a response, then close the connection
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => Server::handle_connection(stream, &mut self.response_fn),
                Err(err) => println!("Failed to read connection, received error: {}", err.kind()),
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, response_fn: &mut F) {
        println!("Connection established!");

        let buf_reader = BufReader::new(&stream);
        let request_contents = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty());

        match HttpRequest::from_lines(request_contents) {
            Ok(request) => {
                println!("Request: {request:#?}");
                let response = response_fn(request);
                stream.write_all(response.to_string().as_bytes()).unwrap();
            }
            Err(err) => {
                let error_message = format!("Invalid request: {err:?}");
                let response = HttpResponse {
                    version: HttpVersion::Http1_1,
                    status: HttpStatus::BadRequest400,
                    content: error_message
                };
                stream.write_all(response.to_string().as_bytes()).unwrap();
            }
        }
    }
}
