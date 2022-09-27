use std::{collections::HashMap};

use self::{reader::data::RequestData, parser::components::{RequestMethod, RequestBody}};

pub mod parser;
pub mod reader;

pub struct Request {
    pub method: RequestMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<RequestBody>,
}

impl Request {
    pub fn new(
        method: RequestMethod,
        path: String,
        headers: HashMap<String, String>,
        body: Option<RequestBody>,
    ) -> Request {
        Request { method, path, headers, body }
    }

    pub fn from(data: RequestData) -> Request {
      parser::parse_req(&data)
    }
}
