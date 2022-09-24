use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2020")?;
    println!("Listening on port 2020...");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    request_line = request_line.trim().to_string();

    let header_lines: Vec<_> = reader.lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // TODO: parse headers and read body
    // let mut body_bytes = Vec::with_capacity(capacity)

    println!("Req: {request_line}");
    println!("Headers: {header_lines:?}");
}
