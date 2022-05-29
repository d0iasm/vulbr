mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::http::{HttpRequest, Method};
use crate::net::http;
use crate::renderer::render;
use crate::url::ParsedUrl;

fn handle_input(url: String) -> String {
    println!("handle_url : {}", url);

    let parsed_url = ParsedUrl::new(url.to_string());
    println!("parsed_url : {:?}", parsed_url);

    let request = HttpRequest::new(Method::Get, &parsed_url);
    let response = match http(request) {
        Ok(res) => res,
        Err(e) => panic!("failed to get http response: {:?}", e),
    };

    println!("response: {:?}", response.body());

    render(response.body());

    response.body()
}

fn main() {
    gui::start_browser_window(handle_input);
}
