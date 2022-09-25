use std::{collections::HashMap, fmt::Debug};

use json::JsonValue;

pub enum RequestData {
    WithoutBody(String, Vec<String>),
    WithBody(String, Vec<String>, Vec<u8>),
}

impl Debug for RequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WithoutBody(arg0, arg1) => f
                .debug_tuple("WithoutBody")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WithBody(arg0, arg1, arg2) => f
                .debug_tuple("WithBody")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
        }
    }
}

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

pub enum RequestType {
    Get(String),
    Put(String),
    Delete(String),
    Patch(String),
    Post(String),
    Unknown,
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

impl Debug for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextPlain(arg0) => f.debug_tuple("TextPlain").field(arg0).finish(),
            Self::ApplicationJson(arg0) => f.debug_tuple("ApplicationJson").field(arg0).finish(),
            Self::Unknown(arg0) => f.debug_tuple("Unknown").field(arg0).finish(),
        }
    }
}
