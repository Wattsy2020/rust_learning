use std::net::TcpListener;
use webserver::http::*;

fn hello_world_responder(request: HttpRequest) -> HttpResponse {
    HttpResponse {
        version: request.version(),
        status: HttpStatus::Ok200,
        content: "Hello World!".to_string(),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("The 7878 port should be free, otherwise the program cannot run");
    let mut server = Server::new(listener, hello_world_responder);
    server.serve();
}
