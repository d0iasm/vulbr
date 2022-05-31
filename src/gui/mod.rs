mod browser_window;

use crate::renderer::html::dom::{ElementKind, NodeKind};
use crate::renderer::layout::render_tree::{RenderObject, RenderTree};
use browser_window::BrowserWindow;
use core::cell::RefCell;
use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, Box, HeaderBar, Label, Orientation, SearchBar, SearchEntry, ToggleButton,
};
use std::rc::Rc;

fn paint_dom_node(node: &Rc<RefCell<RenderObject>>, content_area: &Box) {
    match &node.borrow().kind {
        NodeKind::Document => {}
        NodeKind::Element(element) => match element.kind {
            ElementKind::Html
            | ElementKind::Head
            | ElementKind::Style
            | ElementKind::Script
            | ElementKind::Body => {}
            ElementKind::Link => {}
            ElementKind::Text => {}
            ElementKind::Ul => {}
            ElementKind::Li => {}
            ElementKind::Div => {
                println!(
                    "div !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!1 {:?}",
                    node.borrow().style.to_string()
                );
                let div = Box::builder()
                    .css_classes(node.borrow().style.to_string())
                    .build();
                content_area.append(&div);
            }
        },
        NodeKind::Text(text) => {
            let label = Label::builder()
                .label(text)
                .wrap(true)
                .vexpand(true)
                .halign(Align::Center)
                .valign(Align::Center)
                .build();

            content_area.append(&label);
        }
    }
}

fn paint_dom(node: &Option<Rc<RefCell<RenderObject>>>, content_area: &Box) {
    match node {
        Some(n) => {
            println!("{:?} {:?}", n.borrow().kind, n.borrow().style);
            paint_dom_node(n, &content_area);

            let child_content_area = Box::new(Orientation::Vertical, 0);
            content_area.append(&child_content_area);
            paint_dom(&n.borrow().first_child(), &child_content_area);
            paint_dom(&n.borrow().next_sibling(), content_area);
        }
        None => return,
    }
}

pub fn start_browser_window(handle_input: fn(String) -> RenderTree) {
    let application = Application::builder().application_id("vulbr").build();

    application.connect_activate(
        clone!(@strong application, @strong handle_input => move |app| {
            let window = BrowserWindow::new(app);
            window.set_default_size(1280, 800);
            window.set_title(Some("vulbr"));

            let header_bar = HeaderBar::new();
            window.set_titlebar(Some(&header_bar));

            let search_button = ToggleButton::new();
            search_button.set_icon_name("system-search-symbolic");
            search_button.set_active(true);
            header_bar.pack_end(&search_button);

            let container = Box::new(Orientation::Vertical, 6);
            window.set_child(Some(&container));

            let search_bar = SearchBar::builder()
                .valign(Align::Start)
                .key_capture_widget(&window)
                .build();

            container.append(&search_bar);

            search_button
                .bind_property("active", &search_bar, "search-mode-enabled")
                .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();

            let entry = SearchEntry::new();
            entry.set_hexpand(true);
            search_bar.set_child(Some(&entry));

            let label = Label::builder()
                .label("Welcome to VulBr!")
                .wrap(true)
                .vexpand(true)
                .halign(Align::Center)
                .valign(Align::Center)
                .build();

            container.append(&label);

            entry.connect_activate(clone!(@weak label, @weak window => move |entry| {
                container.remove(&label);

                let render_tree = handle_input(entry.text().to_string());
                paint_dom(&render_tree.root, &container);
                //label.set_label(&entry.text().to_string());
            }));

            window.show();
        }),
    );

    application.run();
}
