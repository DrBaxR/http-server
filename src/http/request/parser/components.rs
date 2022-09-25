use std::{collections::HashMap, fmt::Debug};

use json::JsonValue;

pub enum RequestType {
    Get(String),
    Put(String),
    Delete(String),
    Patch(String),
    Post(String),
    Unknown,
}

impl RequestType {
    pub fn from(req_data: &String) -> RequestType {
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
}

impl Debug for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(arg0) => f.debug_tuple("Get").field(arg0).finish(),
            Self::Put(arg0) => f.debug_tuple("Put").field(arg0).finish(),
            Self::Delete(arg0) => f.debug_tuple("Delete").field(arg0).finish(),
            Self::Patch(arg0) => f.debug_tuple("Patch").field(arg0).finish(),
            Self::Post(arg0) => f.debug_tuple("Post").field(arg0).finish(),
            Self::Unknown => f.debug_tuple("Unknown").finish(),
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

impl Debug for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextPlain(arg0) => f.debug_tuple("TextPlain").field(arg0).finish(),
            Self::ApplicationJson(arg0) => f.debug_tuple("ApplicationJson").field(arg0).finish(),
            Self::Unknown(arg0) => f.debug_tuple("Unknown").field(arg0).finish(),
        }
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
