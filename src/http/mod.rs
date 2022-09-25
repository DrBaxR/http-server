use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use self::models::{RequestData, RequestType, RequestBody};

pub mod models;

pub fn read_req(stream: TcpStream) -> RequestData {
    let mut reader = BufReader::new(&stream);

    let request_line = read_request_line(&mut reader);
    let header_lines = read_header_lines(&mut reader);
    let body = read_body_bytes(&mut reader, &header_lines);

    if let Some(body_bytes) = body {
        RequestData::WithBody(request_line, header_lines, body_bytes)
    } else {
        RequestData::WithoutBody(request_line, header_lines)
    }
}

fn read_request_line(reader: &mut BufReader<&TcpStream>) -> String {
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    request_line.trim().to_string()
}

fn read_header_lines(reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

fn read_body_bytes(
    reader: &mut BufReader<&TcpStream>,
    header_lines: &Vec<String>,
) -> Option<Vec<u8>> {
    let content_length_line = header_lines
        .iter()
        .find(|line| line.trim().to_lowercase().contains("content-length:"));

    if let Some(length_line) = content_length_line {
        let length_vec: Vec<_> = length_line
            .split(":")
            .skip(1)
            .map(|val| val.trim())
            .collect();
        let length: usize = length_vec[0]
            .parse()
            .expect("Request 'content-length' header not formatted correctly");

        Some(reader.bytes().take(length).map(|b| b.unwrap()).collect())
    } else {
        None
    }
}

pub fn parse_req(req_data: &RequestData) {
    let (req_data, headers_data, body_data) = match req_data {
        RequestData::WithoutBody(req, headers) => (req, headers, None),
        RequestData::WithBody(req, headers, body) => (req, headers, Some(body)),
    };

    let req = parse_req_type(req_data);
    let headers = parse_headers(headers_data);

    if let Some(body_bytes) = body_data {
        let body = parse_body(
            headers
                .get("content-type")
                .expect("No Content-Type header in request"),
            body_bytes,
        );

        println!("{body:?}")
    }

    println!("{req:?}");
    println!("{headers:?}");
}

fn parse_req_type(req_data: &String) -> RequestType {
    let split_data: Vec<_> = req_data.split(" ").collect();
    let (method, uri, _version) = (
        split_data.get(0).unwrap().to_string(),
        split_data.get(1).unwrap().to_string(),
        split_data.get(2).unwrap(),
    );

    match method.as_str() {
        "GET" => RequestType::Get(uri),
        "POST" => RequestType::Post(uri),
        "PUT" => RequestType::Put(uri),
        "PATCH" => RequestType::Patch(uri),
        "DELETE" => RequestType::Delete(uri),
        _ => RequestType::Unknown,
    }
}

fn parse_headers(headers_data: &Vec<String>) -> HashMap<String, String> {
    let mut headers_map = HashMap::new();
    headers_data
        .iter()
        .map(|line| {
            let split: Vec<_> = line
                .split(":")
                .into_iter()
                .map(|s| s.trim().to_lowercase())
                .collect();

            (
                split.get(0).unwrap().to_owned(),
                split.get(1).unwrap().to_owned(),
            )
        })
        .for_each(|(key, value)| {
            headers_map.insert(key, value);
        });

    headers_map
}

fn parse_body(content_type: &str, body_data: &Vec<u8>) -> RequestBody {
    match content_type {
        "text/plain" => RequestBody::TextPlain(parse_text_plain(body_data)),
        _ => RequestBody::Unknown
    }
}

fn parse_text_plain(data: &Vec<u8>) -> String {
    String::from_utf8(data.to_owned()).unwrap()
}
