use std::net::TcpListener;
use std::time::Duration;
use std::{fs, thread};
use webserver::http::*;

fn hello_world_responder(request: HttpRequest) -> HttpResponse {
    match request.path().as_str() {
        "/" => HttpResponse {
            version: request.version(),
            status: HttpStatus::Ok200,
            content: fs::read_to_string("hello.html").unwrap(),
        },
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            HttpResponse {
                version: request.version(),
                status: HttpStatus::Ok200,
                content: fs::read_to_string("hello.html").unwrap(),
            }
        }
        _ => HttpResponse {
            version: request.version(),
            status: HttpStatus::NotFound404,
            content: fs::read_to_string("not_found.html").unwrap(),
        },
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("The 7878 port should be free, otherwise the program cannot run");
    let server = Server::new(listener, hello_world_responder);
    server.serve();
}
