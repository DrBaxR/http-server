use std::{collections::HashMap, fmt::Debug};

use self::{reader::data::RequestData, parser::components::{RequestType, RequestBody}};

pub mod parser;
pub mod reader;

pub struct Request {
    typ: RequestType,
    headers: HashMap<String, String>,
    body: Option<RequestBody>,
}

impl Request {
    pub fn new(
        typ: RequestType,
        headers: HashMap<String, String>,
        body: Option<RequestBody>,
    ) -> Request {
        Request { typ, headers, body }
    }

    pub fn from(data: RequestData) -> Request {
      parser::parse_req(&data)
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("typ", &self.typ)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}
