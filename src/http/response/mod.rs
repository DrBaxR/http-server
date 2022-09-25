use std::collections::HashMap;

use self::status::ResponseStatus;

pub mod status;

pub struct Response {
    status: ResponseStatus,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(
        status: ResponseStatus,
        mut headers: HashMap<String, String>,
        body: Option<Vec<u8>>,
    ) -> Response {
        // set the content length in case response has a body
        if let Some(body) = &body {
            headers.insert(String::from("Content-Length"), body.len().to_string());
        }

        Response {
            status,
            headers,
            body,
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let headers = self
            .headers
            .iter()
            .map(|(key, val)| format!("{key}: {val}"))
            .fold(String::from(""), |acc, val| {
                format!("{}{}\r\n", acc.to_owned(), val.to_owned())
            });

        let status_line = format!("HTTP/1.1 {}", self.status.to_string());

        let res_head = format!("{status_line}\r\n{headers}\r\n");
        let mut res_bytes = Vec::from(res_head.as_bytes());
        if let Some(body) = &mut self.body {
            res_bytes.append(body);
        }

        res_bytes
    }
}
