use std::collections::HashMap;

use self::components::RequestBody;

use super::{RequestData, Request, RequestMethod};

pub mod components;

pub fn parse_req(req_data: &RequestData) -> Request {
    let (req_data, headers_data, body_data) = match req_data {
        RequestData::WithoutBody(req, headers) => (req, headers, None),
        RequestData::WithBody(req, headers, body) => (req, headers, Some(body)),
    };

    let (method, path) = RequestMethod::parse_request_line(req_data);
    let headers = parse_headers(headers_data);

    if let Some(body_bytes) = body_data {
        let body = RequestBody::from(
            headers
                .get("content-type")
                .expect("No Content-Type header in request"),
            body_bytes,
        );

        return Request::new(method, path, headers, Some(body));
    }

    Request::new(method, path, headers, None)
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
