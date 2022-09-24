use std::fmt::Debug;

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
