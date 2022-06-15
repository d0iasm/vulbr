mod gui;
mod http;
mod renderer;
mod url;

use crate::http::HttpClient;
use crate::renderer::css::cssom::*;
use crate::renderer::css::token::*;
use crate::renderer::html::dom::*;
use crate::renderer::html::token::*;
use crate::renderer::js::ast::JsParser;
use crate::renderer::js::runtime::JsRuntime;
use crate::renderer::js::token::JsLexer;
use crate::renderer::layout::render_tree::*;
use crate::url::ParsedUrl;
use core::cell::RefCell;
//use gdk4::Display;
//use gtk4::{CssProvider, StyleContext};
use std::rc::Rc;
use std::string::String;

/// for debug
fn print_dom(node: &Option<Rc<RefCell<Node>>>, depth: usize) {
    match node {
        Some(n) => {
            print!("{}", "  ".repeat(depth));
            println!("{:?}", n.borrow().kind);
            print_dom(&n.borrow().first_child(), depth + 1);
            print_dom(&n.borrow().next_sibling(), depth);
        }
        None => return,
    }
}

/// for debug
fn print_render_object(node: &Option<Rc<RefCell<RenderObject>>>, depth: usize) {
    match node {
        Some(n) => {
            print!("{}", "  ".repeat(depth));
            println!("{:?} {:?}", n.borrow().kind, n.borrow().style);
            print_render_object(&n.borrow().first_child(), depth + 1);
            print_render_object(&n.borrow().next_sibling(), depth);
        }
        None => return,
    }
}

/*
// TODO: replace load_css with gtk4::render_background (?)
fn load_css(css: &[u8]) {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(css);

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
*/

fn handle_input(url: String) -> RenderTree {
    println!("handle_url: {}", url);

    // parse url
    let parsed_url = ParsedUrl::new(url.to_string());
    println!("parsed_url: {:?}", parsed_url);

    // send a HTTP request and get a response
    let client = HttpClient::new();
    let response = match client.get(&parsed_url) {
        Ok(res) => res,
        Err(e) => panic!("failed to get http response: {:?}", e),
    };

    println!("response: {:?}", response.body());

    // html
    let html = response.body();
    let html_tokenizer = HtmlTokenizer::new(html);
    println!("html tokenizer done");
    let dom_root = HtmlParser::new(html_tokenizer).construct_tree();
    println!("DOM:");
    print_dom(&Some(dom_root.clone()), 0);
    println!("----------------------");

    // css
    /*
    let style = get_style_content(dom_root.clone());
    //load_css(style.as_bytes());
    let css_tokenizer = CssTokenizer::new(style);
    let cssom = CssParser::new(css_tokenizer).parse_stylesheet();

    println!("CSSOM:\n{:?}", cssom);
    println!("----------------------");

    // js
    let js = get_js_content(dom_root.clone());
    let lexer = JsLexer::new(js);
    println!("JS lexer {:?}", lexer);

    let mut parser = JsParser::new(lexer);
    let ast = parser.parse_ast();
    println!("JS ast {:?}", ast);

    let mut runtime = JsRuntime::new();
    runtime.execute(&ast);
    */

    // apply css to html and create RenderTree
    let cssom = StyleSheet::new();
    let render_tree = RenderTree::new(dom_root, &cssom);

    println!("----------------------");
    println!("Render Tree:");
    print_render_object(&render_tree.root, 0);
    println!("----------------------");

    render_tree
}

fn main() {
    gui::start_browser_window(handle_input);
}
