mod browser_window;

use browser_window::BrowserWindow;
use glib::clone;
use glib::closure_local;
use gtk4::glib::value::ToValue;
use gtk4::prelude::*;
use gtk4::{gio, glib};
use gtk4::{
    Align, Application, Box, HeaderBar, Label, Orientation, SearchBar, SearchEntry, ToggleButton,
};

pub fn start_browser_window(handle_url: fn(String) -> i32) {
    let application = Application::builder().application_id("vulbr").build();

    application.connect_startup(
        clone!(@strong application, @strong handle_url => move |app| {
            let window = build_ui(app);
            window.connect_closure(
                "signal-test",
                false,
                closure_local!(move |_w: BrowserWindow, _number: i32| {
                    println!("get a signal !!!!!!!!!!! {}", handle_url("test".to_string()));
                }),
            );
        }),
    );
    application.run();
}

fn build_ui(app: &Application) -> BrowserWindow {
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
        .label("Type to start search")
        .vexpand(true)
        .halign(Align::Center)
        .valign(Align::Center)
        .css_classes(vec!["large-title".to_string()])
        .build();

    container.append(&label);

    entry.connect_activate(clone!(@weak label, @weak window => move |entry| {
        println!("connect_activate");
        window.set_property("url", entry.text());
        let n = 42.to_value();
        window.emit_by_name::<()>("signal-test", &[&n]);
    }));

    window.show();

    window
}
