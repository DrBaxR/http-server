use web_server::HttpServer;

fn main() {
    let mut server = HttpServer::new(2020);
    server.start();
}
