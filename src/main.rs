use web_server::{
    http::{
        request::parser::components::RequestMethod,
        response::{Response, ResponseStatus},
    },
    Handler, HttpServer,
};

fn main() {
    let handlers = vec![Handler::new(RequestMethod::Get, "/", |_| {
        let payload = "This is the response".as_bytes();
        Response::build(
            ResponseStatus::Ok,
            &[("Content-Length", "0")],
            Some(Vec::from(payload)),
        )
    })];

    let mut server = HttpServer::new(2020, handlers);
    server.start();
}
