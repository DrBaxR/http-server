use std::{collections::HashMap, fmt::Debug};

use json::JsonValue;

pub enum RequestMethod {
    Get,
    Put,
    Delete,
    Patch,
    Post,
    Unknown,
}

impl RequestMethod {
    pub fn parse_request_line(req_data: &String) -> (RequestMethod, String) {
        let split_data: Vec<_> = req_data.split(" ").collect();
        let (method, uri, _version) = (
            split_data.get(0).unwrap().to_string(),
            split_data.get(1).unwrap().to_string(),
            split_data.get(2).unwrap(),
        );

        let method = match method.as_str() {
            "GET" => RequestMethod::Get,
            "POST" => RequestMethod::Post,
            "PUT" => RequestMethod::Put,
            "PATCH" => RequestMethod::Patch,
            "DELETE" => RequestMethod::Delete,
            _ => RequestMethod::Unknown,
        };

        (method, uri)
    }
}

impl Debug for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "Get"),
            Self::Put => write!(f, "Put"),
            Self::Delete => write!(f, "Delete"),
            Self::Patch => write!(f, "Patch"),
            Self::Post => write!(f, "Post"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

pub enum RequestBody {
    TextPlain(String),
    ApplicationJson(JsonValue),
    Unknown(Vec<u8>),
}

impl RequestBody {
    pub fn from(content_type: &str, body_data: &Vec<u8>) -> RequestBody {
        match content_type {
            "text/plain" => {
                RequestBody::TextPlain(RequestBody::parse_text_plain(body_data))
            },
            "application/json" => {
                RequestBody::ApplicationJson(RequestBody::parse_application_json(body_data))
            }
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
}

pub fn parse_headers(headers_data: &Vec<String>) -> HashMap<String, String> {
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
