use std::collections::HashMap;

use json::JsonValue;

use super::models::{Request, RequestBody, RequestData, RequestType};

pub fn parse_req(req_data: &RequestData) -> Request {
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

        return Request::new(req, headers, Some(body));
    }

    Request::new(req, headers, None)
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
        "application/json" => RequestBody::ApplicationJson(parse_application_json(body_data)),
        _ => RequestBody::Unknown(body_data.to_owned()),
    }
}

fn parse_text_plain(data: &Vec<u8>) -> String {
    String::from_utf8(data.to_owned()).unwrap()
}

fn parse_application_json(data: &Vec<u8>) -> JsonValue {
    let text = String::from_utf8(data.to_owned()).unwrap();

    json::parse(&text).unwrap()
}
