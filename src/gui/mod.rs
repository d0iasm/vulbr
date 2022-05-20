mod browser_window;

use browser_window::Window;
use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, Box, HeaderBar, Label, Orientation, SearchBar, SearchEntry, ToggleButton,
};

pub fn init_browser_window() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
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

    entry.connect_search_started(clone!(@weak search_button => move |_| {
        search_button.set_active(true);
    }));

    entry.connect_stop_search(clone!(@weak search_button => move |_| {
        search_button.set_active(false);
    }));

    entry.connect_search_changed(clone!(@weak label => move |entry| {
        if entry.text() != "" {
            label.set_text(&entry.text());
        } else {
            label.set_text("Type to start search");
        }
    }));

    window.show();
}

/*
fn build_ui(app: &Application) {
    // Create a window
    let window = Window::new(app);

    // ANCHOR: button
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // ANCHOR_END: button

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
*/
