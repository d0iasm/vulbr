mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::http::{HttpRequest, Method};
use crate::net::http;
use crate::url::ParsedUrl;

fn handle_url(url: String) -> String {
    println!("handle_url : {}", url);

    let parsed_url = ParsedUrl::new(url.to_string());
    println!("parsed_url : {:?}", parsed_url);

    let request = HttpRequest::new(Method::Get, &parsed_url);
    let response = match http(request) {
        Ok(res) => res,
        Err(e) => panic!("failed to get http response: {:?}", e),
    };

    format!("{:?}", response)
}

fn main() {
    gui::start_browser_window(handle_url);

    /*
    let response = udp_response(&parsed_url);
    render(response, &app);
    */
}
