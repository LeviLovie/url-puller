use log::{LevelFilter, info};
use tiny_http::Server;
use std::thread;

mod serve;
mod logic;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_utc_timestamps()
        .init().unwrap();
    let server = Server::http("192.168.1.124:7878").unwrap();
    info!("Starting server on http://192.168.1.124:7878/");

    for request in server.incoming_requests() {
        thread::spawn(move || {
            match request.method() {
                &tiny_http::Method::Get => serve::serve(request),
                &tiny_http::Method::Post => logic::handle(request),
                _ => {}
            }
        });
    }
}
