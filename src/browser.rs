use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, HeaderBar, Label, Orientation, SearchBar,
    SearchEntry, ToggleButton,
};

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_default_size(1280, 800);
    window.set_title(Some("My Browser"));

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

pub struct Browser {
    app: Application,
    url: Option<String>,
}

impl Browser {
    pub fn new() -> Self {
        Self {
            app: Application::builder().application_id("my.browser").build(),
            url: None,
        }
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn run(&self) {
        self.app.connect_activate(build_ui);

        self.app.run();
    }
}
