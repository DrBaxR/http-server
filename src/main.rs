use std::{net::{TcpListener, TcpStream}};

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
    let req_data = http::read_req(stream);

    println!("{req_data:?}")
}
