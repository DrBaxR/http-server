use std::fmt::Debug;

use json::JsonValue;

#[derive(Eq, PartialEq, Debug)]
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

pub enum RequestBody {
    TextPlain(String),
    ApplicationJson(JsonValue),
    Unknown(Vec<u8>),
}

impl RequestBody {
    pub fn from(content_type: &str, body_data: &Vec<u8>) -> RequestBody {
        match content_type {
            "text/plain" => RequestBody::TextPlain(RequestBody::parse_text_plain(body_data)),
            "application/json" => {
                RequestBody::ApplicationJson(RequestBody::parse_application_json(body_data))
            }
            _ => RequestBody::Unknown(body_data.to_owned()),
        }
    }

    pub fn data(&self) -> Vec<u8> {
        match self {
            Self::TextPlain(string) => string.as_bytes().to_vec(),
            Self::ApplicationJson(json) => json.to_string().as_bytes().to_vec(),
            Self::Unknown(data) => data.clone()
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
