pub mod http {
  use std::{net::TcpStream, io::{BufReader, BufRead}};

  pub fn read_req(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    // read the first line
    let request_line = read_request_line(&mut reader);

    // read headers
    let header_lines = read_header_lines(&mut reader);
    
    // read body (if it exists)
    read_body_bytes(&mut stream, &header_lines);
  }

  fn read_request_line(reader: &mut BufReader<&TcpStream>) -> String {
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    request_line.trim().to_string()
  }

  fn read_header_lines(reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    reader.lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
  }

  fn read_body_bytes(stream: &mut TcpStream, header_lines: &Vec<String>) {
    let content_length_line = header_lines
      .iter()
      .find(|line| line.trim().to_lowercase().contains("content-length:"));
    println!("{content_length_line:?}")
  }
}
