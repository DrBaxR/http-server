use std::net::{TcpListener, TcpStream};

use web_server::http;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2020")?;
    println!("Listening on port 2020...");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let req_data = http::reader::read_req(stream);
    let req = http::parser::parse_req(&req_data);

    println!("{req:?}")
}
