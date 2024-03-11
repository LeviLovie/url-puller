use tiny_http::{HeaderField, Response};
use serde_json::{Result, Value};
use std::str;
use log::info;

fn get_img_urls(html: &str) -> Vec<String> {
    let mut urls: Vec<String> = vec![];
    let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
    let nodes = dom.nodes();
    for node in nodes.iter() {
        match node.as_tag() {
            Some(tag) => {
                if tag.name() == "img".as_bytes() {
                    let attrs = tag.attributes();
                    if attrs.get("src").is_some() {
                        urls.push(str::from_utf8(attrs.get("src").unwrap().unwrap().as_bytes()).unwrap().to_string());
                    }
                }
            }
            None => {}
        }
    }

    return urls;
}

fn get_url_site(url: &str) -> (bool, String) {
    let response = reqwest::blocking::get(url);
    match response {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                return (true, response.text().unwrap());
            } else {
                return (false, "".to_string());
            }
        }
        Err(_) => {
            return (false, "".to_string());
        }
    }
}

pub fn handle(request: tiny_http::Request) {
    let mut request = request;
    info!("Handling POST request from {:?}", request.remote_addr());
    // Check for header Content-Type: application/json
    if request.headers().iter().any(|h| h.field.eq(&HeaderField::from_bytes(b"Content-Type").unwrap()) && h.value.eq("application/json")) {
        let mut data = String::new();
        request.as_reader().read_to_string(&mut data).unwrap();
        let v: Result<Value> = serde_json::from_str(&data);
        if let Ok(v) = v {
            if v["url"].is_string() && v["url"].as_str().unwrap().starts_with("http") && !v["url"].is_null() {
                let url = v["url"].as_str().unwrap();
                let website = get_url_site(url);
                if website.0 {
                    let json = serde_json::to_string(&get_img_urls(&website.1)).unwrap();
                    request.respond(Response::from_string(json)).unwrap();
                    return;
                } else {
                    request.respond(Response::from_string(format!("There is nothing on the {} url.\n", url))).unwrap();
                    return;
                }
            } else {
                request.respond(Response::from_string("The url is not valid.\n")).unwrap();
                return;
            }
        } else {
            request.respond(Response::from_string("Unable to parse json of the request body.\n")).unwrap();
            return;
        }
    } else {
        request.respond(Response::from_string("Content-Type is not application/json.\n")).unwrap();
        return;
    }
}
