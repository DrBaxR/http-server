use self::components::RequestBody;

use super::{RequestData, Request, RequestMethod};

pub mod components;

pub fn parse_req(req_data: &RequestData) -> Request {
    let (req_data, headers_data, body_data) = match req_data {
        RequestData::WithoutBody(req, headers) => (req, headers, None),
        RequestData::WithBody(req, headers, body) => (req, headers, Some(body)),
    };

    let (method, path) = RequestMethod::parse_request_line(req_data);
    let headers = components::parse_headers(headers_data);

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
