mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::url::ParsedUrl;

fn handle_url(url: String) -> String {
    println!("handle_url : {}", url);

    let parsed_url = ParsedUrl::new(url.to_string());
    println!("parsed_url : {:?}", parsed_url);

    format!("{:?}", parsed_url)
}

fn main() {
    gui::start_browser_window(handle_url);

    /*
    let response = udp_response(&parsed_url);
    render(response, &app);
    */
}
