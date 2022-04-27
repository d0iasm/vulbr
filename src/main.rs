mod default_page;
mod gui;
mod http;
mod net;
mod renderer;
mod url;

use crate::default_page::DEFAULT_PAGE;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use std::string::ToString;
//use crate::net::udp_response;
use crate::renderer::render;
use crate::url::ParsedUrl;

fn help_message() {
    println!("Usage: browser-rs.bin [ OPTIONS ]");
    println!("       -u, --url      URL. Default: http://127.0.0.1:8888/index.html");
    println!("       -h, --help     Show this help message.");
    println!("       -d, --default  Show a default page with embedded HTML content for test.");
    std::process::exit(0);
}

fn main() {
    let mut url = "http://127.0.0.1:8888/index.html";
    let mut default = false;

    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if "--help".to_string() == args[i] || "-h" == args[i] {
            help_message();
            return;
        }

        if "--url".to_string() == args[i] || "-u".to_string() == args[i] {
            if i + 1 >= args.len() {
                help_message();
            }
            url = &args[i + 1];
        }

        if "--default".to_string() == args[i] || "-d".to_string() == args[i] {
            default = true;
        }
    }

    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();
    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.show();
    });

    app.run();

    if default {
        render(DEFAULT_PAGE.to_string());
    } else {
        /*
        let parsed_url = ParsedUrl::new(url.to_string());
        println!("parsed_url: {:?}", parsed_url);

        println!("----- receiving a response -----");
        let response = udp_response(&parsed_url);
        println!("{}", response);

        render(response, &app);
        */
    }
}
