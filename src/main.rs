use log::{error, info};
use simple_logger::SimpleLogger;
use std::env;
use tiny_http::{Response, Server};

fn main() {
    SimpleLogger::new().init().unwrap();

    const ROBOTS: &str = concat!("# └[∵┌]└[ ∵ ]┘[┐∵]┘", "\n\nUser-agent: *", "\nDisallow: /");

    let address = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());

    let http_server = Server::http(format!("{}:{}", address, port)).unwrap_or_else(|e| {
        error!("could not create tiny_http server: {}", e);
        panic!();
    });

    for request in http_server.incoming_requests() {
        info!(
            "incoming {:?} request from {:?} to {:?} with {:?}",
            request.method(),
            request.remote_addr(),
            request.url(),
            request.headers()
        );
        request
            .respond(Response::from_string(ROBOTS))
            .unwrap_or_else(|e| error!("failed to respond: {}", e));
    }
}
