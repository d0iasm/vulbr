mod default_page;
mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::renderer::render;
use crate::url::ParsedUrl;

fn main() {
    //let browser = Browser::new();
    //browser.run();

    gui::init_browser_window();

    /*
    let parsed_url = ParsedUrl::new(url.to_string());
    let response = udp_response(&parsed_url);
    render(response, &app);
    */
}
