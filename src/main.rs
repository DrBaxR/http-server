use std::{net::{TcpListener, TcpStream}, io::Write};

use http::response::ResponseStatus;
use web_server::http::{self, request::Request, response::{Response}};

fn main() -> std::io::Result<()> {
    const PORT: &str = "2020";
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;
    println!("[MAIN] Listening on port {PORT}...");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let req_data = http::request::reader::read_req(&stream);
    let req = Request::from(req_data);

    println!("[MAIN] Received request: {:?} {}", req.method, req.path);

    let payload = "This is the response".as_bytes();
    let response_data = Response::build_data(ResponseStatus::Ok, &[("Content-Length", "0")], Some(Vec::from(payload)));

    stream.write_all(&response_data).unwrap();
}
