mod browser;
mod default_page;
mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::browser::Browser;
use crate::renderer::render;
use crate::url::ParsedUrl;

fn main() {
    let browser = Browser::new();
    browser.run();

    /*
    let parsed_url = ParsedUrl::new(url.to_string());
    let response = udp_response(&parsed_url);
    render(response, &app);
    */
}
