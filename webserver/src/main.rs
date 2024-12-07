use std::fs;
use std::net::TcpListener;
use webserver::http::*;

fn hello_world_responder(request: HttpRequest) -> HttpResponse {
    if request.path() == "/" {
        HttpResponse {
            version: request.version(),
            status: HttpStatus::Ok200,
            content: fs::read_to_string("hello.html").unwrap(),
        }
    }
    else {
        HttpResponse {
            version: request.version(),
            status: HttpStatus::NotFound404,
            content: fs::read_to_string("not_found.html").unwrap()
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("The 7878 port should be free, otherwise the program cannot run");
    let mut server = Server::new(listener, hello_world_responder);
    server.serve();
}
