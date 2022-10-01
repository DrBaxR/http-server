use std::{
    collections::HashMap,
    io::Write,
    net::{TcpListener, TcpStream},
};

use http::{
    request::{parser::components::RequestMethod, Request},
    response::Response,
};

use crate::http::response::ResponseStatus;

pub mod http;

pub struct HttpServer
{
    port: usize,
    stream: Option<TcpStream>,
    handlers: Vec<Handler>,
}

impl HttpServer
{
    pub fn new(port: usize, handlers: Vec<Handler>) -> Self {
        HttpServer {
            port,
            stream: None,
            handlers,
        }
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
        // TODO: refactor this
        let req = if let Some(stream) = &mut self.stream {
            let req_data = http::request::reader::read_req(&stream);
            let req = Request::from(req_data);

            println!("[SERVER] Received request: {:?} {}", req.method, req.path);

            Some(req)
        } else {
            None
        };

        let res = if let Some(req) = req {
            Some(self.handle_req(&req))
        } else {
            None
        };

        if let (Some(stream), Some(mut res)) = (&mut self.stream, res) {
            stream.write_all(&res.serialize()).unwrap();
        }
        self.stream = None;
    }

    fn handle_req(&self, req: &Request) -> Response {
        let handler = self.handlers.iter().find(|h| h.can_handle(req));

        if let Some(handler) = handler {
            handler.handle(req)
        } else {
            Response::new(ResponseStatus::NotFound, HashMap::new(), Some("No handler found for method".as_bytes().to_vec()))
        }
    }
}

pub struct Handler
{
    method: RequestMethod,
    path: &'static str,
    handle: Box<dyn Fn(&Request) -> Response>,
}

impl Handler
{
    pub fn new(method: RequestMethod, path: &'static str, handle: Box<dyn Fn(&Request) -> Response>) -> Self {
        Handler {
            method,
            path,
            handle,
        }
    }

    pub fn handle(&self, req: &Request) -> Response {
        (self.handle)(req)
    }

    pub fn can_handle(&self, req: &Request) -> bool {
        return self.method == req.method && self.path == req.path;
    }
}
