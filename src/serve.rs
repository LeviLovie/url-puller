use tiny_http::Response;
use std::fs::File;
use log::info;

pub fn serve(request: tiny_http::Request) {
    let address = request.remote_addr();
    match request.url() {
        "/" => {
            info!("Served index.html to {:?}", address);
            request.respond(Response::from_file(File::open("static/index.html").unwrap())).unwrap();
        }
        "/style.css" => {
            info!("Served style.css to {:?}", address);
            request.respond(Response::from_file(File::open("static/style.css").unwrap())).unwrap();
        }
        _ => {
            info!("Served 404.html to {:?}", address);
            request.respond(Response::from_file(File::open("static/404.html").unwrap())).unwrap();
        }
    }
}
