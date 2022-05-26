mod browser_window;

use browser_window::BrowserWindow;
use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, Box, HeaderBar, Label, Orientation, SearchBar, SearchEntry, ToggleButton,
};

pub fn start_browser_window(handle_input: fn(String) -> String) {
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
                .vexpand(true)
                .halign(Align::Center)
                .valign(Align::Center)
                .css_classes(vec!["large-title".to_string()])
                .build();

            container.append(&label);

            entry.connect_activate(clone!(@weak label, @weak window => move |entry| {
                println!("connect_activate");
                let result = handle_input(entry.text().to_string());
                label.set_label(&result.split_at(100).0);
            }));

            window.show();
        }),
    );

    application.run();
}
