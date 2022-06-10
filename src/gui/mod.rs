mod browser_window;

use crate::renderer::html::dom::{ElementKind, NodeKind};
use crate::renderer::layout::render_tree::{RenderObject, RenderTree};
use browser_window::BrowserWindow;
use core::cell::RefCell;
use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, Box, DrawingArea, HeaderBar, Label, Orientation, SearchBar, SearchEntry,
    ToggleButton,
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
                let width = node.borrow().style.width();
                let height = node.borrow().style.height();
                let div = DrawingArea::builder()
                    .content_height(height as i32)
                    .content_width(width as i32)
                    .margin_start(node.borrow().style.margin_left() as i32)
                    .margin_top(node.borrow().style.margin_top() as i32)
                    .margin_end(node.borrow().style.margin_right() as i32)
                    .margin_bottom(node.borrow().style.margin_bottom() as i32)
                    .build();

                let bg_rgb = node.borrow().style.background_color();
                let padding_top = node.borrow().style.padding_top();
                let padding_right = node.borrow().style.padding_right();
                let padding_bottom = node.borrow().style.padding_bottom();
                let padding_left = node.borrow().style.padding_left();
                div.set_draw_func(move |_drawing_area, cairo_context, _w, _h| {
                    cairo_context.rectangle(
                        padding_left as f64,
                        padding_top as f64,
                        (width - padding_right) as f64,
                        (height - padding_bottom) as f64,
                    );
                    cairo_context.set_source_rgb(bg_rgb.r, bg_rgb.g, bg_rgb.b);
                    cairo_context.fill().expect("failed to fill out div");
                });
                /*
                for attr in &element.attributes {
                    if attr.name == "id" {
                        div.set_widget_name(&attr.value);
                    }
                    if attr.name == "class" {
                        div.add_css_class(&attr.value);
                    }
                }
                */
                content_area.append(&div);
            }
        },
        NodeKind::Text(text) => {
            let label = Label::builder().label(text).wrap(true).build();

            content_area.append(&label);
        }
    }
}

fn paint_dom(node: &Option<Rc<RefCell<RenderObject>>>, content_area: &Box) {
    match node {
        Some(n) => {
            //println!("{:?} {:?}", n.borrow().kind, n.borrow().style);
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
            }));

            window.show();
        }),
    );

    application.run();
}
