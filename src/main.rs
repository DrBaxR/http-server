use web_server::{
    http::{
        request::{parser::components::RequestMethod, Request},
        response::{Response, ResponseStatus},
    },
    Handler, HttpServer,
};

fn main() {
    let root_handler = Box::new(|_: &Request| {
        let payload = "This is the response".as_bytes();
        Response::build(
            ResponseStatus::Ok,
            &[("Content-Length", "0")],
            Some(Vec::from(payload)),
        )
    });

    let post_handler = Box::new(|req: &Request| {
        let payload = if let Some(body) = &req.body {
            body.data()
        } else {
            Vec::new()
        };

        Response::build(ResponseStatus::Ok, &[], Some(payload))
    });

    let handlers = vec![
        Handler::new(RequestMethod::Get, "/", root_handler),
        Handler::new(RequestMethod::Post, "/post", post_handler),
    ];

    let mut server = HttpServer::new(2020, handlers);
    server.start();
}
