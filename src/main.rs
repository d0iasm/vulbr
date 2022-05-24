mod default_page;
mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::renderer::render;
use crate::url::ParsedUrl;

fn handle_url(url: String) -> i32 {
    println!("url: {}", url);
    42
}

fn main() {
    gui::start_browser_window(handle_url);
    /*
    let parsed_url = ParsedUrl::new(url.to_string());
    let response = udp_response(&parsed_url);
    render(response, &app);
    */
}
