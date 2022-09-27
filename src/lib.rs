use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use http::{request::Request, response::Response};

use crate::http::response::ResponseStatus;

pub mod http;

pub struct HttpServer {
    port: usize,
    stream: Option<TcpStream>,
}

impl HttpServer {
    pub fn new(port: usize) -> Self {
        HttpServer { port, stream: None }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        println!("[SERVER] Listening on port {}...", self.port);

        for stream in listener.incoming() {
            self.stream = Some(stream.unwrap());
            self.handle_connection();
        }
    }

    fn handle_connection(&mut self) {
        if let Some(stream) = &mut self.stream {
            let req_data = http::request::reader::read_req(&stream);
            let req = Request::from(req_data);

            println!("[SERVER] Received request: {:?} {}", req.method, req.path);

            // TODO: system where you add request handlers to the server, and based on the method and type, this picks what handler to use
            // !
            let payload = "This is the response".as_bytes();
            let response_data = Response::build_data(
                ResponseStatus::Ok,
                &[("Content-Length", "0")],
                Some(Vec::from(payload)),
            );
            // !

            stream.write_all(&response_data).unwrap();
        }

        self.stream = None;
    }
}
