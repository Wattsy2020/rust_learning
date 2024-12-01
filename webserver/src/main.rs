use std::net::TcpListener;
use webserver::http::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("The 7878 port should be free, otherwise the program cannot run");

    // Each stream is a connection between the client and the server
    // In TCP, for each request received from client we need to send a response, then close the connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(err) => println!("Failed to read connection, received error: {}", err.kind()),
        }
    }
}
