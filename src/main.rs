mod default_page;
mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::renderer::render;
use crate::url::ParsedUrl;
use glib::closure_local;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button};

fn main() {
    //let browser = Browser::new();
    //browser.run();

    let _app = gui::start_browser_window();

    /*
    match app.active_window() {
        Some(w) => {
            w.connect_closure(
                "signal-test",
                false,
                closure_local!(|number: i32| {
                    println!("{}", number);
                }),
            );
        }
        None => {}
    }
    */

    /*
    let parsed_url = ParsedUrl::new(url.to_string());
    let response = udp_response(&parsed_url);
    render(response, &app);
    */
}
