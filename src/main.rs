use std::{net::{TcpListener, TcpStream}, collections::HashMap, io::Write};

use web_server::http::{self, request::Request, response::Response};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2020")?;
    println!("Listening on port 2020...");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let req_data = http::request::reader::read_req(&stream);
    let req = Request::from(req_data);

    // todo: cleanup
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Length"), String::from("0"));
    let mut res = Response::new(http::response::status::ResponseStatus::Ok, headers, None);
    let ser = res.serialize();

    println!("{req:?}\n");
    stream.write_all(&ser).unwrap();
}
